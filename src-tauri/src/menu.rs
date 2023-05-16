// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Tray Menu (macOS only)
pub(crate) fn tray_menu() -> SystemTrayMenu {
    let version = CustomMenuItem::new("version".to_string(), "Version: ".to_string() + VERSION);
    let preferences = CustomMenuItem::new("preferences".to_string(), "Preferences");
    let on_twitter = CustomMenuItem::new("on_twitter".to_string(), "Follow on Twitter");
    let send_feedback = CustomMenuItem::new("send_feedback".to_string(), "Send Feedback");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Tray App");

    SystemTrayMenu::new()
        .add_item(version.disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(on_twitter)
        .add_item(send_feedback)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(preferences)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
}
