// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use log::info;
use std::path::PathBuf;
use tauri::api::dialog;
use tauri::http::{Request, Response};
use tauri::{AppHandle, Manager, RunEvent, WindowBuilder, WindowEvent, WindowUrl};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

use crate::{command, config, menu, meta, tray, utils};

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];

#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Error;

pub fn initialize() {
    let mut builder = tauri::Builder::default();
    let _app_config = config::AppConfig::load();

    // register tauri plugins
    builder = builder
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .with_colors(ColoredLevelConfig::default())
                .level_for("tauri", log::LevelFilter::Info)
                .level(LOG_LEVEL)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        // .plugin(tauri_plugin_window::init())
        // .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .plugin(tauri_plugin_positioner::init());

    // setup and create window
    builder = builder.setup(|app| {
        let handle = app.handle();

        // Set activation policy to `Accessory` to prevent
        // the app icon from showing on the dock.
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Regular);

        // Create main window for the application.
        create_window(&handle, meta::MAIN_WINDOW, "index.html");

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
                    // don't kill the app when the user clicks close.
                    if e.window().label() == meta::MAIN_WINDOW {
                        e.window().hide().unwrap();
                        api.prevent_close();
                    }
                }
                _ => {}
            }
        });

    // the deeplink url will be: myapp://x-callback
    builder = builder.register_uri_scheme_protocol("x-callback", callback);

    // run the application
    builder
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::open_devtools,
            command::set_darkmode,
            command::check_update,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }

            // if app_config.enable_auto_update {}
            // RunEvent::Updater(updater_event) => {
            //     match updater_event {
            //         tauri::UpdaterEvent::UpdateAvailable {
            //             body,
            //             date,
            //             version,
            //         } => {
            //             info!("update available {} {:?} {}", body, date, version);
            //         }
            //         // Emitted when the download is about to be started.
            //         tauri::UpdaterEvent::Pending => info!("update is pending!"),
            //         tauri::UpdaterEvent::DownloadProgress {
            //             chunk_length,
            //             content_length,
            //         } => {
            //             info!("downloaded {} of {:?}", chunk_length, content_length);
            //         }
            //         // Emitted when the download has finished and the update is about to be installed.
            //         tauri::UpdaterEvent::Downloaded => info!("update has been downloaded!"),
            //         // Emitted when the update was installed. You can then ask to restart the app.
            //         tauri::UpdaterEvent::Updated => info!("app has been updated"),
            //         // Emitted when the app already has the latest version installed
            //         // and an update is not needed.
            //         tauri::UpdaterEvent::AlreadyUpToDate => info!("app is already up to date"),
            //         // Emitted when there is an error with the updater. We suggest
            //         // to listen to this event even if the default dialog is enabled.
            //         tauri::UpdaterEvent::Error(error) => info!("failed to update: {}", error),
            //     }
            // }
            _ => {}
        });
}

const JS_INIT_SCRIPT: &str = r#"
    (function() {
        console.info('init script: not yet implemented');
    })();
"#;

fn create_window(app: &AppHandle, label: &str, url: &str) {
    let app_config = config::AppConfig::load();
    let window_url = WindowUrl::App(PathBuf::from(url));
    let mut wb = WindowBuilder::new(app, label, window_url);

    wb = wb
        .initialization_script(JS_INIT_SCRIPT)
        .user_agent(meta::USER_AGENT)
        .min_inner_size(620.0, 680.0)
        .inner_size(940.0, 720.0)
        .resizable(true)
        .enable_clipboard_access()
        .accept_first_mouse(true)
        .focused(true);

    if app_config.enable_darkmode {
        wb = wb.theme(Some(tauri::Theme::Dark))
    } else {
        wb = wb.theme(Some(tauri::Theme::Light))
    }

    #[cfg(target_os = "macos")]
    let window = wb
        .tabbing_identifier(meta::APP_NAME)
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
        info!("New status: {:?}", msg);
    });
}

fn callback(app: &AppHandle, req: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    info!("Callback URI: {:?}", req.uri());
    let window = app.get_window(meta::MAIN_WINDOW).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
    dialog::message(Some(&window), "Welcome Back", "Not yet implemented!");
    todo!()
}
