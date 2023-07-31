// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use simple_home_dir::*;

use crate::meta;

// $HOME/.config/<app_name>/config.json
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default)]
    pub zoom_factor: f64,

    #[serde(default)]
    pub dark_mode_enabled: bool,

    #[serde(default)]
    pub notification_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        return Self {
            zoom_factor: 1.0,
            dark_mode_enabled: false,
            notification_enabled: true,
        };
    }
}

impl AppConfig {
    fn config_dir() -> std::path::PathBuf {
        return home_dir().unwrap().join(".config").join(meta::APP_NAME);
    }

    fn config_path() -> std::path::PathBuf {
        return Self::config_dir().join("config.json");
    }

    fn create_config_dir() {
        let config_dir = Self::config_dir();

        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).unwrap();
        }
    }

    pub fn load() -> Self {
        let config_path = Self::config_path();

        if !config_path.exists() {
            Self::create_config_dir();
            return Self::default();
        }

        let config_file = std::fs::File::open(config_path).unwrap();

        return serde_json::from_reader(config_file).unwrap();
    }

    pub fn save(&self) {
        let config_path = Self::config_path();

        let config_file = std::fs::File::create(config_path).unwrap();
        serde_json::to_writer_pretty(config_file, self).unwrap();
    }

    #[allow(dead_code)]
    pub fn dark_mode_state(&self) -> &'static str {
        if self.dark_mode_enabled {
            return "Disable Dark Mode";
        } else {
            return "Enable Dark Mode";
        }
    }
}
