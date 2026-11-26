//! Dibujo de escenas con el compositor de Windows.
#![allow(unsafe_code)]

mod composition_error;
mod controls_layer;
mod divider_layer;
mod dot_layer;
mod fill_triangle;
mod fit_thumbnail;
mod graphics_device;
mod icon_layer;
mod label_layer;
mod overlay_layers;
mod paint_bitmap;
mod paint_drawing;
mod paint_label;
mod paint_surface;
mod paint_text;
mod painter;
mod panel_layer;
mod scene_layers;
mod scene_renderer;
mod solid_brush;
mod text_layer;
mod window_thumbnails;
mod windows_color;

pub use composition_error::CompositionError;
pub use scene_renderer::SceneRenderer;
