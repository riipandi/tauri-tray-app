// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use tauri::menu::{Menu, PredefinedMenuItem, Submenu};
use tauri::Runtime;

pub fn init<R: Runtime>(handle: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
    let menu_bar = Menu::new(handle)?;

    let app_menu = Submenu::new(handle, "App", true)?;
    app_menu.append_items(&[
        &PredefinedMenuItem::about(handle, None, None)?,
        &PredefinedMenuItem::separator(handle)?,
        &PredefinedMenuItem::services(handle, None)?,
        &PredefinedMenuItem::separator(handle)?,
        &PredefinedMenuItem::hide(handle, None)?,
        &PredefinedMenuItem::hide_others(handle, None)?,
        &PredefinedMenuItem::show_all(handle, None)?,
        &PredefinedMenuItem::separator(handle)?,
        &PredefinedMenuItem::quit(handle, None)?,
    ])?;

    menu_bar.append(&app_menu)?;

    let file_m = Submenu::new(handle, "&File", true)?;
    menu_bar.append_items(&[&file_m])?;

    Ok(menu_bar)
}
