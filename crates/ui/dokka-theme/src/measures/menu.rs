//! Medidas del menú del clic derecho.

/// Ancho fijo del menú.
pub const WIDTH: f32 = 250.0;
/// Alto de cada fila de texto.
pub const ROW_HEIGHT: f32 = 24.0;
/// Aire alrededor de las filas, dentro del panel.
pub const PADDING: f32 = 5.0;
/// Separación del texto respecto al borde izquierdo del resaltado.
pub const TEXT_INSET: f32 = 10.0;
/// Columna reservada a la izquierda para la marca en los menús que la usan.
pub const CHECK_WIDTH: f32 = 18.0;
/// Lado del icono de marca, centrado en su columna como el chevrón.
pub const CHECK_SIDE: f32 = 12.0;
/// Columna reservada a la derecha para el icono del chevrón en las filas con submenú.
/// El icono va centrado en esta columna.
pub const ARROW_WIDTH: f32 = 18.0;
/// Lado del icono del chevrón, centrado en la fila y en la columna del chevrón.
pub const CHEVRON_SIDE: f32 = 12.0;
/// Tamaño del glifo Lucide dentro del lienzo de 100 (centrado en 50, 50).
/// Se ajusta para que el trazo aparente iguale al chevrón anterior.
pub const CHEVRON_GLYPH_SIZE: f32 = 88.0;
/// Tamaño del glifo de la marca dentro del lienzo de 100.
pub const CHECK_GLYPH_SIZE: f32 = 88.0;
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
