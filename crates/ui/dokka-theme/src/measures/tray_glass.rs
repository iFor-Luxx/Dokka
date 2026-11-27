//! Medidas comunes de los menús de la barra superior, estilo Tahoe: un
//! solo cristal oscuro con secciones separadas por líneas finas.
//! Radios concéntricos: el resaltado recorta el radio del panel.

/// Ancho de un menú de la barra.
pub const WIDTH: f32 = 320.0;
/// Hueco entre piezas (se conserva por compatibilidad).
pub const GAP: f32 = 10.0;
/// Aire dentro del cristal único.
pub const PADDING: f32 = 12.0;
/// Esquinas del cristal único (14-16).
pub const PANEL_RADIUS: f32 = 16.0;
pub const PANEL_BORDER_WIDTH: f32 = 1.0;
/// Esquinas de las tarjetas (se conserva por compatibilidad).
pub const TILE_RADIUS: f32 = 24.0;
pub const TILE_BORDER_WIDTH: f32 = 1.0;
/// Alto del título en negrita arriba a la izquierda.
pub const TITLE_HEIGHT: f32 = 20.0;
/// Alto de una fila compacta de texto (32-36).
pub const ROW_HEIGHT: f32 = 34.0;
/// Grosor de la línea que separa secciones.
pub const SEPARATOR_HEIGHT: f32 = 1.0;
/// Radio del resaltado suave dentro del cristal, concéntrico.
pub const HIGHLIGHT_RADIUS: f32 = 10.0;
/// Separación del resaltado al borde del panel, igual a izquierda y derecha.
pub const ROW_MARGIN: f32 = 6.0;
/// Aire dentro del resaltado antes del contenido, igual a izquierda y derecha.
pub const ROW_INSET: f32 = 10.0;
/// Alto de una cápsula (icono, título y subtítulo, compatibilidad).
pub const CAPSULE_HEIGHT: f32 = 60.0;
/// Lado de los círculos de icono en filas de dispositivo.
pub const CIRCLE: f32 = 26.0;
/// Lado del icono dentro del círculo.
pub const CIRCLE_ICON: f32 = 16.0;
/// Alto de la cápsula ancha del deslizador de volumen.
pub const SLIDER_HEIGHT: f32 = 18.0;
/// Grosor de la barra del deslizador: la cápsula ancha.
pub const TRACK_HEIGHT: f32 = SLIDER_HEIGHT;
/// Diámetro del tirador redondo blanco del deslizador.
pub const THUMB_DIAMETER: f32 = 14.0;
/// Tamaño de título y de subtítulo.
pub const TITLE_SIZE: f32 = 13.0;
pub const SUBTITLE_SIZE: f32 = 12.0;
