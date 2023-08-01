// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use simple_home_dir::*;

use crate::meta;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub zoom_factor: f64,

    #[serde(default)]
    pub enable_darkmode: bool,

    #[serde(default)]
    pub enable_auto_update: bool,

    #[serde(default)]
    pub exit_to_tray: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        return Self {
            zoom_factor: 1.0,
            enable_darkmode: false,
            enable_auto_update: true,
            exit_to_tray: true,
        };
    }
}

impl AppConfig {
    #[allow(dead_code)]
    fn config_path(handle: tauri::AppHandle) -> std::path::PathBuf {
        let config_dir = handle.path_resolver().app_config_dir().unwrap();
        return config_dir.join("settings.json");
    }

    fn config_dir() -> std::path::PathBuf {
        return home_dir().unwrap().join(".config").join(meta::APP_NAME);
    }

    fn custom_config_path() -> std::path::PathBuf {
        return Self::config_dir().join("config.json");
    }

    fn create_config_dir() {
        let config_dir = Self::config_dir();
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).unwrap();
        }
    }

    pub fn load() -> Self {
        let custom_config_path = Self::custom_config_path();

        if !custom_config_path.exists() {
            Self::create_config_dir();
            return Self::default();
        }

        let config_file = std::fs::File::open(custom_config_path).unwrap();

        return serde_json::from_reader(config_file).unwrap();
    }

    pub fn save(&self) {
        let custom_config_path = Self::custom_config_path();

        let config_file = std::fs::File::create(custom_config_path).unwrap();
        serde_json::to_writer_pretty(config_file, self).unwrap();
    }

    pub fn dark_mode_state(&self) -> &'static str {
        if self.enable_darkmode {
            return "Disable Dark Mode";
        } else {
            return "Enable Dark Mode";
        }
    }
}
