// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 导入模块
mod auth;
mod window;

// 使用auth模块的功能
use auth::{login, get_current_user, get_current_token, is_logged_in, logout};
use window::{close, minimize};

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
            close
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
