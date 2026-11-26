//! Apariencia de las etiquetas flotantes.

use dokka_scene::Color;

pub const FONT_SIZE: f32 = 13.0;
pub const PADDING_X: f32 = 12.0;
pub const PADDING_Y: f32 = 6.0;
pub const CORNER_RADIUS: f32 = 8.0;
/// Alto de la punta que señala al elemento.
pub const POINTER_HEIGHT: f32 = 6.0;
/// Separación entre la punta y el elemento.
pub const GAP: f32 = 4.0;
pub const BACKGROUND: Color = Color::rgba(232, 238, 232, 235);
pub const TEXT: Color = Color::rgba(30, 32, 36, 255);
/// Los textos más largos se recortan.
pub const MAXIMUM_CHARACTERS: usize = 30;
