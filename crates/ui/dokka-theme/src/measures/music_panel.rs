//! Medidas del panel de música de la barra superior.
//!
//! Tarjeta clara como el calendario: título y artista arriba y barra de
//! botones abajo. El radio interior es el exterior menos la separación,
//! para que queden concéntricos.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo de la tarjeta.
pub const WIDTH: f32 = 280.0;
/// Alto fijo: cabecera de dos líneas más barra de botones.
pub const HEIGHT: f32 = 124.0;
/// Aire alrededor del contenido, como en los paneles flotantes.
pub const PADDING: f32 = 12.0;
/// Radio exterior, como el de los paneles flotantes.
pub const CORNER_RADIUS: f32 = 16.0;
pub const BORDER_WIDTH: f32 = 1.0;

/// Centro vertical del título desde el borde superior.
pub const TITLE_CENTER_OFFSET: f32 = 24.0;
/// Centro vertical del artista desde el borde superior.
pub const ARTIST_CENTER_OFFSET: f32 = 46.0;
/// Títulos largos recortados con "…".
pub const TITLE_CHARACTERS: usize = 30;
/// Artistas largos recortados con "…".
pub const ARTIST_CHARACTERS: usize = 34;
/// Texto cuando ninguna app informa de reproducción.
pub const EMPTY_TEXT: &str = "Nada en reproducción";

/// Alto de la barra de botones.
pub const BUTTON_BAR_HEIGHT: f32 = 44.0;
/// Radio de la barra: exterior menos la separación, concéntrico.
pub const BUTTON_BAR_RADIUS: f32 = CORNER_RADIUS - PADDING;
/// Lado del icono de cada botón, centrado en su tercio.
pub const BUTTON_ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100, fuente efectiva 20 a 20 de lado.
pub const BUTTON_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo de la nota musical de la barra, igual que los de la barra superior.
pub const BAR_GLYPH_SIZE: f32 = DRAWING_SIDE;
