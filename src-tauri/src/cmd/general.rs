// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use machine_uid;
use tauri::api::dialog;

use crate::utils;

#[tauri::command]
pub fn open_devtools(window: tauri::Window) {
    if !window.is_devtools_open() {
        window.open_devtools()
    }
}

#[tauri::command]
pub async fn check_update(handle: tauri::AppHandle) {
    utils::updater::check_update(handle).await
}

#[tauri::command]
pub fn get_machine_id() -> String {
    match machine_uid::get() {
        Ok(machine_id) => machine_id,
        Err(err) => format!("Error getting machine ID: {}", err),
    }
}

#[tauri::command]
pub fn set_darkmode(window: tauri::Window, enable: bool) {
    let theme = window.theme().unwrap();
    let msg;

    if enable {
        msg = format!("switch to dark from {}", theme);
        log::info!("switch to dark from {}", theme);
    } else {
        msg = format!("switch to light from {}", theme);
        log::info!("switch to light from {}", theme)
    }

    dialog::message(Some(&window), "Information", msg);
}
