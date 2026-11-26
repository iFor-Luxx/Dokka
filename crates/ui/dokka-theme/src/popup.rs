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

pub const PREVIEW_SLOT_WIDTH: f32 = 200.0;
pub const PREVIEW_IMAGE_HEIGHT: f32 = 124.0;
pub const PREVIEW_TITLE_HEIGHT: f32 = 28.0;
pub const PREVIEW_MAXIMUM_WINDOWS: usize = 3;

/// Alto de la canción más sus botones.
pub const MEDIA_HEIGHT: f32 = 58.0;
pub const MEDIA_BUTTON_WIDTH: f32 = 46.0;
pub const MEDIA_BUTTON_HEIGHT: f32 = 30.0;
pub const MEDIA_SYMBOL_SIZE: f32 = 18.0;
