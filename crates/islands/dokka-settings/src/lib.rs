//! Isla de Preferencias: ventana de configuración estilo Tahoe, solo diseño.
mod drawings;
mod events;
mod logic;
mod motion;
mod painted;
mod state;
mod view;

pub use events::SettingsAction;
pub use logic::{SECTION_NAMES, drag_zone};
pub use painted::PaintedSettings;
pub use state::Settings;
pub use view::settings_scene;
