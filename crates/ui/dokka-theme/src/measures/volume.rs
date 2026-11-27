//! Medidas del panel de volumen estilo Tahoe: un solo cristal oscuro con
//! título y porcentaje, barra fina del componente `slider` con tirador en
//! cápsula entre dos altavoces, silenciar y ajustes separados por líneas finas.
//!
//! El deslizador es el componente `slider` con su pista fina de 4 y su
//! tirador alargado; aquí solo quedan la fila, los iconos y sus aires.
//! Radios concéntricos: el resaltado recorta el radio del panel.

use dokka_scene::DRAWING_SIDE;

use super::tray_glass;

/// Ancho fijo del panel (el de los menús Tahoe).
pub const PANEL_WIDTH: f32 = tray_glass::WIDTH;
/// Aire dentro del cristal único.
pub const PADDING: f32 = tray_glass::PADDING;
/// Esquinas y borde del cristal único.
pub const PANEL_RADIUS: f32 = tray_glass::PANEL_RADIUS;
pub const PANEL_BORDER_WIDTH: f32 = tray_glass::PANEL_BORDER_WIDTH;

/// Alto de la fila del título ("Sonido" y porcentaje).
pub const TITLE_HEIGHT: f32 = tray_glass::TITLE_HEIGHT;
/// Separación entre la fila del título y la del deslizador.
pub const TITLE_GAP: f32 = 6.0;

/// Alto de la fila del deslizador: cómoda al pulsar, con aire para que el
/// tirador en cápsula quede centrado sin salirse de ella.
pub const SLIDER_ROW_HEIGHT: f32 = 28.0;
/// Lado de cada altavoz a los extremos de la barra.
pub const SLIDER_ICON_SIDE: f32 = 16.0;
/// Aire entre cada altavoz y la barra.
pub const SLIDER_ICON_GAP: f32 = 8.0;
/// Ancho reservado al porcentaje a la derecha del título: cabe "100%" y,
/// al ir centrado en él, no salta al cambiar de dígitos.
pub const PERCENT_BOX_WIDTH: f32 = 44.0;
/// Lado del glifo del altavoz de la fila de silencio.
pub const ICON_SIDE: f32 = 16.0;
/// Aire entre el icono y el texto de la fila de silencio.
pub const ICON_GAP: f32 = 10.0;
/// Alto de cada fila compacta (silenciar y ajustes).
pub const ROW_HEIGHT: f32 = tray_glass::ROW_HEIGHT;
/// Separación entre la fila del deslizador o una fila y su línea.
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

/// Título de la sección de salidas.
pub const OUTPUT_TITLE: &str = "Salida";
/// Salidas visibles como máximo (6 × 34 + título y aires = 386, cabe en 444).
pub const MAX_OUTPUTS: usize = 6;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 28;
/// Lado de la marca de verificación de la salida activa.
pub const CHECK_SIDE: f32 = 16.0;
/// Título del panel.
pub const TITLE: &str = "Sonido";
/// Fila que silencia.
pub const MUTE_LABEL: &str = "Silenciar";
/// Fila que abre los ajustes.
pub const SETTINGS_LABEL: &str = "Configuración de sonido…";

/// Tamaños de texto del panel Tahoe.
pub const TITLE_SIZE: f32 = tray_glass::TITLE_SIZE;
pub const SUBTITLE_SIZE: f32 = tray_glass::SUBTITLE_SIZE;

/// Salidas que se muestran de `total`.
pub fn visible_outputs(total: usize) -> usize {
    total.min(MAX_OUTPUTS)
}

/// Alto del panel: título, deslizador, sección de salidas si hay, dos
/// líneas y dos filas con sus aires. Con seis salidas son 386, cabe en
/// la ventana (480 menos franja de 30 y hueco de 6 = 444).
pub fn panel_height(output_count: usize) -> f32 {
    let visible = visible_outputs(output_count) as f32;
    let outputs = if visible > 0.0 {
        TITLE_GAP + TITLE_HEIGHT + visible * ROW_HEIGHT
    } else {
        0.0
    };
    PADDING
        + TITLE_HEIGHT
        + TITLE_GAP
        + SLIDER_ROW_HEIGHT
        + outputs
        + SECTION_GAP
        + tray_glass::SEPARATOR_HEIGHT
        + ROW_HEIGHT
        + tray_glass::SEPARATOR_HEIGHT
        + ROW_HEIGHT
        + PADDING
}
