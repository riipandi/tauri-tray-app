// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use tauri::Icon;

pub fn tray_icon() -> Icon {
    Icon::Raw(include_bytes!("../../icons/tray-icon-256.png").to_vec())
}

use super::tray_icons;
use crate::cmd::general::check_update;

// #[tauri::command]
pub async fn set_tray_icon(app_handle: tauri::AppHandle) -> Result<(), String> {
    let ms = 50; // loop interval ms
    let mut intv = tokio::time::interval(tokio::time::Duration::from_millis(ms));
    let icon_vec = tray_icons::tray_icon_loading();

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
                handle.set_icon(tray_icons::tray_icon()).unwrap();
                break;
            }
        }
    });
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_0.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_1.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_2.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_3.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_4.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_5.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_6.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_7.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_8.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_9.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_10.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_11.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_12.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_13.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_14.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_15.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_16.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_17.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_18.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_19.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_20.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_21.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_22.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_23.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_24.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_25.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_26.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_27.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_28.png").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-macos/loading_29.png").to_vec(),
    ));
    icon_vec
}

#[cfg(not(target_os = "macos"))]
pub fn tray_icon_loading() -> Vec<Icon> {
    let mut icon_vec: Vec<Icon> = Vec::new();
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_0.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_1.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_2.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_3.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_4.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_5.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_6.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_7.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_8.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_9.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_10.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_11.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_12.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_13.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_14.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_15.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_16.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_17.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_18.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_19.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_20.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_21.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_22.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_23.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_24.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_25.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_26.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_27.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_28.ico").to_vec(),
    ));
    icon_vec.push(Icon::Raw(
        include_bytes!("../../icons/tray-windows/loading_29.ico").to_vec(),
    ));
    icon_vec
}
