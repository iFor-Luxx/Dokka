//! Dibujo de escenas con el compositor de Windows.
#![allow(unsafe_code)]

mod blur_effect;
mod blur_stage;
mod composition_error;
mod controls_layer;
mod cursor_layer;
mod divider_layer;
mod dot_layer;
mod fill_triangle;
mod fit_thumbnail;
mod graphics_device;
mod icon_layer;
mod label_layer;
mod label_pointer;
mod large_preview;
mod layer_sync;
mod overlay_layers;
mod overlay_dismiss;
mod overlay_style;
mod paint_bitmap;
mod paint_drawing;
mod paint_surface;
mod paint_text;
mod painter;
mod panel_layer;
mod pointer_layer;
mod scene_layers;
mod scene_renderer;
mod shadow_geometry;
mod shadow_layer;
mod solid_brush;
mod text_layer;
mod tile_brushes;
mod window_thumbnails;
mod windows_color;

pub use composition_error::CompositionError;
pub use scene_renderer::{SceneRenderer, WindowHandles};
