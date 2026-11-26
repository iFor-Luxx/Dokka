//! Medidas de los paneles flotantes.

use dokka_scene::DRAWING_SIDE;

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
/// Lado del icono de cada flecha, centrado en su zona pulsable (28):
/// deja 2 de aire por lado y la zona de clic no se reduce.
pub const CALENDAR_ARROW_ICON_SIDE: f32 = 24.0;
/// Tamaño del glifo Lucide para las flechas: llena el lienzo de 100,
/// de modo que la fuente efectiva es 24 al mostrarse a 24 de lado.
pub const CALENDAR_ARROW_GLYPH_SIZE: f32 = DRAWING_SIDE;
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
/// Lado del icono de cada botón, centrado en su botón de 52 × 42.
pub const MEDIA_ICON_SIDE: f32 = 24.0;
/// Tamaño del glifo Lucide para los botones de música: llena el lienzo
/// de 100, de modo que la fuente efectiva es 24 al mostrarse a 24 de lado.
pub const MEDIA_GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Distancia entre la píldora y el borde inferior de la imagen.
pub const MEDIA_INSET: f32 = 10.0;

/// Cuánto tarda en entrar un panel, creciendo desde el icono con un rebote leve.
pub const POPUP_ENTER_SECONDS: f32 = 0.28;
/// Cuánto tarda en salir un panel, encogiéndose hacia el icono y acelerando.
pub const POPUP_EXIT_SECONDS: f32 = 0.18;
/// Escala inicial al entrar, con origen en el centro inferior sobre el icono.
pub const POPUP_ENTER_SCALE: f32 = 0.85;
/// Escala final al salir, hacia el icono.
pub const POPUP_EXIT_SCALE: f32 = 0.9;
/// Cuánto sube un panel al entrar.
pub const POPUP_ENTER_TRAVEL: f32 = 14.0;
/// Cuánto baja un panel al salir.
pub const POPUP_EXIT_TRAVEL: f32 = 10.0;
/// Fracción de la animación en la que la opacidad ya llegó a 1.
pub const POPUP_OPACITY_SPAN: f32 = 0.5;
