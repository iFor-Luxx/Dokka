//! Medidas de los paneles de red e idioma que cuelgan de la barra.
//!
//! Estilo Tahoe: piezas de cristal separadas sobre fondo invisible.
//! Radios concéntricos: el interior es el exterior menos la separación.
//! Los valores genéricos espejan `tray_glass`.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo de ambos paneles (como `tray_glass::WIDTH`).
pub const PANEL_WIDTH: f32 = 320.0;
/// Hueco entre piezas y con el borde (como `tray_glass::GAP`).
pub const GAP: f32 = 10.0;
/// Aire dentro de cada pieza (como `tray_glass::PADDING`).
pub const PADDING: f32 = 12.0;
/// Esquinas de las tarjetas (como `tray_glass::TILE_RADIUS`).
pub const TILE_RADIUS: f32 = 24.0;
pub const TILE_BORDER_WIDTH: f32 = 1.0;
/// Alto de la pieza con el título.
pub const TITLE_HEIGHT: f32 = 40.0;
/// Alto de cada cápsula con icono, nombre y subtítulo.
pub const ROW_HEIGHT: f32 = 60.0;
/// Alto de la zona propia de ajustes (círculo con engranaje y etiqueta).
pub const SETTINGS_HEIGHT: f32 = 52.0;
/// Lado de los círculos de icono dentro de una cápsula.
pub const CIRCLE: f32 = 38.0;
/// Lado del icono dentro del círculo.
pub const CIRCLE_ICON: f32 = 20.0;
/// Lado del chevrón de la fila de ajustes.
pub const CHEVRON_SIDE: f32 = 16.0;
/// Aire entre el círculo y los textos.
pub const ICON_GAP: f32 = 12.0;
/// Radio del resaltado suave dentro de una tarjeta.
pub const HIGHLIGHT_RADIUS: f32 = TILE_RADIUS - GAP;
/// El glifo llena el lienzo de 100: la fuente efectiva iguala al lado.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Tamaños de título y subtítulo (como `tray_glass`).
pub const TITLE_SIZE: f32 = 13.0;
pub const SUBTITLE_SIZE: f32 = 12.0;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 28;
/// Título de cada pieza.
pub const NETWORK_TITLE: &str = "Red";
pub const LANGUAGE_TITLE: &str = "Idioma";
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
/// Subtítulo de la fila del idioma en uso.
pub const ACTIVE_SUBTITLE: &str = "Activa";

/// Alto del panel de red: título, cápsula y ajustes con sus huecos.
pub fn network_height() -> f32 {
    GAP * 4.0 + TITLE_HEIGHT + ROW_HEIGHT + SETTINGS_HEIGHT
}

/// Alto del panel de idioma: título, una cápsula por idioma y ajustes.
pub fn language_height(count: usize) -> f32 {
    GAP * (count as f32 + 3.0) + TITLE_HEIGHT + ROW_HEIGHT * count as f32 + SETTINGS_HEIGHT
}
