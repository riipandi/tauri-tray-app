// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

use tauri::api::dialog;
use tauri::http::{Request, Response};
use tauri::{AppHandle, Manager};

use crate::meta;

pub fn callback(app: &AppHandle, req: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    let window = app.get_window(meta::MAIN_WINDOW).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
    log::info!("Callback URI: {:?}", req.uri());
    dialog::message(Some(&window), "Welcome Back", "Not yet implemented!");
    todo!()
}
