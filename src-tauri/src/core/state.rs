// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use anyhow::anyhow;
use native_db::{native_db, Database, InnerKeyValue};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::fmt;
use tauri::Error;

use super::theme::Theme;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Settings {
    #[primary_key]
    pub param: String,
    pub value: SettingsValue,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SettingsValue {
    StringValue(String),
    BooleanValue(bool),
    NumberValue(i32),
    Null,
}

#[typeshare::typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct AppSettings {
    pub theme: Theme,
    pub zoom_factor: i32,
    pub auto_check_updates: bool,
}

impl fmt::Display for SettingsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsValue::Null => write!(f, "null"),
            _ => write!(f, "{}", serde_json::to_string(&self).unwrap()),
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_setting(param: &str, db: tauri::State<Database>) -> JsonValue {
    let tx = db.r_transaction().expect("failed to create ro transaction");
    let setting: Option<Settings> = tx.get().primary(param).expect("failed to read setting");
    setting.map_or_else(
        || JsonValue::Null, // Return null if setting not found
        |s| match s.value {
            SettingsValue::StringValue(val) => json!(val),
            SettingsValue::BooleanValue(val) => json!(val),
            SettingsValue::NumberValue(val) => json!(val),
            SettingsValue::Null => JsonValue::Null,
        },
    )
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_setting(param: &str, value: &str, db: tauri::State<Database>) -> tauri::Result<JsonValue> {
    let value: JsonValue = serde_json::from_str(value).unwrap_or_else(|_| JsonValue::String(value.to_string()));

    let svalue = match value {
        JsonValue::Null => SettingsValue::Null,
        JsonValue::Bool(b) => SettingsValue::BooleanValue(b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                SettingsValue::NumberValue(i as i32)
            } else if let Some(f) = n.as_f64() {
                SettingsValue::NumberValue(f as i32) // Assuming truncation to integer
            } else {
                return Err(Error::Anyhow(anyhow!("Unable to convert number to i32")));
            }
        }
        JsonValue::String(s) => SettingsValue::StringValue(s),
        _ => return Err(Error::Anyhow(anyhow!("Unsupported value type"))),
    };

    let param = param.to_string();
    let setting = Settings { param, value: svalue };
    log::debug!("saving setting: {:?}", setting);

    let tx = db.rw_transaction().expect("failed to create transaction");
    tx.insert(setting.clone()).expect("failed to save setting");
    tx.commit().expect("failed to commit setting");

    let setting_json = json!({ "param": &setting.param, "value": &setting.value});
    log::debug!("setting saved successfully: {:?}", setting_json);

    Ok(setting_json)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_settings_data(db: tauri::State<Database>) -> Vec<JsonValue> {
    let tx = db.r_transaction().expect("failed to create ro transaction");
    let settings = tx.scan().primary().expect("failed to read settings");
    let settings: Vec<Settings> = settings.all().collect();

    let settings_json: Vec<JsonValue> = settings
        .iter()
        .map(|setting| {
            let value_json = match &setting.value {
                SettingsValue::StringValue(s) => json!(s),
                SettingsValue::BooleanValue(b) => json!(b),
                SettingsValue::NumberValue(n) => json!(n),
                SettingsValue::Null => json!(null),
            };
            json!({ "param": &setting.param, "value": value_json})
        })
        .collect();

    log::debug!("settings: {:?}", settings_json);

    settings_json
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_app_settings(db: tauri::State<Database>) -> AppSettings {
    let tx = db.r_transaction().expect("failed to create ro transaction");
    let settings = tx.scan().primary().expect("failed to read settings");
    let settings: Vec<Settings> = settings.all().collect();

    let mut app_settings = AppSettings {
        theme: Theme::Auto,       // Set a default value for theme
        zoom_factor: 1,           // Set a default value for zoom_factor
        auto_check_updates: true, // Set a default value for auto_check_updates
    };

    for setting in settings {
        match setting.param.as_str() {
            "theme" => {
                if let SettingsValue::StringValue(theme_str) = setting.value {
                    app_settings.theme = Theme::from(theme_str)
                }
            }
            "zoom_factor" => {
                if let SettingsValue::NumberValue(zoom_factor) = setting.value {
                    app_settings.zoom_factor = zoom_factor;
                }
            }
            "auto_check_updates" => {
                if let SettingsValue::BooleanValue(auto_check_updates) = setting.value {
                    app_settings.auto_check_updates = auto_check_updates;
                }
            }
            _ => {} // Ignore unknown settings
        }
    }

    log::debug!("{:?}", app_settings);

    app_settings
}
