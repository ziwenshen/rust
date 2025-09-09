use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// 主界面属性
#[derive(Properties, PartialEq)]
pub struct MainAppProps {
    pub username: String,
    pub on_logout: Callback<()>,
}

// 菜单项枚举
#[derive(Clone, PartialEq)]
pub enum MenuItem {
    Dashboard,
    Production,
    Inventory,
    Quality,
    Settings,
}

// 主界面组件
#[function_component(MainApp)]
pub fn main_app(props: &MainAppProps) -> Html {
    let selected_menu = use_state(|| MenuItem::Dashboard);

    // 处理菜单选择
    let on_menu_select = {
        let selected_menu = selected_menu.clone();
        Callback::from(move |item: MenuItem| {
            selected_menu.set(item);
        })
    };

    // 处理退出登录
    let _on_logout = {
        let callback = props.on_logout.clone();
        Callback::from(move |_e: MouseEvent| {
            // 清除本地存储的token
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.remove_item("access_token");
                    let _ = storage.remove_item("username");
                }
            }
            web_sys::console::log_1(&"用户退出登录".into());
            callback.emit(());
        })
    };

    // 处理窗口最小化
    let _on_minimize = {
        Callback::from(move |_e: MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke("minimize", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            });
        })
    };

    // 处理窗口关闭
    let _on_close = {
        Callback::from(move |_e: MouseEvent| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = invoke("close", serde_wasm_bindgen::to_value(&()).unwrap()).await;
            });
        })
    };

    html! {
        <div class="main-app-container">
            // Windows风格的窗口控制按钮
            <div class="window-titlebar" data-tauri-drag-region="true">
                <div class="window-controls">
                    <button class="window-control minimize" onclick={
                        Callback::from(move |_e: MouseEvent| {
                            wasm_bindgen_futures::spawn_local(async move {
                                let _ = invoke("minimize", serde_wasm_bindgen::to_value(&()).unwrap()).await;
                            });
                        })
                    }>
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <path d="M0,5 L10,5" stroke="currentColor" stroke-width="1"/>
                        </svg>
                    </button>
                    <button class="window-control maximize" onclick={
                        Callback::from(move |_e: MouseEvent| {
                            wasm_bindgen_futures::spawn_local(async move {
                                let _ = invoke("toggle_maximize", serde_wasm_bindgen::to_value(&()).unwrap()).await;
                            });
                        })
                    }>
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <path d="M0,0 L10,0 L10,10 L0,10 Z" stroke="currentColor" stroke-width="1" fill="none"/>
                        </svg>
                    </button>
                    <button class="window-control close" onclick={
                        Callback::from(move |_e: MouseEvent| {
                            wasm_bindgen_futures::spawn_local(async move {
                                let _ = invoke("close", serde_wasm_bindgen::to_value(&()).unwrap()).await;
                            });
                        })
                    }>
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <path d="M0,0 L10,10 M0,10 L10,0" stroke="currentColor" stroke-width="1"/>
                        </svg>
                    </button>
                </div>
            </div>

            // 主要内容区域
            <main class="app-main">
                // 左侧菜单栏
                <aside class="sidebar">
                    <nav class="menu">
                        <div class="menu-item-container">
                            <div class={if *selected_menu == MenuItem::Dashboard { "menu-item active" } else { "menu-item" }}
                                 onclick={
                                     let on_menu_select = on_menu_select.clone();
                                     Callback::from(move |_| on_menu_select.emit(MenuItem::Dashboard))
                                 }>
                                <div class="menu-icon">
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                        <rect x="3" y="3" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                                        <rect x="14" y="3" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                                        <rect x="14" y="14" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                                        <rect x="3" y="14" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                                    </svg>
                                </div>
                                <span class="menu-text">{"仪表板"}</span>
                            </div>
                            
                            <div class={if *selected_menu == MenuItem::Production { "menu-item active" } else { "menu-item" }}
                                 onclick={
                                     let on_menu_select = on_menu_select.clone();
                                     Callback::from(move |_| on_menu_select.emit(MenuItem::Production))
                                 }>
                                <div class="menu-icon">
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                        <path d="M9 7H6a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3" stroke="currentColor" stroke-width="2"/>
                                        <rect x="9" y="1" width="6" height="6" rx="2" stroke="currentColor" stroke-width="2"/>
                                    </svg>
                                </div>
                                <span class="menu-text">{"生产管理"}</span>
                            </div>
                            
                            <div class={if *selected_menu == MenuItem::Inventory { "menu-item active" } else { "menu-item" }}
                                 onclick={
                                     let on_menu_select = on_menu_select.clone();
                                     Callback::from(move |_| on_menu_select.emit(MenuItem::Inventory))
                                 }>
                                <div class="menu-icon">
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                        <path d="M21 16V8a2 2 0 0 0-1-1.73L12 2L4 6.27A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73L12 22l8-4.27A2 2 0 0 0 21 16z" stroke="currentColor" stroke-width="2"/>
                                    </svg>
                                </div>
                                <span class="menu-text">{"库存管理"}</span>
                            </div>
                            
                            <div class={if *selected_menu == MenuItem::Quality { "menu-item active" } else { "menu-item" }}
                                 onclick={
                                     let on_menu_select = on_menu_select.clone();
                                     Callback::from(move |_| on_menu_select.emit(MenuItem::Quality))
                                 }>
                                <div class="menu-icon">
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                        <path d="M9 12l2 2 4-4" stroke="currentColor" stroke-width="2"/>
                                        <path d="M21 12c-1 0-3-1-3-3s2-3 3-3 3 1 3 3-2 3-3 3" stroke="currentColor" stroke-width="2"/>
                                        <path d="M3 12c1 0 3-1 3-3s-2-3-3-3-3 1-3 3 2 3 3 3" stroke="currentColor" stroke-width="2"/>
                                        <path d="M12 21c0-1-1-3-3-3s-3 2-3 3 1 3 3 3 3-2 3-3" stroke="currentColor" stroke-width="2"/>
                                        <path d="M12 3c0 1-1 3-3 3s-3-2-3-3 1-3 3-3 3 2 3 3" stroke="currentColor" stroke-width="2"/>
                                    </svg>
                                </div>
                                <span class="menu-text">{"质量管理"}</span>
                            </div>
                            
                            <div class={if *selected_menu == MenuItem::Settings { "menu-item active" } else { "menu-item" }}
                                 onclick={
                                     let on_menu_select = on_menu_select.clone();
                                     Callback::from(move |_| on_menu_select.emit(MenuItem::Settings))
                                 }>
                                <div class="menu-icon">
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                        <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2"/>
                                        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" stroke="currentColor" stroke-width="2"/>
                                    </svg>
                                </div>
                                <span class="menu-text">{"系统设置"}</span>
                            </div>
                        </div>
                    </nav>
                </aside>

                // 右侧内容区域
                <section class="content-area">
                    { render_content(&selected_menu) }
                </section>
            </main>
        </div>
    }
}

