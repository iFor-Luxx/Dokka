//! Medidas de la fila de iconos de la derecha, a la izquierda de la fecha.
//! De derecha a izquierda: centro de control, buscar, idioma (texto),
//! música, red y volumen. Cada icono es un botón con hover plano suave;
//! cada panel real mide lo suyo y la ventana cubre al más alto.

use dokka_scene::DRAWING_SIDE;

/// Lado del glifo de cada icono, igual que el de Dokka.
pub const ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100, fuente efectiva 20.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Ancho del hueco pulsable de cada icono.
pub const CELL: f32 = 30.0;
/// Ancho del hueco del idioma (texto de código como "ES").
pub const LANG_WIDTH: f32 = 46.0;
/// Separación entre la fila y el hueco de la fecha.
pub const CLOCK_GAP: f32 = 8.0;
/// Radio del hover plano, como el del icono de Dokka.
pub const HOVER_RADIUS: f32 = 8.0;
/// Aire del hover alrededor del icono.
pub const HOVER_PADDING: f32 = 4.0;
