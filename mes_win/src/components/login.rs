// 一次性导入 Yew 框架中所有最核心和常用的项
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

//定义登录表单的状态结构
// 宏定义允许结构体被克隆和比较
#[derive(Clone, PartialEq)]
pub struct LoginState{
    pub username: String,
    pub password: String,
    pub is_loading: bool, 
    pub error_message: Option<String>,
    pub is_logged_in: bool,  // 添加登录状态标记
    pub saved_username: String, // 保存的用户名（用于已登录状态显示）
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
            is_logged_in: false,      // 默认未登录
            saved_username: "管理员".to_string(), // 模拟已保存的用户名
            show_message: false,      // 默认不显示消息弹窗
        }
    }
}

// 主要的登录组件
#[function_component(Login)]
pub fn login() -> Html { 
    // 定义不可变状态变量 使用 use_state 钩子来管理登录状态
    let login_state = use_state(|| {
        let mut state = LoginState::default();
        // 设为 false 来显示登录表单，设为 true 来显示已登录状态
        state.is_logged_in = false; // 改为 false 来测试登录表单
        state
    });

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
        Callback::from(move |_e: MouseEvent| {
            if login_state.is_logged_in {
                // 如果已登录，进入系统
                // 这里可以跳转到主应用界面
                web_sys::console::log_1(&"进入系统...".into());
            } else {
                // 执行登录逻辑
                let mut new_state = (*login_state).clone();
                new_state.is_loading = true;
                new_state.error_message = None;
                login_state.set(new_state.clone());
                
                // 模拟登录验证（实际应用中应该调用 Tauri 命令）
                let login_state_inner = login_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // 模拟网络延迟
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                    
                    let mut final_state = (*login_state_inner).clone();
                    final_state.is_loading = false;
                    
                    // 简单的验证逻辑（实际应用中应该更安全）
                    if new_state.username == "admin" && new_state.password == "123456" {
                        final_state.is_logged_in = true;
                        final_state.saved_username = new_state.username.clone();
                        final_state.username.clear();
                        final_state.password.clear();
                        final_state.error_message = None;
                        final_state.show_message = false;
                    } else {
                        final_state.error_message = Some("用户名或密码错误".to_string());
                        final_state.show_message = true; // 显示弹窗
                    }
                    
                    login_state_inner.set(final_state);
                });
            }
        })
    };

    // 处理切换账号
    let on_switch_account = {
        let login_state = login_state.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut new_state = (*login_state).clone();
            new_state.is_logged_in = false;
            new_state.username.clear();
            new_state.password.clear();
            new_state.error_message = None;
            login_state.set(new_state);
        })
    };

    // 处理窗口最小化
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
    
     html! {
        <div class="wechat-login-container">
            // 窗口控制栏
            <div class="window-controls">
                <div class="window-title">{"MES管理系统"}</div>
                <div class="control-buttons">
                    <button class="control-btn minimize" onclick={on_minimize}>{"−"}</button>
                    <button class="control-btn close" onclick={on_close}>{"×"}</button>
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
                
                // 用户名区域 - 根据登录状态显示不同内容
                <div class="username-section">
                    if login_state.is_logged_in {
                        <h2 class="username">{&login_state.saved_username}</h2>
                    } else {
                        <h2 class="username">{"用户登录"}</h2>
                    }
                </div>
                
                // 登录表单或登录按钮
                if login_state.is_logged_in {
                    // 已登录状态 - 显示进入系统按钮
                    <div class="login-actions">
                        <button 
                            type="button" 
                            class="wechat-login-button"
                            disabled={login_state.is_loading}
                            onclick={on_login.clone()}
                        >
                            <span>{"进入MES系统"}</span>
                        </button>
                    </div>
                    
                    // 底部选项
                    <div class="bottom-options">
                        <button class="option-btn" onclick={on_switch_account}>{"切换账号"}</button>
                        <span class="divider">{"丨"}</span>
                        <button class="option-btn">{"仅传输文件"}</button>
                    </div>
                } else {
                    // 未登录状态 - 显示登录表单
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
                }
                
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