//! Medidas de los paneles de red e idioma que cuelgan de la barra.
//!
//! Estilo Tahoe: un solo cristal oscuro con título, filas compactas y
//! ajustes separados por líneas finas. Radios concéntricos: el resaltado
//! recorta el radio del panel.

use dokka_scene::DRAWING_SIDE;

use super::tray_glass;

/// Ancho fijo de ambos paneles (como `tray_glass::WIDTH`).
pub const PANEL_WIDTH: f32 = 320.0;
/// Aire dentro del cristal único.
pub const PADDING: f32 = 12.0;
/// Esquinas y borde del cristal único.
pub const PANEL_RADIUS: f32 = 16.0;
pub const PANEL_BORDER_WIDTH: f32 = 1.0;
/// Hueco entre piezas (se conserva por compatibilidad).
pub const GAP: f32 = 10.0;
/// Esquinas de las tarjetas (se conserva por compatibilidad).
pub const TILE_RADIUS: f32 = 24.0;
pub const TILE_BORDER_WIDTH: f32 = 1.0;
/// Alto de la fila del título.
pub const TITLE_HEIGHT: f32 = 20.0;
/// Separación entre el título y la primera fila.
pub const TITLE_GAP: f32 = 6.0;
/// Alto de la fila de estado de red (tipo y nombre, dos líneas).
pub const STATUS_HEIGHT: f32 = 52.0;
/// Alto de cada fila compacta (idioma y ajustes, 32-36).
pub const ROW_HEIGHT: f32 = 34.0;
/// Separación entre filas de idioma.
pub const ROW_GAP: f32 = 4.0;
/// Separación entre una fila y su línea.
pub const SECTION_GAP: f32 = 8.0;
/// Lado de los círculos de icono de red.
pub const CIRCLE: f32 = 26.0;
/// Lado del icono dentro del círculo.
pub const CIRCLE_ICON: f32 = 16.0;
/// Lado del chevrón de la fila de ajustes.
pub const CHEVRON_SIDE: f32 = 16.0;
/// Aire entre el círculo o la marca y los textos.
pub const ICON_GAP: f32 = 10.0;
/// Lado de la marca de verificación del idioma activo.
pub const CHECK_SIDE: f32 = 16.0;
/// Caja del código del idioma: recuadro pequeño a la derecha.
pub const CODE_WIDTH: f32 = 34.0;
pub const CODE_HEIGHT: f32 = 20.0;
pub const CODE_RADIUS: f32 = 6.0;
/// Radio del resaltado suave de cada fila, concéntrico.
pub const HIGHLIGHT_RADIUS: f32 = 10.0;
/// El glifo llena el lienzo de 100: la fuente efectiva iguala al lado.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaños de título y subtítulo (como `tray_glass`).
pub const TITLE_SIZE: f32 = 13.0;
pub const SUBTITLE_SIZE: f32 = 12.0;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 28;
/// Título de cada panel.
pub const NETWORK_TITLE: &str = "Red";
pub const LANGUAGE_TITLE: &str = "Idioma";
/// Fila que abre los ajustes.
pub const NETWORK_SETTINGS_LABEL: &str = "Configuración de red…";
pub const LANGUAGE_SETTINGS_LABEL: &str = "Configuración de idioma…";
/// Cabecera cuando no hay conexión.
pub const OFFLINE_NAME: &str = "Sin conexión";
/// Tipos de conexión para la línea de detalle.
pub const TYPE_WIFI: &str = "Wi-Fi";
pub const TYPE_ETHERNET: &str = "Cable";
pub const TYPE_OTHER: &str = "Red";
pub const TYPE_OFFLINE: &str = "Sin conexión";
/// Estado de internet en la línea de detalle.
pub const WITH_INTERNET: &str = "Con internet";
pub const WITHOUT_INTERNET: &str = "Sin internet";
/// Subtítulo de la fila del idioma en uso.
pub const ACTIVE_SUBTITLE: &str = "Activa";

/// Alto del panel de red: título, estado, línea y ajustes.
pub fn network_height() -> f32 {
    PADDING
        + TITLE_HEIGHT
        + TITLE_GAP
        + STATUS_HEIGHT
        + SECTION_GAP
        + tray_glass::SEPARATOR_HEIGHT
        + ROW_HEIGHT
        + PADDING
}

/// Alto del panel de idioma: título, una fila compacta por idioma y ajustes.
pub fn language_height(count: usize) -> f32 {
    let rows = count as f32 * ROW_HEIGHT + (count as f32 - 1.0).max(0.0) * ROW_GAP;
    PADDING
        + TITLE_HEIGHT
        + TITLE_GAP
        + rows
        + SECTION_GAP
        + tray_glass::SEPARATOR_HEIGHT
        + ROW_HEIGHT
        + PADDING
}
