// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// pub const PKG_ARCH: &'static str = std::env::consts::ARCH;
// pub const PKG_OS: &'static str = std::env::consts::OS;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_TITLE: &'static str = "Tauri App";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const MAIN_WINDOW: &'static str = "main";
pub const USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.2 Safari/605.1.15";

// informational metadata
pub const FEEDBACK_URL: &'static str = "https://ripandis.com/feedback?product=tauri-tray-app";
pub const WEBSITE_URL: &'static str = "https://twitter.com/riipandi";
