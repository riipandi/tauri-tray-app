// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use native_db::{native_db, Database, InnerKeyValue};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

// @ref: https://github.com/vincent-herlemont/native_db_tauri_vanilla

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Settings {
    #[primary_key]
    pub param: String,
    pub value: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn load_settings(db: tauri::State<Database>) -> Vec<Settings> {
    let tx = db.r_transaction().expect("failed to create ro transaction");
    let settings = tx.scan().primary().expect("failed to read settings");
    settings.all().collect()
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_setting(param: &str, value: &str, db: tauri::State<Database>) {
    let setting: Settings = Settings {
        param: param.to_string(),
        value: value.to_string(),
    };
    log::debug!("saving setting: {:?}", setting);
    let tx = db.rw_transaction().expect("failed to create transaction");
    tx.insert(setting).expect("failed to save setting");
    tx.commit().expect("failed to commit setting");
    log::info!("setting saved successfully");
}
