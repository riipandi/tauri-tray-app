// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

pub const PKG_ARCH: &'static str = std::env::consts::ARCH;
pub const PKG_OS: &'static str = std::env::consts::OS;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_TITLE: &'static str = "Tauri App";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const MAIN_WINDOW: &'static str = "main";
pub const SETTING_WINDOW: &'static str = "app-setting";

// The deeplink url will be: myapp://x-callback
// Replace `myapp` from `CFBundleURLSchemes` (Info.plist)
pub const SCHEME_PROTOCOL: &'static str = "x-callback";

// Informational metadata for the application
pub const FEEDBACK_URL: &'static str = "https://ripandis.com/feedback?product=tauri-tray-app";
pub const WEBSITE_URL: &'static str = "https://twitter.com/riipandi";

// Read value from envars, injected at compile time
// pub const API_BASE_URL: &'static str = env!("TAURI_API_BASE_URL");

// Disable webview native context menu.
// Optional, injected when webview loaded.
pub const JS_INIT_SCRIPT: &'static str = r#"
    (function() {
        document.addEventListener("contextmenu",
            (e) => { e.preventDefault(); return false; },
            { capture: true }
        );
    })();
"#;
