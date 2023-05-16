// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::shell::open, Manager};
use tauri::{SystemTray, SystemTrayEvent,CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_positioner::{Position, WindowExt};

mod command;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn tray() -> SystemTray {
    let version = CustomMenuItem::new("version".to_string(), "Tray App v".to_string() + VERSION);
    let preferences = CustomMenuItem::new("preferences".to_string(), "Preferences");
    let on_twitter = CustomMenuItem::new("on_twitter".to_string(), "Follow on Twitter");
    let send_feedback = CustomMenuItem::new("send_feedback".to_string(), "Send Feedback");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Tray App").accelerator("Cmd+Q");

    let tray_menu = SystemTrayMenu::new()
        .add_item(version.disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(on_twitter)
        .add_item(send_feedback)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(preferences)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

  return SystemTray::new().with_menu(tray_menu);
}

fn main() {
    tauri::Builder::default().setup(|app| {
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
    .system_tray(tray())
    .on_system_tray_event(|app, event| {
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
    })
    .on_window_event(|event| match event.event() {
        tauri::WindowEvent::Focused(is_focused) => {
            // detect click outside of the focused window and hide the app
            if !is_focused {
                event.window().hide().unwrap();
            }
        }
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
