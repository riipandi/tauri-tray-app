// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use tauri::{App, Manager, Runtime};
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use tauri_tray_app::core::{cmd, state};
use tauri_tray_app::meta;

static DB: Lazy<native_db::DatabaseBuilder> = Lazy::new(|| {
    let mut builder = native_db::DatabaseBuilder::new();
    builder.define::<state::Settings>().expect("failed to define model");
    builder
});

#[tokio::main]
async fn main() {
    // Initialize Tauri context and builder
    let tauri_ctx = tauri::generate_context!();
    let builder = tauri::Builder::default();

    // Logger plugin should be called as early in the execution of the app as possible.
    let builder = builder.plugin(logger().build());

    // Register Tauri plugins
    let builder = builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init());

    // Setup Tauri application builder
    let builder = builder.setup(move |app| {
        setup_global_state(app);

        // Setup application menu and tray icon
        #[cfg(all(desktop, not(test)))]
        {
            tauri_tray_app::core::tray::init(app)?;
        }

        setup_main_window(app)?;

        Ok(())
    });

    // Configure window event handlers
    let builder = builder.on_window_event(|window, event| match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            if window.label() == meta::MAIN_WINDOW {
                // window.app_handle().runtime_handle.set_activation_policy(tauri::ActivationPolicy::Accessory);
                window.hide().expect("failed to hide window");
                api.prevent_close();
            }
        }
        _ => {}
    });

    // Build Tauri application
    let mut main_app = builder
        .invoke_handler(tauri::generate_handler![
            cmd::open_settings_window,
            cmd::open_with_shell,
            cmd::toggle_devtools,
            cmd::greet,
            state::load_settings,
            state::save_setting,
        ])
        .build(tauri_ctx)
        .expect("error while running tauri application");

    // Set activation policy to `Accessory` to prevent the app icon from showing on the dock.
    #[cfg(target_os = "macos")]
    main_app.set_activation_policy(tauri::ActivationPolicy::Regular);

    // Finally, run the application
    main_app.run(|_app, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            log::debug!("Exit requested");
            api.prevent_exit();
        }
        _ => {}
    });
}

fn logger() -> tauri_plugin_log::Builder {
    use tauri_plugin_log::fern::colors::ColoredLevelConfig;
    use tauri_plugin_log::WEBVIEW_TARGET;
    use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};

    let mut log_plugin_builder = tauri_plugin_log::Builder::new()
        .level_for("tauri", log::LevelFilter::Error)
        .level_for("hyper", log::LevelFilter::Off)
        .level_for("tao", log::LevelFilter::Off)
        .level_for("reqwest::connect", log::LevelFilter::Off)
        .timezone_strategy(TimezoneStrategy::UseUtc)
        .with_colors(ColoredLevelConfig::default());

    #[cfg(debug_assertions)]
    let log_filename = format!("tauri-tray-app-debug.log");

    #[cfg(not(debug_assertions))]
    let log_filename = format!("tauri-tray-app.log");

    let target_stdout = Target::new(TargetKind::Stdout);
    let target_logdir = Target::new(TargetKind::LogDir {
        file_name: Some(log_filename),
    });
    let target_webview = Target::new(TargetKind::Webview).filter(|metadata| metadata.target() == WEBVIEW_TARGET);

    #[cfg(debug_assertions)]
    let log_level = log::LevelFilter::Debug;

    #[cfg(not(debug_assertions))]
    let log_level = log::LevelFilter::Info;

    log_plugin_builder = log_plugin_builder
        .targets([target_stdout, target_logdir, target_webview])
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
        .level(log_level);

    log_plugin_builder
}

fn setup_global_state<R: Runtime>(app: &App<R>) {
    use tauri::path::BaseDirectory;

    log::debug!("Setting up global state");

    let db_file_path = app
        .path()
        .resolve(meta::APP_DB_FILENAME, BaseDirectory::AppConfig)
        .expect("failed to get db file path");

    // Create directory if it doesn't exist
    let config_dir = db_file_path.parent().expect("failed to get config directory");

    // Create directory if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir).expect("failed to create config directory");
    }

    #[cfg(debug_assertions)]
    log::debug!("Config path: {}", db_file_path.display());

    // Create with a file path to persist the database
    let db = DB.create(db_file_path).expect("failed to create database");

    // You can migrate the database here, that can be time consuming.
    log::debug!("Running app config migration");
    let tx = db.rw_transaction().expect("failed to create transaction");

    tx.migrate::<state::Settings>().expect("failed to migrate");
    tx.commit().expect("failed to commit migration");

    log::debug!("App config migration succeed");

    // Add the database to the application state
    app.handle().manage(db);
}

fn setup_main_window<R: Runtime>(app: &App<R>) -> tauri::Result<WebviewWindow<R>> {
    let mut wb = WebviewWindowBuilder::new(app, meta::MAIN_WINDOW, WebviewUrl::default());

    #[cfg(all(desktop, not(test)))]
    {
        use tauri_tray_app::core::utils;

        let app_title = &app.package_info().name;
        let user_agent = utils::get_app_user_agent(app.handle());

        wb = wb
            .title(app_title)
            .user_agent(&user_agent)
            .min_inner_size(640., 480.)
            .accept_first_mouse(true)
            .content_protected(true)
            .enable_clipboard_access()
            .resizable(true)
            .focused(true);
    }

    #[cfg(target_os = "macos")]
    {
        let app_menu = tauri_tray_app::core::menu::init(app.app_handle())?;

        wb = wb
            .shadow(true)
            .decorations(true)
            .transparent(false)
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true)
            .menu(app_menu);
    }

    #[cfg(target_os = "windows")]
    {
        wb = wb.decorations(true).transparent(true);
    }

    // Finally, build the webview
    wb.build()
}
