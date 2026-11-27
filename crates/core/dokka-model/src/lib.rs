//! Tipos del dominio de Dokka, sin dependencias de plataforma.

mod app_id;
mod app_info;
mod app_window;
mod dock_app;
mod open_window;
mod preferences;
mod system_status;
mod tray_item;
mod window_id;

pub use app_id::AppId;
pub use app_info::AppInfo;
pub use app_window::AppWindow;
pub use dock_app::DockApp;
pub use open_window::OpenWindow;
pub use preferences::{
    DOCK_ICON_SCALE_RANGE, DOCK_MAGNIFICATION_RANGE, DockPosition, DockPreferences, MenubarItem,
    MenubarPreferences, Preferences,
};
pub use system_status::{
    AudioOutput, KeyboardLanguage, NetworkKind, NetworkState, NowPlayingInfo, VolumeState,
};
pub use tray_item::{TrayItem, TrayItemId};
pub use window_id::WindowId;
