//! Apariencia de los paneles flotantes.

use dokka_scene::Color;

/// Alto reservado sobre el dock para los paneles.
pub const SPACE: f32 = 250.0;
/// Separación entre un panel y los iconos agrandados.
pub const GAP: f32 = 10.0;
pub const CORNER_RADIUS: f32 = 16.0;
pub const TINT: Color = Color::rgba(236, 240, 236, 170);
pub const BORDER: Color = Color::rgba(255, 255, 255, 140);
pub const INK: Color = Color::rgba(30, 32, 36, 255);
pub const HIGHLIGHT: Color = Color::rgba(255, 255, 255, 245);

pub const TITLE_SIZE: f32 = 14.0;
pub const BODY_SIZE: f32 = 13.0;
pub const PADDING: f32 = 12.0;

pub const CALENDAR_CELL_WIDTH: f32 = 38.0;
pub const CALENDAR_CELL_HEIGHT: f32 = 29.0;
pub const CALENDAR_HEADER: f32 = 40.0;

/// Las vistas previas usan la misma apariencia clara que los demás paneles.
pub const PREVIEW_TINT: Color = TINT;
pub const PREVIEW_BORDER: Color = BORDER;
pub const PREVIEW_INK: Color = INK;
pub const PREVIEW_PADDING: f32 = 8.0;
pub const PREVIEW_SLOT_WIDTH: f32 = 270.0;
pub const PREVIEW_IMAGE_HEIGHT: f32 = 180.0;
pub const PREVIEW_TITLE_HEIGHT: f32 = 28.0;
pub const PREVIEW_MAXIMUM_WINDOWS: usize = 3;

/// Píldora de controles sobre la imagen.
pub const MEDIA_TINT: Color = Color::rgba(240, 244, 240, 225);
pub const MEDIA_BORDER: Color = BORDER;
pub const MEDIA_BUTTON_WIDTH: f32 = 52.0;
pub const MEDIA_HEIGHT: f32 = 42.0;
pub const MEDIA_ICON_SIDE: f32 = 22.0;
/// Distancia entre la píldora y el borde inferior de la imagen.
pub const MEDIA_INSET: f32 = 10.0;
