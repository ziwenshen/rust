mod app;
mod ui;         // 新的UI模块
mod auth;       // 认证模块
mod core;       // 核心模块
mod services;   // 服务模块
mod utils;      // 工具模块

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
