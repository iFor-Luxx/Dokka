//! Tipos del dominio de Dokka, sin dependencias de plataforma.

mod app_id;
mod app_info;
mod app_window;
mod dock_app;
mod open_window;
mod window_id;

pub use app_id::AppId;
pub use app_info::AppInfo;
pub use app_window::AppWindow;
pub use dock_app::DockApp;
pub use open_window::OpenWindow;
pub use window_id::WindowId;
