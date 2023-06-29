// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    if name.contains(' ') {
        Err("Names should not contain space".into())
    } else {
        Ok(format!("Hello, {}", name))
    }
}

fn main() {
    app::AppBuilder::new().run();
}
