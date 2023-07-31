// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::Window;

pub fn zoom_webview(window: &Window, factor: f64) {
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
