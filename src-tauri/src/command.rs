// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
pub(crate) fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub(crate) fn open_link(href: &str) {
    println!("Open link: {}", href);
}
