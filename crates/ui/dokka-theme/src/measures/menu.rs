//! Medidas del menú del clic derecho.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo del menú.
pub const WIDTH: f32 = 250.0;
/// Alto de cada fila de texto.
pub const ROW_HEIGHT: f32 = 24.0;
/// Aire alrededor de las filas, dentro del panel.
pub const PADDING: f32 = 5.0;
/// Separación del texto respecto al borde izquierdo del resaltado.
pub const TEXT_INSET: f32 = 10.0;
/// Columna reservada a la izquierda para la marca en los menús que la usan.
/// El icono de 24 la llena y el texto empieza justo donde acaba.
pub const CHECK_WIDTH: f32 = 24.0;
/// Lado del icono de marca, centrado en su columna como el chevrón.
pub const CHECK_SIDE: f32 = 24.0;
/// Columna reservada a la derecha para el icono del chevrón en las filas con submenú.
/// El icono de 24 la llena; va centrado en la fila.
pub const ARROW_WIDTH: f32 = 24.0;
/// Lado del icono del chevrón, centrado en la fila y en la columna del chevrón.
pub const CHEVRON_SIDE: f32 = 24.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100 (centrado en 50, 50),
/// de modo que la fuente efectiva es 24 al mostrarse a 24 de lado.
pub const CHEVRON_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo de la marca: llena el lienzo, igual que el chevrón.
pub const CHECK_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Separación entre el menú madre y su submenú.
pub const SUBMENU_GAP: f32 = 4.0;
pub const CORNER_RADIUS: f32 = 10.0;
pub const HIGHLIGHT_RADIUS: f32 = 5.0;
pub const BORDER_WIDTH: f32 = 1.0;
/// Hueco vertical que ocupa una línea separadora.
pub const SEPARATOR_HEIGHT: f32 = 11.0;
pub const DIVIDER_HEIGHT: f32 = 1.0;
/// Punta que apunta al sitio pulsado: centro x y altura (base = 2 × altura).
pub const POINTER_HEIGHT: f32 = 9.0;
