// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{meta, utils};
use tauri::WindowMenuEvent;

const SCALE_FACTOR: f64 = 1.1;

pub fn handle_menu_event(event: WindowMenuEvent) {
    let mut app_config = crate::config::AppConfig::load();
    let window = event.window();

    match event.menu_item_id() {
        "devtools" => window.open_devtools(),
        "reload" => window.eval("location.reload();").unwrap(),
        "preferences" => {
            let js_script = "window.location.replace('/settings')";
            window.eval(js_script).unwrap();
        }
        "onboarding" => {
            let js_script = "window.location.replace('/onboarding')";
            window.eval(js_script).unwrap();
        }
        "darkmode" => {
            app_config.dark_mode_enabled = !app_config.dark_mode_enabled;

            if app_config.dark_mode_enabled {
                window.eval("console.log('Enable Dark Mode');").unwrap();
            } else {
                window.eval("console.log('Disable Dark Mode');").unwrap();
            }

            app_config.save();

            let menu_handle = window.menu_handle();

            std::thread::spawn(move || {
                menu_handle
                    .get_item("darkmode")
                    .set_title(app_config.dark_mode_state())
                    .expect("failed to set dark reader menu text");
            });
        }
        "zoom_in" => {
            if app_config.zoom_factor < 2.0 {
                app_config.zoom_factor *= SCALE_FACTOR;
                utils::zoom_webview(window, app_config.zoom_factor);
                app_config.save();
            }
        }
        "zoom_out" => {
            if app_config.zoom_factor > 0.1 {
                app_config.zoom_factor /= SCALE_FACTOR;
                utils::zoom_webview(window, app_config.zoom_factor);
                app_config.save();
            }
        }
        "zoom_reset" => {
            if app_config.zoom_factor != 1.0 {
                app_config.zoom_factor = 1.0;
                utils::zoom_webview(window, app_config.zoom_factor);
                app_config.save();
            }
        }
        "send_feedback" => utils::open_browser(meta::FEEDBACK_URL),
        "quit" => std::process::exit(0),
        "close" => event.window().close().unwrap(),
        _ => {}
    }
}
