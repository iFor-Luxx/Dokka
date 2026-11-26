//! Medidas de los paneles flotantes.

/// Alto reservado sobre el dock para los paneles.
pub const SPACE: f32 = 420.0;
/// Separación entre un panel y los iconos agrandados.
pub const GAP: f32 = 10.0;
pub const CORNER_RADIUS: f32 = 16.0;
pub const BORDER_WIDTH: f32 = 1.0;
pub const PADDING: f32 = 12.0;

pub const CALENDAR_CELL_WIDTH: f32 = 38.0;
pub const CALENDAR_CELL_HEIGHT: f32 = 29.0;
pub const CALENDAR_HEADER: f32 = 40.0;
/// Cuánto baja la cabecera respecto a su centro.
pub const CALENDAR_HEADER_DROP: f32 = 2.0;
pub const CALENDAR_ARROW_SIDE: f32 = 28.0;
/// Aire entre el círculo de hoy y su celda.
pub const CALENDAR_TODAY_INSET: f32 = 3.0;

pub const PREVIEW_PADDING: f32 = 8.0;
pub const PREVIEW_SLOT_WIDTH: f32 = 270.0;
pub const PREVIEW_IMAGE_HEIGHT: f32 = 180.0;
pub const PREVIEW_IMAGE_INSET: f32 = 2.0;
pub const PREVIEW_TITLE_HEIGHT: f32 = 28.0;
pub const PREVIEW_TITLE_CHARACTERS: usize = 36;
pub const PREVIEW_MAXIMUM_WINDOWS: usize = 3;

pub const MEDIA_BUTTON_WIDTH: f32 = 52.0;
pub const MEDIA_HEIGHT: f32 = 42.0;
pub const MEDIA_ICON_SIDE: f32 = 22.0;
/// Distancia entre la píldora y el borde inferior de la imagen.
pub const MEDIA_INSET: f32 = 10.0;

/// Cuánto tarda en subir un panel al aparecer, con frenada al final.
pub const POPUP_ENTER_SECONDS: f32 = 0.18;
/// Cuánto tarda en bajar un panel al desaparecer.
pub const POPUP_EXIT_SECONDS: f32 = 0.14;
/// Cuánto sube un panel al aparecer (y baja al salir).
pub const POPUP_TRAVEL: f32 = 18.0;
