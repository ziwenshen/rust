use yew::prelude::*;
use wasm_bindgen::prelude::*;
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

// 个人中心组件属性
#[derive(Properties, PartialEq)]
pub struct ProfileProps {
    pub username: String,
    pub is_visible: bool,
    pub on_close: Callback<()>,
    pub on_logout: Callback<()>,
}

// 个人中心组件
#[function_component(ProfilePanel)]
pub fn profile_panel(props: &ProfileProps) -> Html {
    let is_loading = use_state(|| false);

    // 处理退出登录
    let on_logout_click = {
        let is_loading = is_loading.clone();
        let on_logout = props.on_logout.clone();
        Callback::from(move |_e: MouseEvent| {
            is_loading.set(true);
            
            let is_loading_inner = is_loading.clone();
            let on_logout_inner = on_logout.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                if is_tauri_environment() {
                    // Tauri环境：调用后端API
                    let result = invoke("logout", serde_wasm_bindgen::to_value(&()).unwrap()).await;
                    
                    match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                        Ok(Ok(_)) => {
                            web_sys::console::log_1(&"用户退出登录成功".into());
                        }
                        _ => {
                            web_sys::console::log_1(&"退出登录失败，但继续执行本地清理".into());
                        }
                    }
                } else {
                    // 网页环境：模拟延迟
                    TimeoutFuture::new(500).await;
                    web_sys::console::log_1(&"模拟退出登录成功".into());
                }
                
                // 清除本地存储
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.remove_item("access_token");
                        let _ = storage.remove_item("username");
                    }
                }
                
                is_loading_inner.set(false);
                on_logout_inner.emit(());
            });
        })
    };

    // 处理关闭面板
    let on_panel_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_e: MouseEvent| {
            on_close.emit(());
        })
    };

    // 处理关闭按钮点击  
    let on_close_btn_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_e: MouseEvent| {
            on_close.emit(());
        })
    };

    // 阻止面板内部点击事件冒泡
    let on_panel_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    if !props.is_visible {
        web_sys::console::log_1(&"ProfilePanel: not visible, returning empty HTML".into());
        return html! {};
    }

    web_sys::console::log_1(&"ProfilePanel: visible, rendering panel".into());

    html! {
        <div class="profile-overlay" onclick={on_panel_close} style="background-color: rgba(255, 0, 0, 0.8); z-index: 99999;">
            <div class="profile-panel" onclick={on_panel_click} style="background: white; border: 3px solid red;">
                <div class="profile-header" style="background: blue; color: white;">
                    <div class="profile-avatar">
                        <span class="avatar-text">{ props.username.chars().next().unwrap_or('用').to_string().to_uppercase() }</span>
                    </div>
                    <div class="profile-info">
                        <h3 class="profile-username">{ &props.username }</h3>
                        <p class="profile-role">{ "管理员" }</p>
                    </div>
                    <button class="profile-close-btn" onclick={on_close_btn_click}>
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                    </button>
                </div>
                
                <div class="profile-content">
                    <div class="profile-section">
                        <h4>{ "账户信息" }</h4>
                        <div class="profile-item">
                            <span class="profile-label">{ "用户名:" }</span>
                            <span class="profile-value">{ &props.username }</span>
                        </div>
                        <div class="profile-item">
                            <span class="profile-label">{ "角色:" }</span>
                            <span class="profile-value">{ "系统管理员" }</span>
                        </div>
                        <div class="profile-item">
                            <span class="profile-label">{ "登录时间:" }</span>
                            <span class="profile-value">{ "刚刚" }</span>
                        </div>
                    </div>
                    
                    <div class="profile-section">
                        <h4>{ "系统设置" }</h4>
                        <div class="profile-actions">
                            <button class="profile-action-btn secondary">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4z"></path>
                                    <path d="M16 18v-2a4 4 0 0 0-8 0v2"></path>
                                </svg>
                                { "修改密码" }
                            </button>
                            <button class="profile-action-btn secondary">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <circle cx="12" cy="12" r="3"></circle>
                                    <path d="M12 1v6m0 6v6m11-7h-6m-6 0H1"></path>
                                </svg>
                                { "偏好设置" }
                            </button>
                        </div>
                    </div>
                </div>
                
                <div class="profile-footer">
                    <button 
                        class={classes!("profile-logout-btn", "danger", if *is_loading { Some("loading") } else { None })}
                        onclick={on_logout_click}
                        disabled={*is_loading}
                    >
                        if *is_loading {
                            <div class="spinner"></div>
                            { "退出中..." }
                        } else {
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
                                <polyline points="16,17 21,12 16,7"></polyline>
                                <line x1="21" y1="12" x2="9" y2="12"></line>
                            </svg>
                            { "退出登录" }
                        }
                    </button>
                </div>
            </div>
        </div>
    }
}
