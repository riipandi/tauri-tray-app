// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{CustomMenuItem, Icon, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

#[cfg(target_os = "macos")]
pub(crate) fn tray(app_version: &str) -> SystemTray {
    let version = CustomMenuItem::new(
        "version".to_string(),
        "Tray App v".to_string() + app_version,
    );
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
