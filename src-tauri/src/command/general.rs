// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use log::info;
use tauri::api::dialog;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn open_devtools(window: tauri::Window) {
    if !window.is_devtools_open() {
        window.open_devtools()
    }
}

#[tauri::command]
pub fn set_darkmode(window: tauri::Window, enable: bool) {
    let theme = window.theme().unwrap();

    if enable {
        info!("switch to dark from {}", theme);
    } else {
        info!("switch to light from {}", theme);
    }

    dialog::message(Some(&window), "Information", "Not yet implemented!");
}
