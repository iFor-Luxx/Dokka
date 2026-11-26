//! Medidas del Launchpad a pantalla completa.

//! Icono grande, como en macOS.
pub const ICON_SIDE: f32 = 90.0;
/// Hasta 7 columnas por pantalla.
pub const MAX_COLUMNS: usize = 7;
pub const COLUMN_GAP: f32 = 24.0;
pub const ROW_GAP: f32 = 28.0;
/// Aire entre el icono y su nombre.
pub const TEXT_GAP: f32 = 6.0;
pub const TEXT_SIZE: f32 = 13.0;
/// Alto aproximado de una línea de texto.
pub const TEXT_HEIGHT: f32 = 18.0;
/// Margen superior para centrar la cuadrícula.
pub const TOP_PADDING: f32 = 60.0;
pub const SIDE_PADDING: f32 = 40.0;
/// Reserva inferior para no quedar bajo el dock.
pub const BOTTOM_RESERVE: f32 = 140.0;
pub const HIGHLIGHT_RADIUS: f32 = 16.0;
/// Aire del resaltado alrededor del icono y su nombre.
pub const HIGHLIGHT_PADDING: f32 = 12.0;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 18;
