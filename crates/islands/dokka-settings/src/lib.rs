//! Isla de Preferencias: ventana de configuración estilo Tahoe.
mod controls;
mod drawings;
mod events;
mod general;
mod general_state;
mod logic;
mod menubar_drag;
mod motion;
mod navigation;
mod painted;
mod pointer;
mod reorder_pointer;
mod selection;
mod slider_pointer;
mod state;
mod values;
mod view;

pub use events::{AboutButtonAction, SettingsAction};
pub use general::{AboutButton, GeneralPage};
pub use logic::{SECTION_NAMES, drag_zone};
pub use painted::{OrderedF32, PaintedControl, PaintedSettings};
pub use state::Settings;
pub use view::settings_scene;
