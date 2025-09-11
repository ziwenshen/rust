use super::types::{ApiResponse, LoginRequest};
use super::store::{UserSession, USER_STORE};
use super::client::AUTH_CLIENT;
use tauri::{Manager, Emitter};

/// 登录API调用
/// 
/// # 参数
/// * `username` - 用户名
/// * `password` - 密码
/// 
/// # 返回值
/// * `Ok(ApiResponse)` - 登录成功或失败的API响应
/// * `Err(String)` - 网络错误或其他系统错误
pub async fn login_api(username: String, password: String) -> Result<ApiResponse, String> {
    let login_request = LoginRequest { username, password };
    
    println!("发起登录请求: {:?}", login_request);
    
    // 使用认证客户端的原始客户端进行登录请求（不需要认证头）
    match AUTH_CLIENT
        .raw_client()
        .post("http://127.0.0.1:8080/api/auth/login")
        .json(&login_request)
        .send()
        .await
    {
        Ok(response) => {
            println!("响应状态: {}", response.status());
            
            // 不管是成功还是失败，都尝试解析为 ApiResponse
            match response.json::<ApiResponse>().await {
                Ok(api_response) => {
                    println!("API 响应: {:?}", api_response);
                    Ok(api_response)
                }
                Err(e) => {
                    println!("解析响应失败: {:?}", e);
                    Err("响应格式错误".to_string())
                }
            }
        }
        Err(e) => {
            println!("请求失败: {:?}", e);
            Err(format!("网络连接失败: {}", e))
        }
    }
}

/// Tauri命令：登录
#[tauri::command]
pub async fn login(username: String, password: String) -> Result<ApiResponse, String> {
    match login_api(username, password).await {
        Ok(api_response) => {
            // 如果登录成功，保存用户会话信息
            if api_response.success {
                if let Some(login_data) = &api_response.data {
                    let session = UserSession::from_login_data(login_data.clone());
                    USER_STORE.set_current_session(session);
                    println!("用户登录成功，会话信息已保存");
                }
            }
            Ok(api_response)
        }
        Err(e) => Err(e)
    }
}

/// Tauri命令：获取当前用户信息
#[tauri::command]
pub async fn get_current_user() -> Result<Option<(String, u32)>, String> {
    Ok(USER_STORE.get_current_user_info())
}

/// Tauri命令：获取当前认证Token
#[tauri::command]
pub async fn get_current_token() -> Result<Option<String>, String> {
    Ok(USER_STORE.get_current_auth_header())
}

/// Tauri命令：检查用户是否已登录
#[tauri::command]
pub async fn is_logged_in() -> Result<bool, String> {
    Ok(USER_STORE.get_valid_current_session().is_some())
}

/// 登出API调用
/// 
/// # 返回值
/// * `Ok(ApiResponse)` - 登出成功或失败的API响应
/// * `Err(String)` - 网络错误或其他系统错误
pub async fn logout_api() -> Result<ApiResponse, String> {
    println!("发起登出请求");
    
    // 使用认证客户端的POST方法（会自动添加Authorization头）
    match AUTH_CLIENT
        .post("http://localhost:8080/api/auth/logout")
    {
        Ok(request_builder) => {
            match request_builder
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    println!("登出响应状态: {}", response.status());
                    
                    // 首先获取响应文本
                    match response.text().await {
                        Ok(response_text) => {
                            println!("登出响应内容: {}", response_text);
                            
                            // 尝试解析为 ApiResponse
                            match serde_json::from_str::<ApiResponse>(&response_text) {
                                Ok(api_response) => {
                                    println!("登出API响应解析成功: {:?}", api_response);
                                    Ok(api_response)
                                }
                                Err(e) => {
                                    println!("解析登出响应失败: {:?}", e);
                                    println!("原始响应: {}", response_text);
                                    // 返回一个默认的成功响应
                                    Ok(ApiResponse {
                                        success: true,
                                        code: 200,
                                        message: "登出成功".to_string(),
                                        data: None,
                                    })
                                }
                            }
                        }
                        Err(e) => {
                            println!("读取响应文本失败: {:?}", e);
                            Err("无法读取响应内容".to_string())
                        }
                    }
                }
                Err(e) => {
                    println!("登出请求失败: {:?}", e);
                    Err(format!("登出请求失败: {}", e))
                }
            }
        }
        Err(e) => {
            println!("构建登出请求失败: {}", e);
            Err(e)
        }
    }
}

/// Tauri命令：注销用户
#[tauri::command]
pub async fn logout(app: tauri::AppHandle) -> Result<String, String> {
    println!("开始登出流程");
    
    // 先尝试调用后端API登出
    let logout_result: Result<ApiResponse, String> = logout_api().await;
    
    // 无论后端API调用是否成功，都清理本地会话
    // 这样即使网络问题导致API调用失败，用户也能在本地登出
    USER_STORE.clear_current_session();
    println!("本地会话已清理");
    
    // 关闭个人中心窗口（如果存在）
    if let Some(profile_window) = app.get_webview_window("profile") {
        if let Err(e) = profile_window.close() {
            println!("关闭个人中心窗口失败: {}", e);
        } else {
            println!("个人中心窗口已关闭");
        }
    }
    
    // 通知所有窗口登出事件
    if let Err(e) = app.emit("logout", ()) {
        println!("发送登出事件失败: {}", e);
    } else {
        println!("登出事件已发送");
    }
    
    match logout_result {
        Ok(api_response) => {
            if api_response.success {
                println!("登出成功");
                Ok("登出成功".to_string())
            } else {
                println!("后端登出失败: {}", api_response.message);
                // 即使后端返回失败，本地会话已清理，仍然返回成功
                Ok("本地登出成功".to_string())
            }
        }
        Err(e) => {
            println!("登出API调用失败: {}", e);
            // 即使API调用失败，本地会话已清理，仍然返回成功
            Ok("本地登出成功".to_string())
        }
    }
}
