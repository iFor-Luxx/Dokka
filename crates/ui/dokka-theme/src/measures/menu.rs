//! Medidas del menú del clic derecho.
//!
//! Una sola columna a la izquierda para el icono de la fila o la marca
//! de opción elegida: todos los textos quedan alineados. El chevrón
//! va alineado a la derecha, con el mismo aire que el icono.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo del menú: algo más ancho para que quepan los textos
/// con la columna de iconos.
pub const WIDTH: f32 = 280.0;
/// Alto de cada fila de texto.
pub const ROW_HEIGHT: f32 = 26.0;
/// Aire alrededor de las filas, dentro del panel.
pub const PADDING: f32 = 5.0;
/// Separación del separador respecto al borde del resaltado, a cada lado.
pub const TEXT_INSET: f32 = 10.0;
/// Columna reservada a la izquierda para el icono o la marca:
/// del borde del resaltado al texto. Misma que la derecha del chevrón.
pub const ICON_COLUMN: f32 = 26.0;
/// Lado del icono de la fila, centrado en su columna.
pub const ROW_ICON_SIDE: f32 = 15.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100 (centrado en 50, 50),
/// de modo que la fuente efectiva iguala al lado con que se muestra.
pub const ROW_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Lado de la marca de opción elegida, centrada en la misma columna.
pub const CHECK_SIDE: f32 = 15.0;
/// Columna reservada a la derecha para el chevrón de las filas con submenú.
/// Igual que la de la izquierda, para el mismo aire a ambos lados.
pub const ARROW_WIDTH: f32 = 26.0;
/// Lado del chevrón, centrado en la fila y en su columna.
pub const CHEVRON_SIDE: f32 = 15.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100 (centrado en 50, 50),
/// de modo que la fuente efectiva es 20 al mostrarse a 20 de lado.
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
