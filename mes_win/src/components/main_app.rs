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

// 一级菜单项枚举
#[derive(Clone, PartialEq, Copy)]
pub enum PrimaryMenuItem {
    Dashboard,
    Production,
    Inventory,
    Quality,
    Settings,
}

// 二级菜单项枚举
#[derive(Clone, PartialEq, Copy)]
pub enum SecondaryMenuItem {
    // 仪表板子菜单
    Overview,
    Analytics,
    Reports,
    // 生产管理子菜单
    Orders,
    Schedule,
    Workflow,
    // 库存管理子菜单
    Materials,
    Products,
    Warehouse,
    // 质量管理子菜单
    Inspection,
    Standards,
    Issues,
    // 系统管理子菜单
    Users,
    Permissions,
    System,
}

// 主界面组件
#[function_component(MainApp)]
pub fn main_app(props: &MainAppProps) -> Html {
    let selected_primary_menu = use_state(|| PrimaryMenuItem::Dashboard);
    let selected_secondary_menu = use_state(|| SecondaryMenuItem::Overview);
    let search_term = use_state(|| String::new());

    // 处理一级菜单选择
    let on_primary_menu_select = {
        let selected_primary_menu = selected_primary_menu.clone();
        let selected_secondary_menu = selected_secondary_menu.clone();
        Callback::from(move |item: PrimaryMenuItem| {
            selected_primary_menu.set(item.clone());
            // 根据一级菜单设置默认的二级菜单
            let default_secondary = match item {
                PrimaryMenuItem::Dashboard => SecondaryMenuItem::Overview,
                PrimaryMenuItem::Production => SecondaryMenuItem::Orders,
                PrimaryMenuItem::Inventory => SecondaryMenuItem::Materials,
                PrimaryMenuItem::Quality => SecondaryMenuItem::Inspection,
                PrimaryMenuItem::Settings => SecondaryMenuItem::Users,
            };
            selected_secondary_menu.set(default_secondary);
        })
    };

    // 处理二级菜单选择
    let on_secondary_menu_select = {
        let selected_secondary_menu = selected_secondary_menu.clone();
        Callback::from(move |item: SecondaryMenuItem| {
            selected_secondary_menu.set(item);
        })
    };

    // 处理搜索
    let on_search_change = {
        let search_term = search_term.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_term.set(input.value());
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

            // 三栏主要内容区域
            <main class="app-main">
                // 左侧一级菜单栏
                <aside class="sidebar-primary">
                    // 用户头像区域
                    <div class="user-section">
                        <div class="user-avatar">
                            <img src="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHZpZXdCb3g9IjAgMCA0MCA0MCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMjAiIGN5PSIyMCIgcj0iMjAiIGZpbGw9IiM0Qzc2RjEiLz4KPGV4dCB4PSIyMCIgeT0iMjQiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0id2hpdGUiIHRleHQtYW5jaG9yPSJtaWRkbGUiPnsocHJvcHMudXNlcm5hbWUuY2hhcnNfYXQoMCldfTwvdGV4dD4KPC9zdmc+" alt={props.username.clone()} />
                        </div>
                        <div class="username-tooltip">{&props.username}</div>
                    </div>
                    
                    // 一级菜单
                    <nav class="primary-menu">
                        <div class={if *selected_primary_menu == PrimaryMenuItem::Dashboard { "menu-item active" } else { "menu-item" }}
                             onclick={
                                 let on_primary_menu_select = on_primary_menu_select.clone();
                                 Callback::from(move |_| on_primary_menu_select.emit(PrimaryMenuItem::Dashboard))
                             }>
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <rect x="3" y="3" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                                <rect x="14" y="3" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                                <rect x="14" y="14" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                                <rect x="3" y="14" width="7" height="7" rx="1" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            <span class="menu-tooltip">{"仪表板"}</span>
                        </div>
                        
                        <div class={if *selected_primary_menu == PrimaryMenuItem::Production { "menu-item active" } else { "menu-item" }}
                             onclick={
                                 let on_primary_menu_select = on_primary_menu_select.clone();
                                 Callback::from(move |_| on_primary_menu_select.emit(PrimaryMenuItem::Production))
                             }>
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M9 7H6a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3" stroke="currentColor" stroke-width="2"/>
                                <rect x="9" y="1" width="6" height="6" rx="2" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            <span class="menu-tooltip">{"生产管理"}</span>
                        </div>
                        
                        <div class={if *selected_primary_menu == PrimaryMenuItem::Inventory { "menu-item active" } else { "menu-item" }}
                             onclick={
                                 let on_primary_menu_select = on_primary_menu_select.clone();
                                 Callback::from(move |_| on_primary_menu_select.emit(PrimaryMenuItem::Inventory))
                             }>
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M21 16V8a2 2 0 0 0-1-1.73L12 2L4 6.27A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73L12 22l8-4.27A2 2 0 0 0 21 16z" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            <span class="menu-tooltip">{"库存管理"}</span>
                        </div>
                        
                        <div class={if *selected_primary_menu == PrimaryMenuItem::Quality { "menu-item active" } else { "menu-item" }}
                             onclick={
                                 let on_primary_menu_select = on_primary_menu_select.clone();
                                 Callback::from(move |_| on_primary_menu_select.emit(PrimaryMenuItem::Quality))
                             }>
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M9 12l2 2 4-4" stroke="currentColor" stroke-width="2"/>
                                <path d="M21 12c-1 0-3-1-3-3s2-3 3-3 3 1 3 3-2 3-3 3" stroke="currentColor" stroke-width="2"/>
                                <path d="M3 12c1 0 3-1 3-3s-2-3-3-3-3 1-3 3 2 3 3 3" stroke="currentColor" stroke-width="2"/>
                                <path d="M12 21c0-1-1-3-3-3s-3 2-3 3 1 3 3 3 3-2 3-3" stroke="currentColor" stroke-width="2"/>
                                <path d="M12 3c0 1-1 3-3 3s-3-2-3-3 1-3 3-3 3 2 3 3" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            <span class="menu-tooltip">{"质量管理"}</span>
                        </div>
                        
                        <div class={if *selected_primary_menu == PrimaryMenuItem::Settings { "menu-item active" } else { "menu-item" }}
                             onclick={
                                 let on_primary_menu_select = on_primary_menu_select.clone();
                                 Callback::from(move |_| on_primary_menu_select.emit(PrimaryMenuItem::Settings))
                             }>
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" stroke="currentColor" stroke-width="2"/>
                                <path d="M9 9h6v6H9z" stroke="currentColor" stroke-width="2"/>
                                <path d="M9 3v6" stroke="currentColor" stroke-width="2"/>
                                <path d="M15 3v6" stroke="currentColor" stroke-width="2"/>
                                <path d="M9 15v6" stroke="currentColor" stroke-width="2"/>
                                <path d="M15 15v6" stroke="currentColor" stroke-width="2"/>
                                <path d="M3 9h6" stroke="currentColor" stroke-width="2"/>
                                <path d="M15 9h6" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            <span class="menu-tooltip">{"系统管理"}</span>
                        </div>
                    </nav>
                    
                    // 主题切换按钮
                    <button class="theme-toggle" onclick={on_theme_toggle} title="切换主题">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="sun-icon">
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
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="moon-icon" style="display: none;">
                            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                        </svg>
                    </button>
                </aside>

                // 中间二级菜单栏
                <aside class="sidebar-secondary">
                    // 搜索框
                    <div class="search-container">
                        <div class="search-box">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" class="search-icon">
                                <circle cx="11" cy="11" r="8" stroke="currentColor" stroke-width="2"/>
                                <path d="M21 21l-4.35-4.35" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            <input 
                                type="text" 
                                placeholder="搜索功能..." 
                                value={(*search_term).clone()}
                                onchange={on_search_change}
                            />
                        </div>
                    </div>
                    
                    // 二级菜单
                    <nav class="secondary-menu">
                        { render_secondary_menu(&selected_primary_menu, &selected_secondary_menu, &on_secondary_menu_select) }
                    </nav>
                </aside>

                // 右侧内容区域
                <section class="content-area">
                    { render_content(&selected_primary_menu, &selected_secondary_menu) }
                </section>
            </main>
        </div>
    }
}

