// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod handler;
mod items;

pub use self::handler::*;

// macOS only
pub fn build_menu() -> tauri::Menu {
    #[cfg(target_os = "macos")]
    let menus = tauri::Menu::new().add_submenu(items::macos_app_menu());

    #[cfg(not(target_os = "macos"))]
    let menus = tauri::Menu::new();

    menus
        .add_submenu(items::file_menu())
        .add_submenu(items::edit_menu())
        .add_submenu(items::view_menu())
        .add_submenu(items::window_menu())
        .add_submenu(items::help_menu())
}
