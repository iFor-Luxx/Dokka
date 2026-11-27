//! Medidas de la fila de iconos de la derecha, a la izquierda de la fecha.
//! De derecha a izquierda: centro de control, buscar, idioma (texto),
//! música, red y volumen. Cada icono es un botón con hover plano suave;
//! cada panel real mide lo suyo y la ventana cubre al más alto.

use dokka_scene::DRAWING_SIDE;

/// Lado del glifo de cada icono, igual que el de Dokka y parejo en toda la barra.
pub const ICON_SIDE: f32 = 16.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100, fuente efectiva 20.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Ancho del hueco pulsable de cada icono.
pub const CELL: f32 = 34.0;
/// El centro de control lleva más aire: es el que más se usa.
pub const CONTROL_CELL: f32 = 44.0;
/// Ancho del hueco del idioma (texto de código como "ES").
pub const LANG_WIDTH: f32 = 46.0;
/// Separación entre la fila y el hueco de la fecha.
pub const CLOCK_GAP: f32 = 12.0;
/// Radio del hover plano, como el del icono de Dokka.
pub const HOVER_RADIUS: f32 = 8.0;
/// Aire del hover alrededor del icono.
pub const HOVER_PADDING: f32 = 4.0;
/// Recuadro del código de idioma, como una tecla.
pub const LANG_BOX_WIDTH: f32 = 22.0;
pub const LANG_BOX_HEIGHT: f32 = 17.0;
pub const LANG_BOX_RADIUS: f32 = 4.0;
pub const LANG_TEXT_SIZE: f32 = 11.0;
/// Interruptores apilados del centro de control: tamaño y centro de cada uno
/// en el lienzo de 100.
pub const TOGGLE_GLYPH_SIZE: f32 = 82.0;
pub const TOGGLE_TOP_Y: f32 = 27.0;
pub const TOGGLE_BOTTOM_Y: f32 = 73.0;
