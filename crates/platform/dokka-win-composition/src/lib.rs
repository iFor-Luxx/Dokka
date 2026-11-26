//! Dibujo de escenas con el compositor de Windows.
#![allow(unsafe_code)]

mod composition_error;
mod draw_glass;
mod draw_icon;
mod draw_tint;
mod graphics_device;
mod paint_bitmap;
mod scene_renderer;
mod windows_color;

pub use composition_error::CompositionError;
pub use scene_renderer::SceneRenderer;
