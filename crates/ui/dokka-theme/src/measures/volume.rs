//! Medidas del panel de volumen y del deslizador reutilizable.
//!
//! Panel claro como el calendario, con título, fila de deslizador
//! (altavoz, barra y silencio), separador y fila de ajustes.
//! El deslizador sirve también para Preferencias: solo usa grises
//! del tema y el azul de selección.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo del panel.
pub const PANEL_WIDTH: f32 = 280.0;
/// Aire alrededor del contenido, dentro del panel.
pub const PADDING: f32 = 12.0;
/// Esquinas del panel, como las flotantes.
pub const CORNER_RADIUS: f32 = 16.0;
pub const BORDER_WIDTH: f32 = 1.0;

/// Alto de la fila del título.
pub const TITLE_HEIGHT: f32 = 24.0;
/// Separación entre el título y la fila del deslizador.
pub const TITLE_GAP: f32 = 4.0;

/// Alto de la fila con el altavoz, la barra y el botón de silencio.
pub const SLIDER_HEIGHT: f32 = 32.0;
/// Lado del icono del altavoz, a la izquierda de la barra.
pub const SPEAKER_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide del altavoz: llena el lienzo de 100.
pub const SPEAKER_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Lado del botón cuadrado de silencio, a la derecha de la barra.
pub const MUTE_SIDE: f32 = 28.0;
/// Lado del icono dentro del botón de silencio.
pub const MUTE_ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide del botón de silencio.
pub const MUTE_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Separación del altavoz y del botón respecto a la barra.
pub const ICON_GAP: f32 = 8.0;

/// Grosor de la barra y de su relleno.
pub const TRACK_HEIGHT: f32 = 6.0;
/// Radio de la barra y del relleno: extremos totalmente redondos.
pub const TRACK_RADIUS: f32 = TRACK_HEIGHT / 2.0;
/// Diámetro del tirador que marca el valor sobre la barra.
pub const KNOB_DIAMETER: f32 = 14.0;

/// Separación entre la fila del deslizador y el separador.
pub const SLIDER_GAP: f32 = 8.0;
/// Hueco vertical que ocupa la línea separadora.
pub const SEPARATOR_HEIGHT: f32 = 11.0;
pub const DIVIDER_HEIGHT: f32 = 1.0;

/// Alto de la fila de ajustes.
pub const SETTINGS_HEIGHT: f32 = 28.0;
/// Radio del resaltado suave del botón y de la fila de ajustes.
pub const HIGHLIGHT_RADIUS: f32 = 8.0;
/// Lado del chevrón de la fila de ajustes.
pub const SETTINGS_CHEVRON_SIDE: f32 = 16.0;
/// Tamaño del glifo Lucide del chevrón.
pub const SETTINGS_GLYPH_SIZE: f32 = DRAWING_SIDE;

/// Aire bajo la fila de ajustes, dentro del panel.
pub const BOTTOM_PADDING: f32 = 12.0;

/// Título del panel.
pub const TITLE: &str = "Sonido";
/// Fila que abre los ajustes.
pub const SETTINGS_LABEL: &str = "Configuración de sonido…";

/// Alto del panel: suma de filas, separador y aires.
pub fn panel_height() -> f32 {
    PADDING
        + TITLE_HEIGHT
        + TITLE_GAP
        + SLIDER_HEIGHT
        + SLIDER_GAP
        + SEPARATOR_HEIGHT
        + SETTINGS_HEIGHT
        + BOTTOM_PADDING
}
