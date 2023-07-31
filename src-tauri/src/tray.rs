// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use log::info;
use tauri::{async_runtime::block_on, AppHandle, CustomMenuItem, Icon, Manager};
use tauri::{SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::meta::{APP_TITLE, FEEDBACK_URL, WEBSITE_URL};
use crate::utils::open_browser;

enum TrayIdentifier {
    ShowWindow,
    Preferences,
    Website,
    Feedback,
    UpdateCheck,
    Quit,
    Unimplemented,
}

impl Into<String> for TrayIdentifier {
    fn into(self) -> String {
        match self {
            TrayIdentifier::ShowWindow => "show_window".to_owned(),
            TrayIdentifier::Preferences => "preferences".to_owned(),
            TrayIdentifier::Website => "visit_website".to_owned(),
            TrayIdentifier::Feedback => "send_feedback".to_owned(),
            TrayIdentifier::UpdateCheck => "check_update".to_owned(),
            TrayIdentifier::Quit => "quit".to_owned(),
            TrayIdentifier::Unimplemented => "unimplemented".to_owned(),
        }
    }
}

impl From<String> for TrayIdentifier {
    fn from(val: String) -> Self {
        match val.as_str() {
            "show_window" => TrayIdentifier::ShowWindow,
            "preferences" => TrayIdentifier::Preferences,
            "visit_website" => TrayIdentifier::Website,
            "send_feedback" => TrayIdentifier::Feedback,
            "check_update" => TrayIdentifier::UpdateCheck,
            "quit" => TrayIdentifier::Quit,
            _ => TrayIdentifier::Unimplemented,
        }
    }
}

#[cfg(target_os = "macos")]
pub fn create_tray_menu() -> SystemTray {
    log::debug!("Registering system tray menu...");

    let main_window =
        CustomMenuItem::new(TrayIdentifier::ShowWindow, format!("Open {}", APP_TITLE));
    let preferences =
        CustomMenuItem::new(TrayIdentifier::Preferences, "Preferences").accelerator("CmdOrCtrl+,");
    let quit = CustomMenuItem::new(TrayIdentifier::Quit, format!("Quit {}", APP_TITLE))
        .accelerator("CmdOrCtrl+Q");

    let subitem_website = CustomMenuItem::new(TrayIdentifier::Website, "Follow on Twitter");
    let subitem_feedback = CustomMenuItem::new(TrayIdentifier::Feedback, "Send Feedback");
    let subitem_updates = CustomMenuItem::new(TrayIdentifier::UpdateCheck, "Check for Updates");
    let submenu_help = SystemTrayMenu::new()
        .add_item(subitem_website)
        .add_item(subitem_feedback)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(subitem_updates);

    let tray_menu = SystemTrayMenu::new()
        .add_item(main_window)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(preferences)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(SystemTraySubmenu::new("Help", submenu_help))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    return SystemTray::new().with_menu(tray_menu);
}

pub fn tray_icon() -> Icon {
    Icon::Raw(include_bytes!("../icons/128x128@2x.png").to_vec())
}

#[cfg(target_os = "macos")]
pub fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_0.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_1.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_2.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_3.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_4.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_5.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_6.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_7.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_8.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_9.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_10.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_11.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_12.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_13.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_14.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_15.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_16.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_17.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_18.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_19.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_20.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_21.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_22.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_23.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_24.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_25.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_26.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_27.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_28.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/macos/loading_29.png").to_vec(),
    ));
    icon_vec
}

#[cfg(target_os = "windows")]
pub fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_0.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_1.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_2.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_3.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_4.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_5.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_6.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_7.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_8.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_9.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_10.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_11.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_12.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_13.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_14.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_15.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_16.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_17.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_18.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_19.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_20.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_21.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_22.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_23.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_24.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_25.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_26.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_27.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_28.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/windows/loading_29.ico").to_vec(),
    ));
    icon_vec
}

#[tauri::command]
pub async fn set_tray_icon(handle: AppHandle) -> Result<(), String> {
    let ms = 50; // loop interval ms
    let mut intv = tokio::time::interval(tokio::time::Duration::from_millis(ms));
    let icon_vec = tray_icon_loading();
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
                handle.set_icon(tray_icon()).unwrap();
                break;
            }
        }
    });
    Ok(())
}

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            info!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            info!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            info!("system tray received a double click");
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
                    block_on(set_tray_icon(app.clone())).unwrap();
                    // crate::command::check_update(app.clone(), win);
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
