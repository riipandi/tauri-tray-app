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

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

const CONFIG_FILENAME: &str = "tauri-plugin-theme";
const ERROR_MESSAGE: &str = "Get app config dir failed";

#[cfg(target_os = "windows")]
pub fn init<R: Runtime>(config: &mut Config) -> TauriPlugin<R> {
    let theme = saved_theme_value_from_config(&config);
    for window in &mut config.tauri.windows {
        match theme {
            Theme::Auto => window.theme = None,
            Theme::Light => window.theme = Some(tauri::Theme::Light),
            Theme::Dark => window.theme = Some(tauri::Theme::Dark),
        }
    }
    Builder::new(PLUGIN_NAME)
        .invoke_handler(generate_handler![get_theme, set_theme])
        .build()
}

#[tauri::command]
pub fn get_theme<R: Runtime>(app: AppHandle<R>) -> Result<Theme, ()> {
    let theme = saved_theme_value(&app);
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

#[cfg(target_os = "windows")]
pub fn saved_theme_value_from_config(config: &Config) -> Theme {
    if let Some(dir) = dirs_next::config_dir() {
        let p = dir.join(&config.tauri.bundle.identifier).join(CONFIG_FILENAME);
        return std::fs::read_to_string(p).map(Theme::from).unwrap_or(Theme::Auto);
    }
    Theme::Auto
}

pub fn saved_theme_value<R: Runtime>(app: &AppHandle<R>) -> Theme {
    let config_dir = app.path().app_config_dir().expect(ERROR_MESSAGE);
    let p = config_dir.join(CONFIG_FILENAME);
    std::fs::read_to_string(p).map(Theme::from).unwrap_or(Theme::Auto)
}

pub fn save_theme_value<R: Runtime>(app: &AppHandle<R>, theme: Theme) {
    let config_dir = app.path().app_config_dir().expect(ERROR_MESSAGE);
    if !config_dir.exists() {
        let _ = std::fs::create_dir_all(&config_dir);
    }
    let p = config_dir.join(CONFIG_FILENAME);
    let _ = std::fs::write(p, theme.to_string());
}
