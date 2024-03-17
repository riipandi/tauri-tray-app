// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};

use crate::meta;

pub fn get_app_user_agent<R: Runtime>(handle: &AppHandle<R>) -> String {
    // User-Agent: Mozilla/5.0 (<system-information>) <platform> (<platform-details>) <extensions>
    // @ref: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    // @ref: https://www.useragents.me

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let app_name = &handle.package_info().name;
    let app_version = &handle.package_info().version;
    let user_agent: String;

    #[cfg(target_os = "macos")]
    {
        user_agent = format!("Mozilla/5.0 ({os}; Intel Mac OS X 10_15_7; {arch}) AppleWebKit/537.36 (KHTML, like Gecko) Version/16.2 {app_name}/{app_version}");
    }

    #[cfg(target_os = "windows")]
    {
        user_agent = format!("Mozilla/5.0 ({os}; Win64; {arch}) AppleWebKit/537.36 (KHTML, like Gecko) Version/16.2 {app_name}/{app_version}");
    }

    #[cfg(target_os = "linux")]
    {
        user_agent = format!("Mozilla/5.0 ({os}; Linux {arch}) AppleWebKit/537.36 (KHTML, like Gecko) Version/16.2 {app_name}/{app_version}");
    }

    user_agent
}

pub fn handle_settings_window<R: Runtime>(handle: &tauri::AppHandle<R>) {
    if let Some(main_window) = handle.get_webview_window(meta::MAIN_WINDOW) {
        let is_main_window_visible = main_window.is_visible().expect("failed to get window visibility");

        // Get saved theme from settings
        // TODO: detect current system theme for auto mode
        let db_state: tauri::State<native_db::Database> = handle.state();
        let saved_theme = match super::state::get_setting("theme", db_state).as_str() {
            Some("light") => Some(tauri::Theme::Light),
            Some("dark") => Some(tauri::Theme::Dark),
            Some("auto") => None,
            _ => None,
        };

        if !is_main_window_visible {
            main_window.show().expect("failed to show window");
            main_window.set_focus().expect("failed to set focus");
        }

        if handle.get_webview_window(meta::SETTING_WINDOW).is_none() {
            let setting_window =
                WebviewWindowBuilder::new(handle, meta::SETTING_WINDOW, WebviewUrl::App("/settings".into()))
                    .title("Settings")
                    .initialization_script(meta::JS_INIT_SCRIPT)
                    .min_inner_size(meta::SETTING_WINDOW_WIDTH, meta::SETTING_WINDOW_HEIGHT)
                    .max_inner_size(meta::SETTING_WINDOW_WIDTH, meta::SETTING_WINDOW_HEIGHT)
                    .inner_size(meta::SETTING_WINDOW_WIDTH, meta::SETTING_WINDOW_HEIGHT)
                    .theme(saved_theme)
                    .resizable(false)
                    .minimizable(false)
                    .maximizable(false)
                    .closable(true)
                    .enable_clipboard_access()
                    .accept_first_mouse(true)
                    .decorations(true)
                    .focused(true)
                    .shadow(true);

            #[cfg(target_os = "macos")]
            let setting_window = setting_window
                .title_bar_style(tauri::TitleBarStyle::Overlay)
                .hidden_title(true);

            setting_window
                .parent(&main_window)
                .expect("failed to get parent window")
                .build()
                .expect("failed to build setting window");
        } else {
            let setting_window = handle
                .get_webview_window(meta::SETTING_WINDOW)
                .expect("Settings window not found");
            let _ = setting_window.show();
            let _ = setting_window.set_focus();
        }
    }
}
