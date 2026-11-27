//! Medidas del panel de volumen estilo Tahoe: un solo cristal oscuro con
//! título, cápsula ancha del deslizador, silenciar y ajustes separados
//! por líneas finas. El deslizador es el componente `slider` reutilizable:
//! solo recibe sus colores.
//!
//! Todo sale de `tray_glass` (ancho, aire, radio del cristal, filas y
//! cápsula). Radios concéntricos: el resaltado recorta el radio del panel.

use dokka_scene::DRAWING_SIDE;

use super::tray_glass;

/// Ancho fijo del panel (el de los menús Tahoe).
pub const PANEL_WIDTH: f32 = tray_glass::WIDTH;
/// Aire dentro del cristal único.
pub const PADDING: f32 = tray_glass::PADDING;
/// Esquinas y borde del cristal único.
pub const PANEL_RADIUS: f32 = tray_glass::PANEL_RADIUS;
pub const PANEL_BORDER_WIDTH: f32 = tray_glass::PANEL_BORDER_WIDTH;

/// Alto de la fila del título.
pub const TITLE_HEIGHT: f32 = tray_glass::TITLE_HEIGHT;
/// Separación entre el título y la cápsula del deslizador.
pub const TITLE_GAP: f32 = 6.0;

/// Alto del área de pulsación del deslizador: cómoda, más alta que él.
pub const SLIDER_ROW_HEIGHT: f32 = 28.0;
/// Alto de la cápsula ancha (relleno blanco, tirador redondo).
pub const SLIDER_HEIGHT: f32 = tray_glass::SLIDER_HEIGHT;
/// Diámetro del tirador blanco.
pub const THUMB_DIAMETER: f32 = tray_glass::THUMB_DIAMETER;
/// Aire del número dentro de la cápsula, a la izquierda.
pub const PERCENT_INSET: f32 = 10.0;
/// Ancho mínimo del relleno para leer el número en oscuro sobre blanco.
pub const PERCENT_FILL_MIN: f32 = 48.0;
/// Lado del glifo del altavoz de la fila de silencio.
pub const ICON_SIDE: f32 = 16.0;
/// Aire entre el icono y el texto de la fila de silencio.
pub const ICON_GAP: f32 = 10.0;
/// Alto de cada fila compacta (silenciar y ajustes).
pub const ROW_HEIGHT: f32 = tray_glass::ROW_HEIGHT;
/// Separación entre la cápsula o una fila y su línea.
pub const SECTION_GAP: f32 = 8.0;
/// Radio del resaltado suave de cada fila, concéntrico.
pub const HIGHLIGHT_RADIUS: f32 = tray_glass::HIGHLIGHT_RADIUS;

/// Tamaño del glifo Lucide del altavoz: llena el lienzo de 100.
pub const SPEAKER_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo Lucide del botón de silencio anterior: se conserva
/// para no romper usos externos; el panel Tahoe usa el altavoz en su fila.
pub const MUTE_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaño del glifo Lucide del chevrón de ajustes.
pub const SETTINGS_GLYPH_SIZE: f32 = DRAWING_SIDE;

/// Lado del chevrón de la fila de ajustes.
pub const SETTINGS_CHEVRON_SIDE: f32 = 16.0;

/// Título del panel.
pub const TITLE: &str = "Sonido";
/// Fila que silencia.
pub const MUTE_LABEL: &str = "Silenciar";
/// Fila que abre los ajustes.
pub const SETTINGS_LABEL: &str = "Configuración de sonido…";

/// Tamaños de texto del panel Tahoe.
pub const TITLE_SIZE: f32 = tray_glass::TITLE_SIZE;

/// Alto del panel: título, cápsula, dos líneas y dos filas con sus aires.
pub fn panel_height() -> f32 {
    PADDING
        + TITLE_HEIGHT
        + TITLE_GAP
        + SLIDER_ROW_HEIGHT
        + SECTION_GAP
        + tray_glass::SEPARATOR_HEIGHT
        + ROW_HEIGHT
        + tray_glass::SEPARATOR_HEIGHT
        + ROW_HEIGHT
        + PADDING
}
