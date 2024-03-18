// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

// Learn more about Tauri commands at https://beta.tauri.app/features/commands

// When declaring arguments in Rust using snake_case, the arguments are converted to camelCase for JavaScript.
// To use snake_case in JavaScript, you have to declare it in the tauri::command statement
// @ref: https://tauri.app/v1/guides/features/command/#passing-arguments

use anyhow::anyhow;
use tauri::{AppHandle, Error, WebviewWindow};
use tauri_plugin_notification::NotificationExt;

use crate::core::utils;

#[tauri::command(rename_all = "snake_case")]
pub fn greet(name: &str, app: tauri::AppHandle) -> tauri::Result<String> {
    let app_title = &app.package_info().name;
    let notif_builder = app.notification().builder();

    let message = format!("Hello {name}, this message was sent from Rust.");
    if let Err(e) = notif_builder.body(message.clone()).title(app_title).show() {
        return Err(Error::Anyhow(anyhow!("failed to show notification: {:?}", e)));
    }

    Ok(message)
}

#[tauri::command(rename_all = "snake_case")]
pub fn toggle_devtools(window: WebviewWindow) {
    if !window.is_devtools_open() {
        window.open_devtools()
    } else if window.is_devtools_open() {
        window.close_devtools()
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_settings_window(handle: AppHandle) {
    utils::handle_settings_window(&handle)
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_with_shell(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    }

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    }

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    }
}
