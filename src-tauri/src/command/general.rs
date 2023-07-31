// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use log::{error, info};
use tauri::api::dialog;
use tauri::utils::platform;
use tauri::Manager;

use crate::meta;

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
    let msg;

    if enable {
        msg = format!("switch to dark from {}", theme);
        info!("switch to dark from {}", theme);
    } else {
        msg = format!("switch to light from {}", theme);
        info!("switch to light from {}", theme)
    }

    dialog::message(Some(&window), "Information", msg);
}

#[tauri::command]
pub async fn check_update(handle: tauri::AppHandle) {
    let window = handle.get_window(meta::MAIN_WINDOW).unwrap();
    let app_platform = platform::target_triple().unwrap();
    let target: &str;

    match app_platform.as_str() {
        "aarch64-apple-darwin" => target = "darwin-aarch64",
        "x86_64-apple-darwin" => target = "darwin-x86_64",
        "x86_64-pc-windows" => target = "windows-x86_64",
        "x86_64-unknown-linux" => target = "linux-x86_64",
        _ => target = "darwin-universal",
    }

    let builder = tauri::updater::builder(handle).target(target);

    match builder.check().await {
        Ok(update) => {
            if update.is_update_available() {
                dialog::ask(
                    Some(&window),
                    "Updates available",
                    "Do you want to download this version?",
                    |answer| {
                        if answer {
                            tauri::async_runtime::spawn(async move {
                                update.download_and_install().await.unwrap();
                            });
                        } else {
                            info!("update cancelled")
                        }
                    },
                );
            } else {
                let msg = format!(
                    "{} {} ({}) is currently the newest version available.",
                    meta::APP_TITLE,
                    meta::APP_VERSION,
                    target
                );
                dialog::message(Some(&window), "You're up-to-date!", msg);
            }
        }
        Err(error) => {
            dialog::message(Some(&window), "Failed to get update", error.to_string());
            error!("failed to get update: {}", error.to_string());
        }
    }
}
