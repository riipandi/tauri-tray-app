// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod builder;
mod command;
mod config;
mod fetcher;
mod menu;
mod meta;
mod scripts;
mod tray;
mod utils;

fn main() {
    builder::initialize()
}
