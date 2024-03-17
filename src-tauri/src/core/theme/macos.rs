// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use super::{save_theme_value, Theme};
use cocoa::{
    appkit::{NSAppearance, NSAppearanceNameVibrantDark, NSAppearanceNameVibrantLight, NSWindow},
    base::{id, nil},
};
use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    save_theme_value(&app, theme);
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
