// 一次性导入 Yew 框架中所有最核心和常用的项
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_timers::future::TimeoutFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// 检查是否在Tauri环境中运行
fn is_tauri_environment() -> bool {
    web_sys::window()
        .and_then(|w| w.get("__TAURI__"))
        .and_then(|t| t.dyn_into::<web_sys::js_sys::Object>().ok())
        .is_some()
}

// 模拟登录API调用（用于网页版）
async fn mock_login_api(username: &str, password: &str) -> Result<LoginData, String> {
    // 模拟网络延迟
    TimeoutFuture::new(1000).await;
    
    // 简单的用户名密码验证
    if username == "admin" && password == "admin" {
        Ok(LoginData {
            access_token: "mock_token_12345".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            username: username.to_string(),
            user_id: 1,
        })
    } else {
        Err("用户名或密码错误".to_string())
    }
}

// 登录组件属性
#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub on_login_success: Callback<String>, // 登录成功回调，传递用户名
}

// 登录请求参数
#[derive(Serialize)]
struct LoginArgs {
    username: String,
    password: String,
}

// API通用响应结构
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // 允许未使用的字段，这些字段来自API响应
struct ApiResponse<T> {
    success: bool,
    code: u32,
    message: String,
    data: Option<T>,
}

// 登录响应数据
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // 允许未使用的字段，这些字段来自API响应
struct LoginData {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "tokenType")]
    token_type: String,
    #[serde(rename = "expiresIn")]
    expires_in: u32,
    username: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

//定义登录表单的状态结构
// 宏定义允许结构体被克隆和比较
#[derive(Clone, PartialEq)]
pub struct LoginState{
    pub username: String,
    pub password: String,
    pub is_loading: bool, 
    pub error_message: Option<String>,
    pub show_message: bool,  // 控制消息弹窗显示
}

//实现默认值
impl Default for LoginState{
    fn default() -> Self {
        Self{
            username: String::new(),
            password: String::new(), 
            is_loading: false,
            error_message: None,
            show_message: false,      // 默认不显示消息弹窗
        }
    }
}

