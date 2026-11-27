//! Medidas del panel de volumen estilo Tahoe: dos piezas de cristal
//! separadas (tarjeta "Sonido" con deslizador y tarjeta con la fila de
//! ajustes). El deslizador es el componente `slider` reutilizable (luego lo
//! usará Preferencias): solo recibe sus colores.
//!
//! Todo sale de `tray_glass` (ancho, hueco, aire, radio y borde de cada
//! pieza, grosor de la barra y tamaños de texto). Radios concéntricos:
//! interior = exterior - separación.

use dokka_scene::DRAWING_SIDE;

use super::tray_glass;

/// Ancho fijo del panel (el de las piezas Tahoe).
pub const PANEL_WIDTH: f32 = tray_glass::WIDTH;
/// Aire dentro de cada pieza.
pub const PADDING: f32 = tray_glass::PADDING;
/// Hueco entre las dos piezas.
pub const GAP: f32 = tray_glass::GAP;
/// Esquinas y borde de cada pieza de cristal.
pub const TILE_RADIUS: f32 = tray_glass::TILE_RADIUS;
pub const TILE_BORDER_WIDTH: f32 = tray_glass::TILE_BORDER_WIDTH;

/// Alto de la fila del título dentro de la tarjeta de sonido.
pub const TITLE_HEIGHT: f32 = 20.0;
/// Separación entre el título y la fila del deslizador.
pub const TITLE_GAP: f32 = 6.0;

/// Alto del área de pulsación del deslizador: cómoda, más alta que la barra.
pub const SLIDER_ROW_HEIGHT: f32 = 28.0;
/// Grosor de la barra (el de las piezas Tahoe).
pub const TRACK_HEIGHT: f32 = tray_glass::TRACK_HEIGHT;
/// Lado del glifo del altavoz dentro de su hueco.
pub const ICON_SIDE: f32 = 16.0;
/// Lado del hueco pulsable del altavoz de la izquierda (silenciar).
pub const MUTE_HIT_SIDE: f32 = 28.0;
/// Separación del altavoz y del icono derecho respecto a la barra.
pub const ICON_GAP: f32 = 8.0;

/// Tamaño del glifo Lucide del altavoz: llena el lienzo de 100.
pub const SPEAKER_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo Lucide del botón de silencio anterior: se conserva para
/// no romper usos externos; el panel Tahoe usa el altavoz como botón.
pub const MUTE_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo Lucide del chevrón de ajustes.
pub const SETTINGS_GLYPH_SIZE: f32 = DRAWING_SIDE;

/// Alto de la tarjeta con la fila de ajustes.
pub const SETTINGS_TILE_HEIGHT: f32 = 44.0;
/// Lado del chevrón de la fila de ajustes.
pub const SETTINGS_CHEVRON_SIDE: f32 = 16.0;

/// Título de la tarjeta de sonido.
pub const TITLE: &str = "Sonido";
/// Fila que abre los ajustes.
pub const SETTINGS_LABEL: &str = "Configuración de sonido…";

/// Tamaños de texto de las piezas Tahoe.
pub const TITLE_SIZE: f32 = tray_glass::TITLE_SIZE;

/// Alto de la tarjeta de sonido: título, deslizador y aires.
pub fn sound_tile_height() -> f32 {
    PADDING + TITLE_HEIGHT + TITLE_GAP + SLIDER_ROW_HEIGHT + PADDING
}

/// Alto del panel: tarjeta de sonido, hueco y tarjeta de ajustes.
pub fn panel_height() -> f32 {
    sound_tile_height() + GAP + SETTINGS_TILE_HEIGHT
}
