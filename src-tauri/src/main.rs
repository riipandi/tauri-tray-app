// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::shell::open, Manager};
use tauri::{SystemTray, SystemTrayEvent};

mod command;
mod menu;

fn main() {
    let app_tray_menu = SystemTray::new().with_menu(menu::tray_menu());
    let app = tauri::Builder::default();

    app.setup(|app| {
        let win = app.get_window("main").unwrap();
        // Listen for update messages
        win.listen("tauri://update-status".to_string(), move |msg| {
            println!("New status: {:?}", msg);
        });
        Ok(())
    })
    .system_tray(app_tray_menu)
    .on_system_tray_event(|app, event| match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            // let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "on_twitter" => {
                    open(&app.shell_scope(), "https://twitter.com/riipandi", None).ok();
                }
                "send_feedback" => {
                    open(
                        &app.shell_scope(),
                        "https://ripandis.com/feedback?product=tauri-tray-app",
                        None,
                    )
                    .ok();
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    })
    .on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    })
    .invoke_handler(tauri::generate_handler![command::greet, command::open_link])
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
