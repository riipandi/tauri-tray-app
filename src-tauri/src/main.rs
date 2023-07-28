// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};

mod command;
mod tray;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Make the dock NOT to have an active app when started
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            // Listen for update messages
            let win = app.get_window("main").unwrap();
            win.listen("tauri://update-status".to_string(), move |msg| {
                println!("New status: {:?}", msg);
            });
            Ok(())
        })
        .system_tray(tray::register())
        .on_system_tray_event(tray::tray_event)
        .on_window_event(|event| match event.event() {
            WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
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
