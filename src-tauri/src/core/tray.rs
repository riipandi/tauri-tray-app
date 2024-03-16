// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

// @ref: https://github.com/tauri-apps/tauri/blob/dev/examples/api/src-tauri/src/lib.rs
// @ref: https://github.com/tauri-apps/muda/blob/dev/examples/wry.rs

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{image::Image, tray::TrayIconBuilder};
use tauri::{Manager, Runtime};

use crate::core::{cmd, utils};
use crate::meta;

pub fn init<R: Runtime>(app: &tauri::App<R>) -> tauri::Result<()> {
    let handle = app.app_handle();

    let toggle_i = MenuItem::with_id(handle, "toggle-window", "Hide Tauri App", true, None::<&str>)?;
    let setting_i = MenuItem::with_id(handle, "settings", "Settings...", true, Some("CmdOrCtrl+,"))?;

    #[cfg(target_os = "macos")]
    let set_title_i = MenuItem::with_id(handle, "set-tray-title", "Set Title", true, None::<&str>)?;

    let remove_tray_i = MenuItem::with_id(handle, "remove-tray", "Remove Tray", true, None::<&str>)?;
    let help_i = Submenu::new(handle, "&Help", true)?;
    let separator = PredefinedMenuItem::separator(handle)?;
    let quit_i = MenuItem::with_id(handle, "quit", "Quit Tauri App", true, None::<&str>)?;

    let feedback_i = MenuItem::with_id(handle, "feedback", "Send Feedback", true, None::<&str>)?;
    let website_i = MenuItem::with_id(handle, "website", "Visit Website", true, None::<&str>)?;

    help_i.append_items(&[&feedback_i, &website_i])?;

    let tray_menu_items = Menu::with_items(
        handle,
        &[
            &toggle_i,
            &separator,
            #[cfg(target_os = "macos")]
            &set_title_i,
            &remove_tray_i,
            &separator,
            &setting_i,
            &separator,
            &help_i,
            &separator,
            &quit_i,
        ],
    )?;

    let tray_icon = Image::from_bytes(include_bytes!("../../icons/tray-icon.png"));

    let _ = TrayIconBuilder::with_id(meta::TRAY_MENU_ID)
        .tooltip("Tauri")
        .icon(tray_icon.expect("failed to load tray icon"))
        .menu(&tray_menu_items)
        .menu_on_left_click(true)
        .on_menu_event(move |handle, event| match event.id.as_ref() {
            "quit" => handle.exit(0),
            "toggle-window" => {
                handle_toggle_window(&toggle_i, handle);
            }
            "settings" => {
                utils::handle_settings_window(handle);
            }
            "feedback" => {
                cmd::open_with_shell(meta::FEEDBACK_URL);
            }
            "website" => {
                cmd::open_with_shell(meta::WEBSITE_URL);
            }

            #[cfg(target_os = "macos")]
            "set-tray-title" => {
                if let Some(tray) = handle.tray_by_id(meta::TRAY_MENU_ID) {
                    let _ = tray.set_title(Some("Tauri App"));
                }
            }

            "remove-tray" => {
                handle.remove_tray_by_id(meta::TRAY_MENU_ID);
            }

            _ => {}
        })
        .build(handle);

    Ok(())
}

fn handle_toggle_window<R: Runtime>(menu_item: &MenuItem<R>, app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window(meta::MAIN_WINDOW) {
        let app_name = &app.app_handle().package_info().name;

        let new_title = if window.is_visible().unwrap_or_default() {
            let _ = window.hide();
            format!("Open {app_name}")
        } else {
            let _ = window.show();
            let _ = window.set_focus();
            format!("Hide {app_name}")
        };

        menu_item.set_text(new_title).expect("failed to set menu item text");
    }
}
