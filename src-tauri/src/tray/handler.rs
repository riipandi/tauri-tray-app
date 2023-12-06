// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use tauri::async_runtime::block_on;
use tauri::{AppHandle, Manager, SystemTrayEvent};
use tauri_plugin_positioner::{Position, WindowExt};

use super::icons;
use crate::meta::{FEEDBACK_URL, WEBSITE_URL};
use crate::utils::{check_update, open_browser};

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            log::info!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            log::info!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            log::info!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let win: tauri::Window = app.get_window(crate::meta::MAIN_WINDOW).unwrap();

            match id.as_str() {
                "show_window" => {
                    if !win.is_visible().unwrap() {
                        let _ = win.move_window(Position::Center);
                    }
                    win.show().unwrap();
                    win.set_focus().unwrap();
                }
                "preferences" => {
                    if win.is_visible().unwrap() {
                        win.eval("window.location.replace('/settings')").unwrap();
                        win.set_focus().unwrap();
                    } else {
                        win.show().unwrap();
                        win.eval("window.location.replace('/settings')").unwrap();
                        win.set_focus().unwrap();
                    }
                }
                "check_update" => {
                    // Trigger loading animation
                    block_on(super::set_tray_icon(app.clone())).unwrap();
                }
                "visit_website" => open_browser(WEBSITE_URL),
                "send_feedback" => open_browser(FEEDBACK_URL),
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

// #[tauri::command]
pub async fn set_tray_icon(app_handle: AppHandle) -> Result<(), String> {
    let ms = 50; // loop interval ms
    let mut intv = tokio::time::interval(tokio::time::Duration::from_millis(ms));
    let icon_vec = icons::tray_icon_loading();

    check_update(app_handle.clone()).await;

    tokio::spawn(async move {
        let mut i = 0;
        let handle = app_handle.tray_handle();

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
                handle.set_icon(icons::tray_icon()).unwrap();
                break;
            }
        }
    });
    Ok(())
}
