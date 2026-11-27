//! Medidas de las etiquetas flotantes: píldora de cristal con punta.

pub const PADDING_X: f32 = 18.0;
pub const PADDING_Y: f32 = 7.0;
/// Alto de la punta que señala al elemento.
pub const POINTER_HEIGHT: f32 = 8.0;
/// Radio del vértice de la punta.
pub const POINTER_RADIUS: f32 = 2.0;
pub const BORDER_WIDTH: f32 = 1.0;
/// Separación entre la punta y el elemento.
pub const GAP: f32 = 4.0;
/// Margen que se reserva para la etiqueta sobre un icono.
pub const RESERVED_HEIGHT: f32 = 40.0;
/// Ancho medio de un carácter a `TOOLTIP_SIZE`, con margen para los anchos.
const AVERAGE_CHARACTER_WIDTH: f32 = 8.5;
/// Ancho que se reserva para una etiqueta lateral: el texto más largo
/// (`MAXIMUM_CHARACTERS` a `TOOLTIP_SIZE`) con su relleno, punta y separación.
/// La isla no mide texto; la plataforma lo mide al pintar.
pub const RESERVED_WIDTH: f32 = MAXIMUM_CHARACTERS as f32 * AVERAGE_CHARACTER_WIDTH
    + PADDING_X * 2.0
    + POINTER_HEIGHT
    + GAP
    + BORDER_WIDTH * 2.0;
/// Los textos más largos se recortan.
pub const MAXIMUM_CHARACTERS: usize = 40;
