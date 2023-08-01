// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::api::dialog;
use tauri::utils::platform;
use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl};

use crate::config::AppConfig;
use crate::{meta, scripts, utils};

pub fn create_window(app: &AppHandle, label: &str, url: &str) {
    let app_config = AppConfig::load();
    let window_url = WindowUrl::App(std::path::PathBuf::from(url));
    let mut wb = WindowBuilder::new(app, label, window_url);

    wb = wb
        .initialization_script(scripts::JS_INIT_SCRIPT)
        .min_inner_size(620.0, 680.0)
        .inner_size(940.0, 720.0)
        .resizable(true)
        .enable_clipboard_access()
        .accept_first_mouse(true)
        .focused(true);

    #[cfg(target_os = "macos")]
    let window = wb
        .tabbing_identifier(meta::APP_NAME)
        .hidden_title(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.2 Safari/605.1.15")
        .theme(if app_config.enable_darkmode {
            Some(tauri::Theme::Dark)
        } else {Some(tauri::Theme::Light)})
        .build()
        .expect("error while creating window");

    #[cfg(not(target_os = "macos"))]
    let window = wb.build().expect("error while creating window");

    // zoom webview
    utils::zoom_webview(&window, app_config.zoom_factor);

    window
        .set_title(meta::APP_TITLE)
        .expect("error while setting window title");

    // Listen to an event on this window.
    window.listen("tauri://update-status".to_string(), move |msg| {
        log::info!("New status: {:?}", msg);
    });
}

pub fn zoom_webview(window: &tauri::Window, factor: f64) {
    window
        .with_webview(move |webview| {
            #[cfg(target_os = "linux")]
            {
                // see https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/struct.WebView.html
                // and https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/trait.WebViewExt.html
                use webkit2gtk::traits::WebViewExt;
                webview.inner().set_zoom_level(factor);
            }

            #[cfg(windows)]
            unsafe {
                // see https://docs.rs/webview2-com/0.19.1/webview2_com/Microsoft/Web/WebView2/Win32/struct.ICoreWebView2Controller.html
                webview.controller().SetZoomFactor(factor).unwrap();
            }

            #[cfg(target_os = "macos")]
            unsafe {
                let () = msg_send![webview.inner(), setPageZoom: factor];
            }
        })
        .expect("error while setting webview");
}

pub fn open_browser(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    }

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    }

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(url)
            .spawn()
            .expect("failed to open browser");
    }
}

// pub fn switch_theme(window: &Window, theme: Theme) {
//     window.theme().expect("error while setting webview");
// }

#[allow(dead_code)]
pub async fn check_update(handle: tauri::AppHandle) {
    let window = handle.get_window(meta::MAIN_WINDOW).unwrap();
    let app_platform = platform::target_triple().unwrap();
    let target: &str;

    match app_platform.as_str() {
        "aarch64-apple-darwin" => target = "darwin-aarch64",
        "x86_64-apple-darwin" => target = "darwin-x86_64",
        "x86_64-pc-windows" => target = "windows-x86_64",
        "x86_64-unknown-linux" => target = "linux-x86_64",
        _ => target = "darwin-universal",
    }

    let builder = tauri::updater::builder(handle.clone()).target(target);

    match builder.check().await {
        Ok(update) => {
            if update.is_update_available() {
                dialog::ask(
                    Some(&window),
                    "Updates available",
                    "Do you want to download this version?",
                    |answer| {
                        if answer {
                            tauri::async_runtime::spawn(async move {
                                update.download_and_install().await.unwrap();
                            });
                        } else {
                            log::info!("update cancelled")
                        }
                    },
                );
            } else {
                let msg = format!(
                    "{} {} ({}) is currently the newest version available.",
                    meta::APP_TITLE,
                    meta::APP_VERSION,
                    target
                );
                dialog::message(Some(&window), "You're up-to-date!", msg);
            }
        }
        Err(error) => {
            dialog::message(Some(&window), "Failed to get update", error.to_string());
            log::error!("failed to get update: {}", error.to_string());
        }
    }
}
