use super::types::{ApiResponse, LoginRequest};
use super::store::{UserSession, USER_STORE};
use super::client::AUTH_CLIENT;

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

/// Tauri命令：注销用户
#[tauri::command]
pub async fn logout() -> Result<(), String> {
    USER_STORE.clear_current_session();
    Ok(())
}
