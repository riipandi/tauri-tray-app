// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

/**
 * Originally from https://github.com/wyhaya/tauri-plugin-theme
 * `wyhaya/tauri-plugin-theme` licensed under the MIT license.
 */

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(target_os = "linux")]
pub use self::linux::*;

use native_db::Database;
use serde::{Deserialize, Serialize};

#[tauri::command]
pub fn get_theme(db: tauri::State<Database>) -> Result<Theme, ()> {
    let theme = saved_theme_value(db);
    Ok(theme)
}

#[typeshare::typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Auto,
    Light,
    Dark,
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::Auto,
        }
    }
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::Auto => "auto".into(),
            Theme::Light => "light".into(),
            Theme::Dark => "dark".into(),
        }
    }
}

pub fn saved_theme_value(db: tauri::State<Database>) -> Theme {
    match super::state::get_setting("theme", db).as_str() {
        Some("light") => Theme::Light,
        Some("dark") => Theme::Dark,
        _ => Theme::Auto,
    }
}

pub fn save_theme_value(db: tauri::State<Database>, theme: Theme) {
    super::state::save_setting("theme", &theme.to_string(), db);
}