// 渲染二级菜单
fn render_secondary_menu(
    primary_menu: &UseStateHandle<PrimaryMenuItem>,
    selected_secondary_menu: &UseStateHandle<SecondaryMenuItem>,
    on_secondary_menu_select: &Callback<SecondaryMenuItem>,
) -> Html {
    let secondary_items = match **primary_menu {
        PrimaryMenuItem::Dashboard => vec![
            (SecondaryMenuItem::Overview, "概览", "总体数据统计"),
            (SecondaryMenuItem::Analytics, "分析", "数据分析报告"),
            (SecondaryMenuItem::Reports, "报表", "生成各类报表"),
        ],
        PrimaryMenuItem::Production => vec![
            (SecondaryMenuItem::Orders, "订单", "生产订单管理"),
            (SecondaryMenuItem::Schedule, "排程", "生产计划排程"),
            (SecondaryMenuItem::Workflow, "工艺", "生产工艺流程"),
        ],
        PrimaryMenuItem::Inventory => vec![
            (SecondaryMenuItem::Materials, "物料", "原材料管理"),
            (SecondaryMenuItem::Products, "产品", "成品库存管理"),
            (SecondaryMenuItem::Warehouse, "仓库", "仓储管理"),
        ],
        PrimaryMenuItem::Quality => vec![
            (SecondaryMenuItem::Inspection, "检验", "质量检验记录"),
            (SecondaryMenuItem::Standards, "标准", "质量标准管理"),
            (SecondaryMenuItem::Issues, "问题", "质量问题跟踪"),
        ],
        PrimaryMenuItem::Settings => vec![
            (SecondaryMenuItem::Users, "用户", "用户账户管理"),
            (SecondaryMenuItem::Permissions, "权限", "权限角色管理"),
            (SecondaryMenuItem::System, "系统", "系统参数设置"),
        ],
    };

    html! {
        <div class="secondary-menu-list">
            { for secondary_items.iter().map(|(item, title, description)| {
                let item_clone = item.clone();
                let on_secondary_menu_select = on_secondary_menu_select.clone();
                html! {
                    <div class={if **selected_secondary_menu == *item { "secondary-menu-item active" } else { "secondary-menu-item" }}
                         onclick={
                             Callback::from(move |_| on_secondary_menu_select.emit(item_clone.clone()))
                         }>
                        <div class="secondary-menu-title">{title}</div>
                        <div class="secondary-menu-description">{description}</div>
                    </div>
                }
            })}
        </div>
    }
}

// 渲染不同内容区域
fn render_content(
    selected_primary_menu: &UseStateHandle<PrimaryMenuItem>,
    selected_secondary_menu: &UseStateHandle<SecondaryMenuItem>,
) -> Html {
    match (**selected_primary_menu, **selected_secondary_menu) {
        (PrimaryMenuItem::Dashboard, SecondaryMenuItem::Overview) => html! {
            <div class="content-panel">
                <div class="panel-header">
                    <h2>{"仪表板 - 概览"}</h2>
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
                </div>
            </div>
        },
        _ => html! {
            <div class="content-panel">
                <div class="panel-header">
                    <h2>{"功能开发中"}</h2>
                    <p>{"该功能模块正在开发中"}</p>
                </div>
                <div class="content-placeholder">
                    <div class="placeholder-icon">
                        <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
                            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
                            <path d="M12 6v6l4 2" stroke="currentColor" stroke-width="2"/>
                        </svg>
                    </div>
                    <h3>{"敬请期待"}</h3>
                    <p>{"更多功能正在紧张开发中..."}</p>
                </div>
            </div>
        },
    }
}
