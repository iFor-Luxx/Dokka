//! Medidas de la ventana del instalador propio, estilo iPhone.
//!
//! Contenido de 720 × 540 lógicas con esquinas de 26, sin marco de Windows.
//! Todo centrado a lo ancho; el botón principal va a `BUTTON_TOP`.

/// Ancho lógico del contenido.
pub const WIDTH: f32 = 720.0;
/// Alto lógico del contenido.
pub const HEIGHT: f32 = 540.0;
/// Esquinas de la ventana, como un teléfono.
pub const CORNER_RADIUS: f32 = 26.0;
pub const BORDER_WIDTH: f32 = 1.0;

/// Margen transparente alrededor donde cabe la sombra del compositor.
pub const SHADOW_MARGIN: f32 = 40.0;
/// Desenfoque de la sombra, como en Preferencias.
pub const SHADOW_BLUR: f32 = 30.0;
/// La sombra cae unos 8 hacia abajo.
pub const SHADOW_OFFSET_Y: f32 = 8.0;

/// Hueco del logo en Bienvenida: cuadrado translúcido.
pub const LOGO_SIDE: f32 = 108.0;
pub const LOGO_RADIUS: f32 = 26.0;
pub const LOGO_TOP: f32 = 76.0;
pub const LOGO_TEXT_SIZE: f32 = 15.0;

/// Logo algo menor en la pantalla de desinstalar.
pub const UNINSTALL_LOGO_SIDE: f32 = 90.0;

/// Título grande centrado.
pub const TITLE_SIZE: f32 = 48.0;
/// Subtítulo bajo el título.
pub const SUBTITLE_SIZE: f32 = 17.0;
/// Aire entre el título y el subtítulo.
pub const TITLE_GAP: f32 = 10.0;

/// Botón píldora principal.
pub const BUTTON_WIDTH: f32 = 248.0;
pub const BUTTON_HEIGHT: f32 = 50.0;
pub const BUTTON_TOP: f32 = 356.0;
pub const BUTTON_RADIUS: f32 = BUTTON_HEIGHT / 2.0;
pub const BUTTON_TEXT_SIZE: f32 = 17.0;

/// Enlace discreto bajo el botón.
pub const LINK_GAP: f32 = 14.0;
pub const LINK_TEXT_SIZE: f32 = 13.0;
pub const LINK_HIT_PAD: f32 = 8.0;

/// Botón redondo de cerrar y volver, arriba a la izquierda.
pub const ROUND_SIDE: f32 = 44.0;
pub const ROUND_LEFT: f32 = 8.0;
pub const ROUND_TOP: f32 = 8.0;
pub const ROUND_RADIUS: f32 = ROUND_SIDE / 2.0;
/// Lado del dibujo de la X y del chevrón dentro del botón.
pub const ROUND_GLYPH_SIDE: f32 = 20.0;

/// Tarjeta de opciones.
pub const CARD_WIDTH: f32 = 380.0;
pub const CARD_RADIUS: f32 = 26.0;
pub const CARD_BORDER_WIDTH: f32 = 1.0;
pub const CARD_TOP: f32 = 236.0;
pub const ROW_HEIGHT: f32 = 58.0;
pub const ROW_TEXT_SIZE: f32 = 14.0;
pub const ROW_PADDING: f32 = 18.0;

/// Barra fina de avance.
pub const BAR_WIDTH: f32 = 248.0;
pub const BAR_HEIGHT: f32 = 6.0;
pub const BAR_TOP: f32 = 368.0;
pub const BAR_RADIUS: f32 = BAR_HEIGHT / 2.0;
pub const STEP_GAP: f32 = 12.0;
pub const STEP_TEXT_SIZE: f32 = 13.0;

/// Círculo de estado (check o aviso).
pub const BADGE_SIDE: f32 = 96.0;
pub const BADGE_RADIUS: f32 = BADGE_SIDE / 2.0;
pub const BADGE_TOP: f32 = 84.0;
pub const BADGE_GLYPH_SIDE: f32 = 48.0;

/// Casilla de "Borrar también mis ajustes".
pub const CHECK_SIDE: f32 = 22.0;
pub const CHECK_RADIUS: f32 = 6.0;
pub const CHECK_GAP: f32 = 12.0;
pub const CHECK_TOP: f32 = 300.0;
pub const CHECK_TEXT_SIZE: f32 = 14.0;

/// Cuánto tarda el fundido del contenido al cambiar de pantalla, en segundos.
pub const FADE_SECONDS: f32 = 0.18;

/// Entrada inicial: las montañas entran desde los lados desenfocadas.
pub const ENTRY_TRAVEL: f32 = 240.0;
/// Desenfoque gaussiano suave inicial de las montañas.
pub const ENTRY_BLUR_MAX: f32 = 3.0;
/// Viaje de las montañas, a ritmo parejo, en segundos.
pub const ENTRY_MOUNTAINS_SECONDS: f32 = 2.4;
/// Fundido de `playa.jpg` encima al llegar, en segundos.
pub const ENTRY_PHOTO_FADE_SECONDS: f32 = 0.35;
/// El contenido aparece en bloque desde este segundo.
pub const ENTRY_CONTENT_DELAY_SECONDS: f32 = 1.9;
/// Fundido y subida del contenido en bloque, en segundos.
pub const ENTRY_CONTENT_SECONDS: f32 = 0.7;
/// Cuánto sube el contenido al aparecer.
pub const ENTRY_CONTENT_RISE: f32 = 14.0;
/// Curva del viaje (cubic-bezier 0.35,0.45,0.35,1).
pub const ENTRY_EASE_X1: f32 = 0.35;
pub const ENTRY_EASE_Y1: f32 = 0.45;
pub const ENTRY_EASE_X2: f32 = 0.35;
pub const ENTRY_EASE_Y2: f32 = 1.0;

/// Centro horizontal del contenido.
pub fn center_x() -> f32 {
    WIDTH / 2.0
}

/// Rectángulo del botón principal.
pub fn button_rect() -> (f32, f32, f32, f32) {
    (
        (WIDTH - BUTTON_WIDTH) / 2.0,
        BUTTON_TOP,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    )
}

/// Rectángulo de la barra de avance.
pub fn bar_rect() -> (f32, f32, f32, f32) {
    ((WIDTH - BAR_WIDTH) / 2.0, BAR_TOP, BAR_WIDTH, BAR_HEIGHT)
}

/// Rectángulo del botón redondo de cerrar o volver.
pub fn round_rect() -> (f32, f32, f32, f32) {
    (ROUND_LEFT, ROUND_TOP, ROUND_SIDE, ROUND_SIDE)
}

/// Rectángulo de la tarjeta de opciones, centrada.
pub fn card_rect() -> (f32, f32, f32, f32) {
    (
        (WIDTH - CARD_WIDTH) / 2.0,
        CARD_TOP,
        CARD_WIDTH,
        ROW_HEIGHT * 2.0,
    )
}

/// Origen del contenido dentro de la ventana exterior: el aire de la sombra.
pub fn content_origin() -> (f32, f32) {
    (SHADOW_MARGIN, SHADOW_MARGIN)
}
