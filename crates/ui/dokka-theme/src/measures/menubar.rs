//! Medidas de la barra superior estilo macOS: franja baja de lado a lado
//! con el icono, el nombre de la app y la fecha a la derecha. El menú que
//! cuelga del icono reutiliza `menu_panel` sin punta; con su separación
//! las esquinas quedan concéntricas (radio interior = exterior - separación).

use dokka_scene::DRAWING_SIDE;

/// Alto de la franja (28-32, como macOS).
pub const HEIGHT: f32 = 30.0;
/// Alto de la ventana: la franja más el hueco y el panel colgando.
/// El panel más alto es el buscador (386 con siete filas); el idioma compacto
/// queda muy por debajo y el calendario (291) sigue cabiendo igual.
pub const WINDOW_HEIGHT: f32 = 480.0;
/// Lado del icono de Dokka (Lucide), igual que los de la derecha.
pub const ICON_SIDE: f32 = 16.0;
/// Tamaño del glifo: llena el lienzo de 100, fuente efectiva 20 a 20 de lado.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Aire a la izquierda del icono y a la derecha del reloj.
pub const SIDE_PADDING: f32 = 12.0;
/// Separación entre el icono y el nombre de la app.
pub const ICON_GAP: f32 = 10.0;
/// Ancho reservado a la derecha para la hora, centrada en él.
pub const CLOCK_SLOT: f32 = 52.0;
/// Separación entre la franja y el menú colgando.
pub const MENU_GAP: f32 = 6.0;
/// Borde izquierdo del menú, bajo el icono.
pub const MENU_X: f32 = 8.0;
/// Radio del resaltado suave detrás del icono.
pub const HOVER_RADIUS: f32 = 8.0;
/// Aire del resaltado alrededor del icono.
pub const HOVER_PADDING: f32 = 4.0;
/// La franja no lleva borde, como en macOS.
pub const BORDER_WIDTH: f32 = 0.0;
