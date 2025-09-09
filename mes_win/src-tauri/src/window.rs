/// 窗口控制相关的Tauri命令

/// 最小化窗口
#[tauri::command]
pub async fn minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

/// 关闭窗口
#[tauri::command]
pub async fn close(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}
