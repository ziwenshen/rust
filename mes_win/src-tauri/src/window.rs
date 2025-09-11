/// 窗口控制相关的Tauri命令
use tauri::{LogicalSize, Size, Manager};
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

/// 打开个人中心窗口
#[tauri::command]
pub async fn open_profile_window(app: tauri::AppHandle) -> Result<(), String> {
    // 检查窗口是否已存在
    if let Some(existing_window) = app.get_webview_window("profile") {
        // 如果窗口已存在，将其置于前台
        existing_window.set_focus().map_err(|e| e.to_string())?;
        println!("个人中心窗口已存在，置于前台");
        return Ok(());
    }

    let profile_window = tauri::WebviewWindowBuilder::new(
        &app,
        "profile", // 窗口标识
        tauri::WebviewUrl::App("public/profile.html".into())
    )
    .title("个人中心")
    .inner_size(400.0, 500.0)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .center()
    .always_on_top(false)
    .decorations(true)
    .build()
    .map_err(|e| {
        println!("创建个人中心窗口失败: {}", e);
        e.to_string()
    })?;
    
    println!("个人中心窗口创建成功");
    Ok(())
}
