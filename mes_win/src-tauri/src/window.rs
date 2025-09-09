/// 窗口控制相关的Tauri命令
use tauri::{LogicalSize, Size};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResizeArgs {
    width: f64,
    height: f64,
}

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

/// 切换最大化/还原窗口
#[tauri::command]
pub async fn toggle_maximize(window: tauri::Window) -> Result<(), String> {
    let is_maximized = window.is_maximized().map_err(|e| e.to_string())?;
    
    if is_maximized {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

/// 调整窗口大小
#[tauri::command]
pub async fn resize_window(window: tauri::Window, args: ResizeArgs) -> Result<(), String> {
    println!("调整窗口大小: {}x{}", args.width, args.height);
    
    let size = LogicalSize::new(args.width, args.height);
    window
        .set_size(Size::Logical(size))
        .map_err(|e| {
            println!("设置窗口大小失败: {}", e);
            e.to_string()
        })?;
    
    // 窗口居中
    window.center().map_err(|e| {
        println!("窗口居中失败: {}", e);
        e.to_string()
    })?;
    
    println!("窗口调整成功");
    Ok(())
}
