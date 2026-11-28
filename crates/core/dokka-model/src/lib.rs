//! Tipos del dominio de Dokka, sin dependencias de plataforma.

mod app_id;
mod app_info;
mod app_window;
mod control_item;
mod dock_app;
mod open_window;
mod preferences;
mod system_info;
mod system_status;
mod tray_item;
mod window_id;

pub use app_id::AppId;
pub use app_info::AppInfo;
pub use app_window::AppWindow;
pub use control_item::{ControlItem, HiddenControls};
pub use dock_app::DockApp;
pub use open_window::OpenWindow;
pub use preferences::{
    DOCK_ICON_SCALE_RANGE, DOCK_MAGNIFICATION_RANGE, DockPosition, DockPreferences,
    GeneralPreferences, MenubarItem, MenubarPreferences, Preferences,
};
pub use system_info::{DateTimeSummary, LocaleSummary, SystemSummary, bytes_in_gb};
pub use system_status::{
    AudioOutput, KeyboardLanguage, NetworkKind, NetworkState, NowPlayingInfo, VolumeState,
};
pub use tray_item::{TrayItem, TrayItemId};
pub use window_id::WindowId;
