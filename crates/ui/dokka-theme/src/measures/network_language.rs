//! Medidas de los paneles de red e idioma que cuelgan de la barra.
//!
//! Cristal claro como `menu_panel`, con tarjeta plana en la cabecera de red.
//! Radios concéntricos: el interior es el exterior menos la separación.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo de ambos paneles.
pub const PANEL_WIDTH: f32 = 300.0;
/// Radio exterior del cristal.
pub const CORNER_RADIUS: f32 = 16.0;
pub const BORDER_WIDTH: f32 = 1.0;
/// Aire interior alrededor de filas y tarjeta.
pub const PADDING: f32 = 6.0;
/// Radio interior, concéntrico con el exterior.
pub const HIGHLIGHT_RADIUS: f32 = CORNER_RADIUS - PADDING;
/// Radio de la tarjeta de cabecera, igual que el resaltado.
pub const CARD_RADIUS: f32 = HIGHLIGHT_RADIUS;
/// Alto de cada fila elegible (idiomas y ajustes).
pub const ROW_HEIGHT: f32 = 32.0;
/// Alto de la tarjeta con el estado de la red.
pub const HEADER_HEIGHT: f32 = 60.0;
/// Lado del icono Lucide de la cabecera de red.
pub const HEADER_ICON_SIDE: f32 = 32.0;
/// Aire entre el icono y los textos de la cabecera.
pub const HEADER_GAP: f32 = 12.0;
/// Separación del texto respecto al borde del resaltado.
pub const TEXT_INSET: f32 = 10.0;
/// Columna reservada a la izquierda para la marca del idioma activo.
pub const CHECK_WIDTH: f32 = 20.0;
/// Lado de la marca del idioma activo.
pub const CHECK_SIDE: f32 = 20.0;
/// El glifo llena el lienzo de 100: la fuente efectiva iguala al lado.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Lado del icono de red en la barra.
pub const BAR_ICON_SIDE: f32 = 20.0;
/// Hueco vertical que ocupa la línea separadora.
pub const SEPARATOR_HEIGHT: f32 = 11.0;
pub const DIVIDER_HEIGHT: f32 = 1.0;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 28;
/// Fila que abre los ajustes.
pub const NETWORK_SETTINGS_LABEL: &str = "Configuración de red…";
pub const LANGUAGE_SETTINGS_LABEL: &str = "Configuración de idioma…";
/// Cabecera cuando no hay conexión.
pub const OFFLINE_NAME: &str = "Sin conexión";
/// Tipos de conexión para la línea de detalle.
pub const TYPE_WIFI: &str = "Wi-Fi";
pub const TYPE_ETHERNET: &str = "Ethernet";
pub const TYPE_OTHER: &str = "Red";
pub const TYPE_OFFLINE: &str = "Sin conexión";
/// Estado de internet en la línea de detalle.
pub const WITH_INTERNET: &str = "Con internet";
pub const WITHOUT_INTERNET: &str = "Sin internet";

/// Alto fijo del panel de red: tarjeta, separador y fila de ajustes.
pub fn network_height() -> f32 {
    PADDING * 2.0 + HEADER_HEIGHT + SEPARATOR_HEIGHT + ROW_HEIGHT
}

/// Alto del panel de idioma: una fila por idioma más la de ajustes.
pub fn language_height(count: usize) -> f32 {
    PADDING * 2.0 + ROW_HEIGHT * (count as f32 + 1.0) + SEPARATOR_HEIGHT
}
