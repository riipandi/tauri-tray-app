// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use crate::{save_theme_value, Theme};
use tauri::{command, AppHandle, Runtime};

#[command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    save_theme_value(&app, theme);
    app.restart();
    Ok(())
}
