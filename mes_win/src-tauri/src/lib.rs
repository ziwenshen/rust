// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 导入模块
mod auth;
mod windows;
mod api;

// 使用新模块结构的功能
use api::auth::{login, get_current_user, get_current_token, is_logged_in, logout};
use windows::manager::{close, minimize, resize_window, toggle_maximize, open_profile_window};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            login, 
            get_current_user, 
            get_current_token, 
            is_logged_in, 
            logout,
            minimize, 
            close,
            resize_window,
            toggle_maximize,
            open_profile_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
