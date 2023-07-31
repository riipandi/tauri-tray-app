// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use tauri::{AppHandle, WindowBuilder, WindowEvent, WindowUrl};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

use crate::{command, config, menu, meta, tray, utils};

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

pub fn initialize() {
    let mut builder = tauri::Builder::default();

    // register tauri plugins
    builder = builder
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .with_colors(ColoredLevelConfig::default())
                .level_for("tauri", log::LevelFilter::Info)
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        // .plugin(tauri_plugin_os::init())
        // .plugin(tauri_plugin_window::init())
        // .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .plugin(tauri_plugin_positioner::init());

    // setup and create window
    builder = builder.setup(|app| {
        // Set activation policy to `Accessory` to prevent
        // the app icon from showing on the dock.
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Regular);

        // Create main window for the application.
        create_window(&app.handle(), meta::MAIN_WINDOW, "index.html");

        Ok(())
    });

    // setup window menu
    builder = builder
        .enable_macos_default_menu(false)
        .menu(menu::build_menu())
        .on_menu_event(menu::handle_menu_event);

    // configure tray menu
    builder = builder
        .system_tray(tray::create_tray_menu())
        .on_system_tray_event(tray::handle_tray_event)
        .on_window_event(|e| {
            match e.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    // don't kill the app when the user clicks close. this is important!
                    e.window().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        });

    // run the application
    builder
        .invoke_handler(tauri::generate_handler![command::greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

const JS_INIT_SCRIPT: &str = r#"
    (function() {
        console.log('init script: not yet implemented');
    })();
"#;

fn create_window(app: &AppHandle, label: &str, url: &str) {
    let app_config = config::AppConfig::load();
    let window_url = WindowUrl::App(PathBuf::from(url));
    let mut wb = WindowBuilder::new(app, label, window_url);

    wb = wb
        .initialization_script(JS_INIT_SCRIPT)
        .user_agent(meta::USER_AGENT)
        .min_inner_size(520.0, 680.0)
        .inner_size(940.0, 720.0)
        .resizable(true)
        .enable_clipboard_access()
        .accept_first_mouse(true);

    #[cfg(target_os = "macos")]
    let window = wb
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .build()
        .expect("error while creating window");

    #[cfg(not(target_os = "macos"))]
    let window = wb.build().expect("error while creating window");

    // zoom webview
    utils::zoom_webview(&window, app_config.zoom_factor);

    window
        .set_title(meta::APP_TITLE)
        .expect("error while setting window title");

    window.listen("tauri://update-status".to_string(), move |msg| {
        println!("New status: {:?}", msg);
    });
}