// 渲染不同内容区域
fn render_content(selected_menu: &UseStateHandle<MenuItem>) -> Html {
    match **selected_menu {
        MenuItem::Dashboard => html! {
            <div class="content-panel">
                <div class="panel-header">
                    <h2>{"仪表板"}</h2>
                    <p>{"系统概览和关键指标"}</p>
                </div>
                <div class="dashboard-grid">
                    <div class="dashboard-card">
                        <div class="card-icon production">
                            <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                <rect x="3" y="3" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
                                <path d="M9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2"/>
                            </svg>
                        </div>
                        <div class="card-content">
                            <h3>{"生产订单"}</h3>
                            <div class="card-value">{"24"}</div>
                            <div class="card-description">{"今日活跃订单"}</div>
                        </div>
                    </div>
                    <div class="dashboard-card">
                        <div class="card-icon inventory">
                            <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                <path d="M21 16V8a2 2 0 0 0-1-1.73L12 2L4 6.27A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73L12 22l8-4.27A2 2 0 0 0 21 16z" stroke="currentColor" stroke-width="2"/>
                            </svg>
                        </div>
                        <div class="card-content">
                            <h3>{"库存状态"}</h3>
                            <div class="card-value">{"正常"}</div>
                            <div class="card-description">{"所有物料充足"}</div>
                        </div>
                    </div>
                    <div class="dashboard-card">
                        <div class="card-icon quality">
                            <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                <path d="M9 12l2 2 4-4" stroke="currentColor" stroke-width="2"/>
                                <circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2"/>
                            </svg>
                        </div>
                        <div class="card-content">
                            <h3>{"质量指标"}</h3>
                            <div class="card-value">{"99.2%"}</div>
                            <div class="card-description">{"合格率"}</div>
                        </div>
                    </div>
                </div>
            </div>
        },
        MenuItem::Production => html! {
            <div class="content-panel">
                <div class="panel-header">
                    <h2>{"生产管理"}</h2>
                    <p>{"管理生产订单和工艺流程"}</p>
                </div>
                <div class="content-placeholder">
                    <div class="placeholder-icon">
                        <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
                            <rect x="3" y="3" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
                            <path d="M9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2"/>
                        </svg>
                    </div>
                    <h3>{"生产管理模块"}</h3>
                    <p>{"该功能正在开发中，敬请期待..."}</p>
                </div>
            </div>
        },
        MenuItem::Inventory => html! {
            <div class="content-panel">
                <div class="panel-header">
                    <h2>{"库存管理"}</h2>
                    <p>{"监控原材料和成品库存"}</p>
                </div>
                <div class="content-placeholder">
                    <div class="placeholder-icon">
                        <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
                            <path d="M21 16V8a2 2 0 0 0-1-1.73L12 2L4 6.27A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73L12 22l8-4.27A2 2 0 0 0 21 16z" stroke="currentColor" stroke-width="2"/>
                        </svg>
                    </div>
                    <h3>{"库存管理模块"}</h3>
                    <p>{"该功能正在开发中，敬请期待..."}</p>
                </div>
            </div>
        },
        MenuItem::Quality => html! {
            <div class="content-panel">
                <div class="panel-header">
                    <h2>{"质量管理"}</h2>
                    <p>{"质量检测和数据分析"}</p>
                </div>
                <div class="content-placeholder">
                    <div class="placeholder-icon">
                        <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
                            <path d="M9 12l2 2 4-4" stroke="currentColor" stroke-width="2"/>
                            <circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2"/>
                        </svg>
                    </div>
                    <h3>{"质量管理模块"}</h3>
                    <p>{"该功能正在开发中，敬请期待..."}</p>
                </div>
            </div>
        },
        MenuItem::Settings => html! {
            <div class="content-panel">
                <div class="panel-header">
                    <h2>{"系统设置"}</h2>
                    <p>{"用户权限和系统配置"}</p>
                </div>
                <div class="content-placeholder">
                    <div class="placeholder-icon">
                        <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
                            <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2"/>
                            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" stroke="currentColor" stroke-width="2"/>
                        </svg>
                    </div>
                    <h3>{"系统设置模块"}</h3>
                    <p>{"该功能正在开发中，敬请期待..."}</p>
                </div>
            </div>
        },
    }
}
