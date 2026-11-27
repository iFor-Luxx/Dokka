//! Efecto genio aislado: imagen viva, tiras y transiciones apagadas.
//!
//! Solo vive durante un vuelo. Si algo falla devuelve `None` y quien llama
//! sigue con minimizar o restaurar como siempre.

#![allow(unsafe_code)]

mod advance;
mod departure;
mod director;
mod director_advance;
mod director_support;
mod flight;
mod flight_restore;
mod fullscreen;
mod geometry;
mod origin;
mod overlay;
mod phase;
mod scene;
mod strips;
mod transitions;

pub use director::GenieDirector;
pub use geometry::{dock_slot_to_screen, monitor_rect_for_window};
pub use origin::rect_before_minimize;
pub use transitions::ensure_clean_startup;
