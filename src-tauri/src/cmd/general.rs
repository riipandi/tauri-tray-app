// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use machine_uid;
use tauri::api::dialog;
use tauri::{WindowBuilder, WindowUrl};

use crate::{meta, utils};

#[tauri::command]
pub fn open_devtools(window: tauri::Window) {
    if !window.is_devtools_open() {
        window.open_devtools()
    }
}

#[tauri::command(async)]
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

#[tauri::command(rename_all = "snake_case", async)]
pub async fn create_child_window(id: String, title: String, window: tauri::Window) {
    let url = WindowUrl::default();

    let child = WindowBuilder::new(&window, id, url)
        .title(title)
        .inner_size(400.0, 300.0);

    #[cfg(target_os = "macos")]
    let child = child.parent_window(window.ns_window().unwrap());

    #[cfg(windows)]
    let child = child.parent_window(window.hwnd().unwrap());

    child
        .initialization_script(meta::JS_INIT_SCRIPT)
        .min_inner_size(620.0, 680.0)
        .inner_size(620.0, 680.0)
        .resizable(true)
        .enable_clipboard_access()
        .accept_first_mouse(true)
        .focused(true)
        .build()
        .unwrap();
}

#[tauri::command(rename_all = "snake_case", async)]
pub async fn open_settings_window(window: tauri::Window) {
    let url = WindowUrl::default();

    let child = WindowBuilder::new(&window, meta::SETTING_WINDOW, url).title("Preferences");

    #[cfg(target_os = "macos")]
    let child = child.parent_window(window.ns_window().unwrap());

    #[cfg(windows)]
    let child = child.parent_window(window.hwnd().unwrap());

    child
        .inner_size(640.0, 560.0)
        .initialization_script(meta::JS_INIT_SCRIPT)
        .resizable(false)
        .minimizable(false)
        .closable(true)
        .enable_clipboard_access()
        .accept_first_mouse(true)
        .decorations(true)
        .hidden_title(true)
        .focused(true)
        .build()
        .unwrap();
}

#[tauri::command(rename_all = "snake_case")]
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
