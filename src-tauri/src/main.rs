// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::shell::open, async_runtime::block_on};
use tauri::{AppHandle, Manager, SystemTrayEvent, WindowEvent};
use tauri_plugin_positioner::{Position, WindowExt};

mod command;
mod tray;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
        .plugin(tauri_plugin_positioner::init())
        .system_tray(tray::tray(VERSION))
        .on_system_tray_event(tray_event)
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

fn tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let win = app.get_window("main").unwrap();
            let _ = win.move_window(Position::TrayCenter);

            if win.is_visible().unwrap() {
                win.hide().unwrap();
            } else {
                win.show().unwrap();
                win.set_focus().unwrap();
            }
            // Trigger loading animation
            block_on(set_tray_icon(app.clone())).unwrap();
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            // let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                // "toggle_window" => {
                //     let win = app.get_window("main").unwrap();
                //     let new_title = if win.is_visible().unwrap() {
                //         win.hide().unwrap();
                //         "Show Window"
                //     } else {
                //         win.show().unwrap();
                //         "Hide Window"
                //     };
                //     item_handle.set_title(new_title).unwrap();
                // }
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
    }
}

#[tauri::command]
async fn set_tray_icon(handle: AppHandle) -> Result<(), String> {
    let ms = 50; // loop interval ms
    let mut intv = tokio::time::interval(tokio::time::Duration::from_millis(ms));
    let icon_vec = tray::tray_icon_loading();
    tokio::spawn(async move {
        let mut i = 0;
        let handle = handle.tray_handle();
        loop {
            // Wait until next tick.
            intv.tick().await;
            #[cfg(target_os = "macos")]
            handle.set_icon_as_template(false).unwrap();
            handle.set_icon(icon_vec[i].clone()).unwrap();
            i = if i >= 29 { 0 } else { i + 1 };
            // force break for test
            if i >= 29 {
                #[cfg(target_os = "macos")]
                handle.set_icon_as_template(true).unwrap();
                handle.set_icon(tray::tray_icon()).unwrap();
                break;
            }
        }
    });
    Ok(())
}
