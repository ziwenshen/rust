pub mod components;
pub mod pages;

// 重新导出主要的UI组件
pub use pages::login::Login;
pub use pages::main_app::MainApp;
pub use pages::profile::ProfilePanel;
