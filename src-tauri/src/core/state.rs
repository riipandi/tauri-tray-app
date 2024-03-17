// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use native_db::{native_db, Database, InnerKeyValue};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

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
    NumberValue(f64),
    Null,
}

impl std::fmt::Display for SettingsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsValue::Null => write!(f, "null"),
            _ => write!(f, "{}", serde_json::to_string(&self).unwrap()),
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn load_settings(db: tauri::State<Database>) -> Vec<serde_json::Value> {
    let tx = db.r_transaction().expect("failed to create ro transaction");
    let settings = tx.scan().primary().expect("failed to read settings");
    let settings: Vec<Settings> = settings.all().collect();

    let settings_json: Vec<serde_json::Value> = settings
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

    log::debug!("Loaded settings: {:?}", settings_json);

    settings_json
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
pub fn save_setting(param: &str, value: &str, db: tauri::State<Database>) {
    let value = match value {
        "null" => SettingsValue::Null,
        _ => match serde_json::from_str(value) {
            Ok(SettingsValue::BooleanValue(_))
            | Ok(SettingsValue::NumberValue(_))
            | Ok(SettingsValue::StringValue(_)) => serde_json::from_str(value).unwrap(),
            _ => SettingsValue::StringValue(value.to_string()),
        },
    };

    let setting = Settings {
        param: param.to_string(),
        value: value.clone(),
    };

    let tx = db.rw_transaction().expect("failed to create transaction");
    tx.insert(setting).expect("failed to save setting");
    tx.commit().expect("failed to commit setting");

    log::info!("setting saved successfully: {}", value);
}
