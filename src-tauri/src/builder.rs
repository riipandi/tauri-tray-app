// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::api::dialog;
use tauri::http::{Request, Response};
use tauri::{AppHandle, Manager, RunEvent, WindowEvent};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};
use tauri_plugin_store::StoreBuilder;

use crate::config::AppConfig;
use crate::{command, menu, meta, tray, utils};

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
    let _app_config = AppConfig::load();

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
        // Set activation policy to `Accessory` to prevent
        // the app icon from showing on the dock.
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Regular);

        let config_dir = app.handle().path_resolver().app_config_dir().unwrap();
        let config_path = config_dir.join("settings.json");
        let store = StoreBuilder::new(app.handle(), config_path).build();

        println!("STORE: {:?}", store.has("ui_config"));

        // Create main window for the application.
        utils::create_window(&app.handle(), meta::MAIN_WINDOW, "index.html");

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
            //             log::info!("update available {} {:?} {}", body, date, version);
            //         }
            //         // Emitted when the download is about to be started.
            //         tauri::UpdaterEvent::Pending => log::info!("update is pending!"),
            //         tauri::UpdaterEvent::DownloadProgress {
            //             chunk_length,
            //             content_length,
            //         } => {
            //             log::info!("downloaded {} of {:?}", chunk_length, content_length);
            //         }
            //         // Emitted when the download has finished and the update is about to be installed.
            //         tauri::UpdaterEvent::Downloaded => log::info!("update has been downloaded!"),
            //         // Emitted when the update was installed. You can then ask to restart the app.
            //         tauri::UpdaterEvent::Updated => log::info!("app has been updated"),
            //         // Emitted when the app already has the latest version installed
            //         // and an update is not needed.
            //         tauri::UpdaterEvent::AlreadyUpToDate => log::info!("app is already up to date"),
            //         // Emitted when there is an error with the updater. We suggest
            //         // to listen to this event even if the default dialog is enabled.
            //         tauri::UpdaterEvent::Error(error) => log::info!("failed to update: {}", error),
            //     }
            // }
            _ => {}
        });
}

fn callback(app: &AppHandle, req: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    log::info!("Callback URI: {:?}", req.uri());
    let window = app.get_window(meta::MAIN_WINDOW).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
    dialog::message(Some(&window), "Welcome Back", "Not yet implemented!");
    todo!()
}
