// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

// Originally taken from: https://github.com/wyhaya/tauri-plugin-theme/tree/tauri-v1

mod platform;

use platform::set_theme;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::api::path::app_config_dir;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::utils::config::TauriConfig;
use tauri::{command, generate_handler, Config};
use tauri::{AppHandle, Runtime};

const PLUGIN_NAME: &str = "theme";
const CONFIG_FILENAME: &str = "tauri-plugin-theme";
const ERROR_MESSAGE: &str = "Get app config dir failed";

pub struct ThemePlugin;

impl ThemePlugin {
    #[cfg(target_os = "macos")]
    pub fn init<R: Runtime>(_config: &mut Config) -> TauriPlugin<R, TauriConfig> {
        use tauri::RunEvent;
        Builder::new(PLUGIN_NAME)
            .invoke_handler(generate_handler![get_theme, set_theme])
            .on_event(|app, e| {
                if let RunEvent::Ready = e {
                    let theme = saved_theme_value(&app.config());
                    let _ = set_theme(app.clone(), theme);
                }
            })
            .build()
    }

    #[cfg(target_os = "linux")]
    pub fn init<R: Runtime>(_config: &mut Config) -> TauriPlugin<R, TauriConfig> {
        Builder::new(PLUGIN_NAME)
            .invoke_handler(generate_handler![get_theme, set_theme])
            .setup(|app| {
                let theme = saved_theme_value(&app.config());
                let _ = set_theme(app.clone(), theme);
                Ok(())
            })
            .build()
    }

    #[cfg(target_os = "windows")]
    pub fn init<R: Runtime>(config: &mut Config) -> TauriPlugin<R, TauriConfig> {
        let theme = saved_theme_value(config);
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
}

#[command]
fn get_theme<R: Runtime>(app: AppHandle<R>) -> Result<Theme, ()> {
    let theme = saved_theme_value(&app.config());
    Ok(theme)
}

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

pub(crate) fn saved_theme_value(config: &Config) -> Theme {
    let p = app_config_dir(config)
        .expect(ERROR_MESSAGE)
        .join(CONFIG_FILENAME);
    fs::read_to_string(p)
        .map(Theme::from)
        .unwrap_or(Theme::Auto)
}

pub(crate) fn save_theme_value(config: &Config, theme: Theme) {
    let dir = app_config_dir(config).expect(ERROR_MESSAGE);
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
    }
    let p = dir.join(CONFIG_FILENAME);
    let _ = fs::write(p, theme.to_string());
}
