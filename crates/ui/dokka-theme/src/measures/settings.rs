//! Medidas de la ventana de Preferencias estilo Tahoe.
//!
//! Ventana de 760 × 720 lógicas con la barra lateral a la izquierda y el
//! contenido a la derecha. Todo son posiciones fijas, sin proporciones.

/// Tamaño lógico de la ventana.
pub const WIDTH: f32 = 760.0;
/// Alto lógico de la ventana.
pub const HEIGHT: f32 = 720.0;
/// Esquinas de la ventana, muy redondeadas.
pub const CORNER_RADIUS: f32 = 20.0;
pub const BORDER_WIDTH: f32 = 1.0;

/// Ancho de la barra lateral.
pub const SIDEBAR_WIDTH: f32 = 260.0;

/// Diámetro de cada semáforo (cerrar, minimizar, zoom).
pub const TRAFFIC_DIAMETER: f32 = 12.0;
/// Distancia entre bordes izquierdos de semáforos seguidos.
pub const TRAFFIC_PITCH: f32 = 20.0;
/// Borde izquierdo del primer semáforo.
pub const TRAFFIC_LEFT: f32 = 20.0;
/// Borde superior de los semáforos.
pub const TRAFFIC_TOP: f32 = 20.0;
/// Aire para pulsar cada semáforo alrededor de su círculo.
pub const TRAFFIC_HIT_PADDING: f32 = 6.0;

/// Margen lateral de la casilla y las filas dentro de la barra.
pub const SIDEBAR_MARGIN: f32 = 12.0;
/// Borde superior de la casilla de búsqueda, bajo los semáforos.
pub const SEARCH_TOP: f32 = 52.0;
pub const SEARCH_HEIGHT: f32 = 30.0;
pub const SEARCH_RADIUS: f32 = 8.0;
pub const SEARCH_BORDER_WIDTH: f32 = 1.0;
/// Lado de la lupa dentro de la casilla.
pub const SEARCH_ICON_SIDE: f32 = 16.0;
/// Aire entre el borde de la casilla y la lupa.
pub const SEARCH_ICON_INSET: f32 = 10.0;
/// Distancia del borde izquierdo al texto: lupa más aire a ambos lados.
pub const SEARCH_TEXT_INSET: f32 = SEARCH_ICON_INSET + SEARCH_ICON_SIDE + 8.0;
pub const SEARCH_TEXT_SIZE: f32 = 13.0;
/// Texto que muestra la casilla, sin ajustes reales todavía.
pub const SEARCH_PLACEHOLDER: &str = "Buscar";

/// Borde superior de la primera fila, bajo la casilla.
pub const ROWS_TOP: f32 = 96.0;
/// Paso vertical entre filas.
pub const ROW_PITCH: f32 = 40.0;
/// Lado del cuadrado de color de cada fila.
pub const BADGE_SIDE: f32 = 28.0;
/// Radio del cuadrado de color.
pub const BADGE_RADIUS: f32 = 7.0;
/// Aire entre el cuadrado y el nombre.
pub const BADGE_TEXT_GAP: f32 = 10.0;
/// Borde izquierdo del cuadrado dentro de la barra.
pub const BADGE_LEFT: f32 = 20.0;
/// Tamaño del nombre de cada sección.
pub const ROW_TEXT_SIZE: f32 = 13.0;
/// Tamaño del glifo Lucide blanco sobre el cuadrado.
pub const ROW_GLYPH_SIZE: f32 = dokka_scene::DRAWING_SIDE;
/// Lado del glifo blanco dentro del cuadrado.
pub const ROW_ICON_SIDE: f32 = 18.0;
/// Margen de la fila resaltada dentro de la barra.
pub const ROW_HIGHLIGHT_MARGIN: f32 = 8.0;
/// Alto de la fila resaltada (selección y hover).
pub const ROW_HIGHLIGHT_HEIGHT: f32 = 36.0;
pub const ROW_HIGHLIGHT_RADIUS: f32 = 8.0;

/// Alto de la zona arrastrable superior (mover la ventana).
pub const DRAG_HEIGHT: f32 = 64.0;

/// Píldora de navegación de la cabecera, hoy con ambos lados atenuados (~72 × 34).
pub const NAV_PILL_WIDTH: f32 = 72.0;
pub const NAV_PILL_HEIGHT: f32 = 34.0;
/// Borde izquierdo de la píldora.
pub const NAV_LEFT: f32 = 276.0;
/// Borde superior de la píldora de la cabecera.
pub const NAV_TOP: f32 = 18.0;
/// Aire entre cada chevrón y el título.
pub const NAV_TITLE_GAP: f32 = 8.0;
/// Tamaño del título de la sección elegida.
pub const TITLE_SIZE: f32 = 20.0;

/// Línea bajo la cabecera: de dónde a dónde y su grosor.
pub const HEADER_DIVIDER_LEFT: f32 = 276.0;
pub const HEADER_DIVIDER_RIGHT: f32 = 744.0;
pub const HEADER_DIVIDER_TOP: f32 = 64.0;
pub const HEADER_DIVIDER_HEIGHT: f32 = 1.0;

/// Tarjeta gris del contenido, todavía sin ajustes.
pub const CARD_MARGIN: f32 = 24.0;
pub const CARD_TOP: f32 = 88.0;
pub const CARD_HEIGHT: f32 = 120.0;
pub const CARD_RADIUS: f32 = 12.0;
pub const CARD_BORDER_WIDTH: f32 = 1.0;
pub const CARD_TEXT_SIZE: f32 = 14.0;
/// Texto de la tarjeta, único contenido por ahora.
pub const CARD_TEXT: &str = "Próximamente";

/// Secciones en orden, como las ve la barra lateral.
pub const SECTIONS: usize = 7;

/// Borde superior de la fila `index`.
pub fn row_top(index: usize) -> f32 {
    ROWS_TOP + index as f32 * ROW_PITCH
}

/// Centro vertical de la fila `index`.
pub fn row_middle(index: usize) -> f32 {
    row_top(index) + ROW_PITCH / 2.0
}

/// Ancho de la casilla de búsqueda dentro de la barra.
pub fn search_width() -> f32 {
    SIDEBAR_WIDTH - SIDEBAR_MARGIN * 2.0
}

/// Borde izquierdo del título, tras la píldora.
pub fn title_left() -> f32 {
    NAV_LEFT + NAV_PILL_WIDTH + NAV_TITLE_GAP
}

/// Centro vertical del título y la píldora.
pub fn header_middle() -> f32 {
    NAV_TOP + NAV_PILL_HEIGHT / 2.0
}

/// Límites de la tarjeta gris del contenido.
pub fn card_bounds() -> (f32, f32, f32) {
    let x = SIDEBAR_WIDTH + CARD_MARGIN;
    let width = WIDTH - x - CARD_MARGIN;
    (x, CARD_TOP, width)
}
