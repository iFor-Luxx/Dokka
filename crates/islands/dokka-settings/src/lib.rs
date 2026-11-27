//! Isla de Preferencias: ventana de configuración estilo Tahoe.
mod controls;
mod drawings;
mod events;
mod logic;
mod motion;
mod painted;
mod pointer;
mod state;
mod values;
mod view;

pub use events::SettingsAction;
pub use logic::{SECTION_NAMES, drag_zone};
pub use painted::{OrderedF32, PaintedControl, PaintedSettings};
pub use state::Settings;
pub use view::settings_scene;
