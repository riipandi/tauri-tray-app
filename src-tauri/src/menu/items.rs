// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn file_menu() -> Submenu {
    Submenu::new("File", Menu::new().add_native_item(MenuItem::CloseWindow))
}

pub fn edit_menu() -> Submenu {
    Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    )
}

pub fn view_menu() -> Submenu {
    let onboarding = CustomMenuItem::new("onboarding", "Onboarding");
    let darkmode =
        CustomMenuItem::new("darkmode", "Enable Dark Mode").accelerator("Shift+CmdOrCtrl+D");

    Submenu::new(
        "View",
        Menu::new()
            .add_item(onboarding)
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("zoom_in", "Zoom In").accelerator("CmdOrCtrl+Plus"))
            .add_item(CustomMenuItem::new("zoom_out", "Zoom Out").accelerator("CmdOrCtrl+-"))
            .add_item(CustomMenuItem::new("zoom_reset", "Reset Zoom").accelerator("CmdOrCtrl+0"))
            .add_native_item(MenuItem::Separator)
            .add_item(darkmode),
    )
}

pub fn window_menu() -> Submenu {
    Submenu::new(
        "Window",
        Menu::new()
            .add_item(CustomMenuItem::new("reload", "Reload").accelerator("CmdOrCtrl+R"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Zoom)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::EnterFullScreen),
    )
}

pub fn help_menu() -> Submenu {
    let menus = Menu::new()
        .add_item(CustomMenuItem::new("unimplemented", "Documentation"))
        .add_native_item(MenuItem::Separator)
        .add_item(CustomMenuItem::new("unimplemented", "Privacy Policy"))
        .add_item(CustomMenuItem::new("unimplemented", "Terms of Service"));

    #[cfg(debug_assertions)]
    let submenu = Submenu::new(
        "Help",
        menus.add_native_item(MenuItem::Separator).add_item(
            CustomMenuItem::new("devtools", "Developer Tools").accelerator("Alt+CmdOrCtrl+J"),
        ),
    );

    #[cfg(not(debug_assertions))]
    let submenu = Submenu::new(
        "Help",
        menus
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("send_feedback", "Send Feedback")),
    );

    submenu
}

#[cfg(target_os = "macos")]
pub fn macos_app_menu() -> Submenu {
    use crate::meta::{APP_TITLE, APP_VERSION};
    use tauri::AboutMetadata;

    let about_meta = AboutMetadata::new()
        .version(APP_VERSION)
        .authors(vec![env!("CARGO_PKG_AUTHORS").to_string()])
        .comments(String::from(env!("CARGO_PKG_DESCRIPTION")))
        .copyright(String::from(env!("CARGO_PKG_LICENSE")))
        .license(String::from(env!("CARGO_PKG_LICENSE")))
        .website(String::from(env!("CARGO_PKG_REPOSITORY")))
        .website_label(String::from("Homepage"));

    Submenu::new(
        APP_TITLE,
        Menu::new()
            .add_native_item(MenuItem::About(String::from(APP_TITLE), about_meta))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("preferences", "Preferences").accelerator("CmdOrCtrl+,"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("check_updates", "Check for Updates"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    )
}
