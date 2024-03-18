// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use cocoa::appkit::{NSAppearance, NSAppearanceNameVibrantDark, NSAppearanceNameVibrantLight, NSWindow};
use cocoa::base::{id, nil};
use tauri::{AppHandle, Manager, Runtime};

use super::{save_theme_value, Theme};

#[tauri::command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    let db_state: tauri::State<native_db::Database> = app.state();
    save_theme_value(db_state, theme);

    for window in app.webview_windows().values() {
        let ptr = window.ns_window().map_err(|_| "Invalid window handle")?;
        unsafe {
            let val = match theme {
                Theme::Auto => nil,
                Theme::Light => NSAppearance(NSAppearanceNameVibrantLight),
                Theme::Dark => NSAppearance(NSAppearanceNameVibrantDark),
            };
            (ptr as id).setAppearance(val);
        }
    }

    Ok(())
}
