//! Medidas del panel de música de la barra superior, estilo Tahoe.
//!
//! Una sola tarjeta de cristal ahumado: carátula cuadrada a la izquierda,
//! título y artista a su derecha y tres controles grandes centrados abajo.
//! Radios concéntricos: la carátula recorta el radio de la tarjeta
//! (exterior menos separación).

use dokka_scene::DRAWING_SIDE;

use super::tray_glass as glass;

/// Ancho de la tarjeta, como los demás paneles de la barra.
pub const WIDTH: f32 = glass::WIDTH;
/// Alto: aire + carátula + hueco + controles + aire.
pub const HEIGHT: f32 = glass::PADDING * 2.0 + ARTWORK_SIDE + glass::GAP + CONTROLS_HEIGHT;
/// Aire dentro de la tarjeta.
pub const PADDING: f32 = glass::PADDING;
/// Hueco entre la fila de carátula y la de controles.
pub const GAP: f32 = glass::GAP;
/// Esquinas de la tarjeta.
pub const TILE_RADIUS: f32 = glass::TILE_RADIUS;
pub const TILE_BORDER_WIDTH: f32 = glass::TILE_BORDER_WIDTH;

/// Lado de la carátula (marcador cuadrado redondeado).
pub const ARTWORK_SIDE: f32 = 56.0;
/// Radio de la carátula: exterior menos la separación, concéntrico.
pub const ARTWORK_RADIUS: f32 = TILE_RADIUS - PADDING;
/// Aire entre la carátula y los textos.
pub const TEXT_GAP: f32 = 10.0;
/// Título en negrita y artista debajo, blancos (el segundo atenuado).
pub const TITLE_SIZE: f32 = glass::TITLE_SIZE;
pub const SUBTITLE_SIZE: f32 = glass::SUBTITLE_SIZE;
/// Títulos largos recortados con "…".
pub const TITLE_CHARACTERS: usize = 24;
/// Artistas largos recortados con "…".
pub const ARTIST_CHARACTERS: usize = 26;
/// Texto cuando ninguna app informa de reproducción.
pub const EMPTY_TEXT: &str = "Nada en reproducción";

/// Alto de la fila de controles.
pub const CONTROLS_HEIGHT: f32 = 48.0;
/// Ancho de cada botón (anterior, reproducir/pausa, siguiente).
pub const BUTTON_WIDTH: f32 = 64.0;
/// Lado del icono de cada botón, centrado en su botón.
pub const BUTTON_ICON_SIDE: f32 = 28.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100, fuente efectiva 28 a 28 de lado.
pub const BUTTON_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo de la nota musical de la barra, igual que los de la barra superior.
pub const BAR_GLYPH_SIZE: f32 = DRAWING_SIDE;
