// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::CustomMenuItem;
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

use crate::meta;

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
pub fn build_menu() -> SystemTray {
    log::debug!("Registering system tray menu...");

    let main_window = CustomMenuItem::new(
        TrayIdentifier::ShowWindow,
        format!("Open {}", meta::APP_TITLE),
    );

    let preferences =
        CustomMenuItem::new(TrayIdentifier::Preferences, "Preferences").accelerator("CmdOrCtrl+,");

    let quit = CustomMenuItem::new(TrayIdentifier::Quit, format!("Quit {}", meta::APP_TITLE))
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
