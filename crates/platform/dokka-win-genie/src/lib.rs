//! Efecto genio aislado: foto, tiras y transiciones apagadas.
//!
//! Solo vive durante un vuelo. Si algo falla devuelve `None` y quien llama
//! sigue con minimizar o restaurar como siempre.

#![allow(unsafe_code)]

mod advance;
mod capture;
mod director;
mod flight;
mod geometry;
mod overlay;
mod paint;
mod snapshot;
mod strips;
mod transitions;

pub use capture::GenieImage;
pub use director::GenieDirector;
pub use flight::Flight;
pub use geometry::{dock_slot_to_screen, monitor_rect_for_window, primary_screen_rect};
pub use snapshot::Snapshot;
pub(crate) use strips::GenieCompositor;
pub use transitions::{animations_enabled, ensure_clean_startup, restore_all_transitions};
