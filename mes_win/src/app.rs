use crate::ui::pages::login::Login;
use crate::ui::pages::main_app::MainApp;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use serde_json;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

// 窗口大小调整参数
#[derive(Serialize, Deserialize)]
struct ResizeArgs {
    width: f64,
    height: f64,
}

// 应用状态
#[derive(Clone, PartialEq)]
pub struct AppState {
    pub is_logged_in: bool,
    pub username: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_logged_in: false,
            username: String::new(),
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(AppState::default);

    // 初始化
    {
        let app_state = app_state.clone();
        use_effect_with((), move |_| {
            // 设置登出事件监听器
            let app_state_for_logout = app_state.clone();
            spawn_local(async move {
                let logout_handler = Closure::wrap(Box::new(move |_event: JsValue| {
                    web_sys::console::log_1(&"收到登出事件，更新应用状态".into());
                    app_state_for_logout.set(AppState::default());
                    
                    // 清除本地存储
                    if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                        let _ = storage.remove_item("app_state");
                    }
                }) as Box<dyn Fn(JsValue)>);
                
                let _ = listen("logout", logout_handler.as_ref().unchecked_ref()).await;
                logout_handler.forget(); // 防止过早释放
            });
            
            // 恢复登录状态
            if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                if let Ok(Some(stored_state)) = storage.get_item("app_state") {
                    if let Ok(state_data) = serde_json::from_str::<serde_json::Value>(&stored_state) {
                        if let (Some(is_logged_in), Some(username)) = 
                            (state_data["is_logged_in"].as_bool(), state_data["username"].as_str()) {
                            app_state.set(AppState {
                                is_logged_in,
                                username: username.to_string(),
                            });

                            // 如果恢复的状态是已登录，立即调整窗口大小
                            if is_logged_in {
                                spawn_local(async move {
                                    let resize_args = serde_json::json!({
                                        "args": {
                                            "width": 1200,
                                            "height": 800
                                        }
                                    });
                                    let _result = invoke("resize_window", serde_wasm_bindgen::to_value(&resize_args).unwrap()).await;
                                    web_sys::console::log_1(&"恢复状态时窗口调整为主界面大小".into());
                                });
                            }
                        }
                    }
                }
            }
            || ()
        });
    }

    // 监听登录状态变化来改变窗口大小
    {
        use_effect_with(app_state.clone(), move |state| {
            if state.is_logged_in {
                web_sys::console::log_1(&"User logged in, resizing window to main size...".into());
                
                // 立即执行窗口调整，无需延迟
                spawn_local(async move {
                    let resize_args = serde_json::json!({
                        "args": {
                            "width": 1200.0,
                            "height": 800.0
                        }
                    });
                    
                    let js_args = serde_wasm_bindgen::to_value(&resize_args).unwrap();
                    let _result = invoke("resize_window", js_args).await;
                    web_sys::console::log_1(&format!("Window resized successfully to {}x{}", 1200.0, 800.0).into());
                });
            } else {
                web_sys::console::log_1(&"User not logged in, resizing window to login size...".into());
                spawn_local(async move {
                    let resize_args = serde_json::json!({
                        "args": {
                            "width": 400.0,
                            "height": 500.0
                        }
                    });
                    
                    let js_args = serde_wasm_bindgen::to_value(&resize_args).unwrap();
                    let _result = invoke("resize_window", js_args).await;
                    web_sys::console::log_1(&"Window resized successfully to login size".into());
                });
            }
            || ()
        });
    }

    // 处理登录成功
    let on_login_success = {
        let app_state = app_state.clone();
        Callback::from(move |username: String| {
            let new_state = AppState {
                is_logged_in: true,
                username: username.clone(),
            };
            app_state.set(new_state.clone());

            // 保存状态到本地存储
            if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                let state_json = serde_json::json!({
                    "is_logged_in": new_state.is_logged_in,
                    "username": new_state.username
                });
                let _ = storage.set_item("app_state", &state_json.to_string());
            }
        })
    };

    // 处理登出
    let on_logout = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.set(AppState::default());
            
            // 清除本地存储
            if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                let _ = storage.remove_item("app_state");
            }
        })
    };

    // 根据登录状态渲染不同界面
    if app_state.is_logged_in {
        html! {
            <MainApp 
                username={app_state.username.clone()} 
                on_logout={on_logout}
            />
        }
    } else {
        html! {
            <Login on_login_success={on_login_success} />
        }
    }
}
