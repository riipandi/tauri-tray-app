// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use crate::{save_theme_value, Theme};
use tauri::{AppHandle, Runtime};

#[tauri::command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    let db_state: tauri::State<native_db::Database> = app.state();
    save_theme_value(db_state, theme);
    app.restart();
    Ok(())
}
