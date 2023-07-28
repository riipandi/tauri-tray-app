// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{api::shell::open, async_runtime::block_on};
use tauri::{AboutMetadata, AppHandle, Icon, Manager, PhysicalPosition, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

#[cfg(target_os = "macos")]
pub(crate) fn register() -> SystemTray {
    let app_title = format!("{} v{}", "Tray App", crate::APP_VERSION);

    let version = CustomMenuItem::new("version".to_string(), app_title);
    let preferences = CustomMenuItem::new("preferences".to_string(), "Preferences")
        .accelerator("CommandOrControl+,");
    let check_updates = CustomMenuItem::new("check_updates".to_string(), "Check for Updates")
        .accelerator("CommandOrControl+R");
    let quit =
        CustomMenuItem::new("quit".to_string(), "Quit Tray App").accelerator("CommandOrControl+Q");

    let submenu_help = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("on_twitter", "Follow on Twitter"))
        .add_item(CustomMenuItem::new("send_feedback", "Send Feedback"));

    let tray_menu = SystemTrayMenu::new()
        .add_item(version.disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(preferences)
        .add_item(check_updates)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("about", "About Tray App"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(SystemTraySubmenu::new("Help", submenu_help))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    return SystemTray::new().with_menu(tray_menu);
}

#[cfg(target_os = "macos")]
pub(crate) fn tray_icon() -> Icon {
    Icon::Raw(include_bytes!("../icons/icon.png").to_vec())
}

#[cfg(target_os = "macos")]
pub(crate) fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_0.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_1.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_2.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_3.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_4.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_5.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_6.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_7.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_8.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_9.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_10.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_11.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_12.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_13.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_14.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_15.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_16.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_17.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_18.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_19.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_20.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_21.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_22.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_23.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_24.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_25.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_26.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_27.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_28.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/macos/loading_29.png").to_vec(),
    ));
    icon_vec
}

#[cfg(target_os = "windows")]
pub(crate) fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_0.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_1.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_2.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_3.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_4.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_5.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_6.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_7.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_8.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_9.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_10.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_11.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_12.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_13.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_14.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_15.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_16.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_17.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_18.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_19.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_20.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_21.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_22.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_23.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_24.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_25.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_26.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_27.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_28.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../assets/images/windows/loading_29.ico").to_vec(),
    ));
    icon_vec
}

pub(crate) fn tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { position, size, .. } => {
            let win: tauri::Window = app.get_window("main").unwrap();
            let is_visible = win.is_visible().unwrap();

            if is_visible {
                win.hide().unwrap();
            } else {
                let window_size = win.outer_size().unwrap();
                let physical_pos = PhysicalPosition {
                    x: position.x as i32 + (size.width as i32 / 2) - (window_size.width as i32 / 2),
                    y: position.y as i32 - window_size.height as i32,
                };

                let _ = win.set_position(tauri::Position::Physical(physical_pos));
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
                "about" => {
                    let about_metadata = AboutMetadata::new()
                        .version(crate::APP_VERSION)
                        .authors(vec![env!("CARGO_PKG_REPOSITORY").to_string()])
                        .website(env!("CARGO_PKG_REPOSITORY"))
                        .license(env!("CARGO_PKG_LICENSE"));

                    println!("{} {:?}", "Tray App", about_metadata);

                    // @todo - show about dialog
                }
                "preferences" => {
                    println!("tray menu Preferences clicked");
                }
                "on_twitter" => {
                    open(&app.shell_scope(), "https://twitter.com/riipandi", None).ok();
                }
                "check_updates" => {
                    // Trigger loading animation
                    block_on(set_tray_icon(app.clone())).unwrap();
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
