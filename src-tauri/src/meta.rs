// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

pub const MAIN_WINDOW: &'static str = "main";
pub const TRAY_MENU_ID: &'static str = "tray-menu";
pub const APP_DB_FILENAME: &'static str = "appdata.db";
pub const SETTING_WINDOW: &'static str = "settings";
pub const SETTING_WINDOW_WIDTH: f64 = 580.;
pub const SETTING_WINDOW_HEIGHT: f64 = 540.;

// Informational metadata for the application
pub const FEEDBACK_URL: &'static str = "https://ripandis.com/feedback?product=tauri-tray-app";
pub const WEBSITE_URL: &'static str = "https://twitter.com/intent/follow?screen_name=riipandi";

// Disable webview native context menu, injected when webview loaded.
pub const JS_INIT_SCRIPT: &'static str = r#"
    (function() {
        console.warn("Browser context menu has been disabled!")
        document.addEventListener("contextmenu",
            (e) => { e.preventDefault(); return false; },
            { capture: true }
        );
    })();
"#;
