//! Dibujo de escenas con el compositor de Windows.
#![allow(unsafe_code)]

mod composition_error;
mod dot_layer;
mod graphics_device;
mod icon_layer;
mod paint_bitmap;
mod panel_layer;
mod scene_layers;
mod scene_renderer;
mod windows_color;

pub use composition_error::CompositionError;
pub use scene_renderer::SceneRenderer;
