// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

fn main() {
    // Read value from `.env` file (compile time)
    dotenv_build::output(dotenv_build::Config::default()).unwrap();
    tauri_build::build()
}