// 主要的登录组件
#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html { 
    // 定义不可变状态变量 使用 use_state 钩子来管理登录状态
    let login_state = use_state(LoginState::default);

    // 处理用户名输入变化
    let on_username_change = {
        let login_state = login_state.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*login_state).clone();
            new_state.username = input.value();
            login_state.set(new_state);
        })
    };

    // 处理密码输入变化
    let on_password_change = {
        let login_state = login_state.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*login_state).clone();
            new_state.password = input.value();
            login_state.set(new_state);
        })
    };

    // 处理登录按钮点击
    let on_login = {
        let login_state = login_state.clone();
        let login_success_callback = props.on_login_success.clone();
        Callback::from(move |_e: MouseEvent| {
            // 执行登录逻辑
            let mut new_state = (*login_state).clone();
            new_state.is_loading = true;
            new_state.error_message = None;
            new_state.show_message = false;
            login_state.set(new_state.clone());
            
            // 调用真实的登录 API
            let login_state_inner = login_state.clone();
            let login_success_callback_inner = login_success_callback.clone();
            let username = new_state.username.clone();
            let password = new_state.password.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let args = LoginArgs { username: username.clone(), password: password.clone() };
                
                let mut final_state = (*login_state_inner).clone();
                final_state.is_loading = false;
                
                // 检查是否在Tauri环境中
                if is_tauri_environment() {
                    // Tauri环境：调用原生API
                    let args_value = serde_wasm_bindgen::to_value(&args).unwrap();
                    let result = invoke("login", args_value).await;
                    
                    // 尝试解析API响应
                    if let Ok(api_response) = serde_wasm_bindgen::from_value::<ApiResponse<LoginData>>(result.clone()) {
                        if api_response.success && api_response.data.is_some() {
                            // 登录成功
                            let login_data = api_response.data.unwrap();
                            let username = login_data.username.clone();
                            
                            // 保存 token
                            if let Some(window) = web_sys::window() {
                                if let Ok(Some(storage)) = window.local_storage() {
                                    let _ = storage.set_item("access_token", &login_data.access_token);
                                    let _ = storage.set_item("username", &username);
                                }
                            }
                            
                            web_sys::console::log_1(&format!("登录成功！用户: {}", username).into());
                            
                            // 触发登录成功回调，切换到主界面
                            login_success_callback_inner.emit(username);
                            
                            // 清除登录表单状态
                            final_state.username.clear();
                            final_state.password.clear();
                            final_state.error_message = None;
                            final_state.show_message = false;
                        } else {
                            // 登录失败，使用API返回的错误消息
                            final_state.error_message = Some(api_response.message);
                            final_state.show_message = true;
                        }
                    } else {
                        // 解析失败，使用默认错误消息
                        let error_msg = if let Some(error_str) = result.as_string() {
                            error_str
                        } else {
                            "登录失败，请检查网络连接".to_string()
                        };
                        
                        final_state.error_message = Some(error_msg);
                        final_state.show_message = true;
                    }
                } else {
                    // 网页环境：使用模拟API
                    web_sys::console::log_1(&"网页版模式：使用模拟登录".into());
                    
                    match mock_login_api(&username, &password).await {
                        Ok(login_data) => {
                            let username = login_data.username.clone();
                            
                            // 保存 token
                            if let Some(window) = web_sys::window() {
                                if let Ok(Some(storage)) = window.local_storage() {
                                    let _ = storage.set_item("access_token", &login_data.access_token);
                                    let _ = storage.set_item("username", &username);
                                }
                            }
                            
                            web_sys::console::log_1(&format!("模拟登录成功！用户: {}", username).into());
                            
                            // 触发登录成功回调，切换到主界面
                            login_success_callback_inner.emit(username);
                            
                            // 清除登录表单状态
                            final_state.username.clear();
                            final_state.password.clear();
                            final_state.error_message = None;
                            final_state.show_message = false;
                        }
                        Err(error_msg) => {
                            // 登录失败
                            final_state.error_message = Some(error_msg);
                            final_state.show_message = true;
                        }
                    }
                }
                
                login_state_inner.set(final_state);
            });
        })
    };    // 处理窗口最小化
    let on_minimize = {
        Callback::from(move |_e: MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke("minimize", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            });
        })
    };

    // 处理窗口关闭
    let on_close = {
        Callback::from(move |_e: MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke("close", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            });
        })
    };

    // 关闭消息弹窗
    let on_close_message = {
        let login_state = login_state.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut new_state = (*login_state).clone();
            new_state.show_message = false;
            login_state.set(new_state);
        })
    };

    // 处理主题切换
    let on_theme_toggle = {
        Callback::from(move |_e: MouseEvent| {
            // 调用JavaScript函数切换主题
            if let Some(_window) = web_sys::window() {
                let _ = js_sys::eval("toggleTheme()");
            }
        })
    };
    
     html! {
        <div class="wechat-login-container">
            // Theme Toggle Button
            <button class="theme-toggle theme-toggle-login" onclick={on_theme_toggle} title="切换主题">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="sun-icon">
                    <circle cx="12" cy="12" r="5"></circle>
                    <line x1="12" y1="1" x2="12" y2="3"></line>
                    <line x1="12" y1="21" x2="12" y2="23"></line>
                    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                    <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                    <line x1="1" y1="12" x2="3" y2="12"></line>
                    <line x1="21" y1="12" x2="23" y2="12"></line>
                    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                    <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                </svg>
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="moon-icon" style="display: none;">
                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                </svg>
            </button>
            
            // Windows风格的窗口控制按钮
            <div class="window-titlebar" data-tauri-drag-region="true">
                <div class="window-controls">
                    <button class="window-control minimize" onclick={on_minimize}>
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <path d="M0,5 L10,5" stroke="currentColor" stroke-width="1"/>
                        </svg>
                    </button>
                    <button class="window-control close" onclick={on_close}>
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <path d="M0,0 L10,10 M0,10 L10,0" stroke="currentColor" stroke-width="1"/>
                        </svg>
                    </button>
                </div>
            </div>
            
            // 主登录区域
            <div class="login-main">
                // 头像区域
                <div class="avatar-section">
                    <div class="avatar">
                        <img src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Ccircle cx='50' cy='50' r='40' fill='%234a9eff'/%3E%3Ctext x='50' y='58' text-anchor='middle' fill='white' font-size='24' font-family='Arial'%3E用%3C/text%3E%3C/svg%3E" alt="头像" />
                    </div>
                </div>
                
                // 用户名区域
                <div class="username-section">
                    <h2 class="username">{"用户登录"}</h2>
                </div>
                
                // 登录表单
                <div class="login-form">
                    <div class="form-group">
                        <input 
                            type="text"
                            class="login-input"
                            placeholder="用户名"
                            value={login_state.username.clone()}
                            onchange={on_username_change}
                            disabled={login_state.is_loading}
                        />
                    </div>
                    
                    <div class="form-group">
                        <input 
                            type="password"
                            class="login-input"
                            placeholder="密码"
                            value={login_state.password.clone()}
                            onchange={on_password_change}
                            disabled={login_state.is_loading}
                        />
                    </div>
                    
                    <div class="login-actions">
                        <button 
                            type="button" 
                            class="wechat-login-button"
                            disabled={login_state.is_loading || login_state.username.is_empty() || login_state.password.is_empty()}
                            onclick={on_login}
                        >
                            if login_state.is_loading {
                                <span class="loading-text">{"登录中..."}</span>
                            } else {
                                <span>{"登录"}</span>
                            }
                        </button>
                    </div>
                </div>
                
                // 消息弹窗（替代原来的错误信息显示）
                if login_state.show_message && login_state.error_message.is_some() {
                    <div class="message-overlay">
                        <div class="message-dialog">
                            <div class="message-header">
                                <span class="message-title">{"提示"}</span>
                                <button class="message-close" onclick={on_close_message.clone()}>{"×"}</button>
                            </div>
                            <div class="message-content">
                                {login_state.error_message.as_ref().unwrap().clone()}
                            </div>
                            <div class="message-footer">
                                <button class="message-btn" onclick={on_close_message}>{"确定"}</button>
                            </div>
                        </div>
                    </div>
                }
            </div>
        </div>
    }


}