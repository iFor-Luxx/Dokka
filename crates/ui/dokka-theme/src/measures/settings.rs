//! Medidas de la ventana de Preferencias estilo Tahoe.
//!
//! Ventana de 760 × 720 lógicas con la barra lateral a la izquierda y el
//! contenido a la derecha. Todo son posiciones fijas, sin proporciones.

/// Tamaño lógico de la ventana.
pub const WIDTH: f32 = 760.0;
/// Alto lógico de la ventana.
pub const HEIGHT: f32 = 720.0;
/// Esquinas de la ventana, muy redondeadas.
pub const CORNER_RADIUS: f32 = 26.0;
pub const BORDER_WIDTH: f32 = 1.0;

/// Panel flotante dentro de la ventana: margen parejo arriba, abajo y a la
/// izquierda donde se ve el fondo claro de la ventana. El borde derecho sigue
/// en `SIDEBAR_WIDTH` para no mover la cabecera ni la tarjeta.
pub const SIDEBAR_INSET: f32 = 8.0;
/// Margen interior parejo del contenido (casilla, insignias) al borde del panel.
pub const SIDEBAR_MARGIN: f32 = 10.0;
/// Esquinas del panel, concéntricas con las de la ventana.
pub const SIDEBAR_RADIUS: f32 = CORNER_RADIUS - SIDEBAR_INSET;
/// Borde fino y luminoso del panel flotante.
pub const SIDEBAR_BORDER_WIDTH: f32 = 1.0;

/// Ancho de la barra lateral.
pub const SIDEBAR_WIDTH: f32 = 260.0;

/// Diámetro de cada semáforo (cerrar, minimizar, zoom).
pub const TRAFFIC_DIAMETER: f32 = 16.0;
/// Distancia entre bordes izquierdos de semáforos seguidos.
pub const TRAFFIC_PITCH: f32 = 25.0;
/// Borde izquierdo del primer semáforo: con aire (~18) al borde del panel.
pub const TRAFFIC_LEFT: f32 = SIDEBAR_INSET + 16.0;
/// Borde superior de los semáforos: centrados entre el panel y la casilla.
pub const TRAFFIC_TOP: f32 = 23.0;
/// Aire para pulsar cada semáforo alrededor de su círculo.
pub const TRAFFIC_HIT_PADDING: f32 = 4.5;
/// Lado del símbolo Lucide dentro de cada semáforo (círculo de 14).
pub const TRAFFIC_GLYPH_SIDE: f32 = 13.0;
/// Desplazamiento de las copias que engrosan el trazo del símbolo.
pub const TRAFFIC_GLYPH_SPREAD: f32 = 0.4;
/// Tamaño del glifo Lucide del semáforo: llena el lienzo de 100, así la
/// fuente efectiva es 10 al mostrarse a 10 de lado.
pub const TRAFFIC_GLYPH_SIZE: f32 = dokka_scene::DRAWING_SIDE;

/// Borde superior de la casilla de búsqueda, bajo los semáforos.
pub const SEARCH_TOP: f32 = 52.0;
pub const SEARCH_HEIGHT: f32 = 30.0;
/// Cápsula: extremos totalmente redondos (mitad del alto).
pub const SEARCH_RADIUS: f32 = SEARCH_HEIGHT / 2.0;
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
/// Ancho aproximado de cada letra escrita en la casilla, para colocar el
/// cursor tras el texto sin medir la fuente en cada redibujado.
pub const SEARCH_CHAR_ADVANCE: f32 = 7.0;
/// Aire entre la última letra y el cursor vertical.
pub const SEARCH_CURSOR_GAP: f32 = 2.0;
/// Ancho del cursor vertical de la casilla enfocada.
pub const SEARCH_CURSOR_WIDTH: f32 = 1.5;
/// Alto del cursor vertical de la casilla enfocada.
pub const SEARCH_CURSOR_HEIGHT: f32 = 15.0;
/// Cuántas letras guarda la búsqueda de Preferencias.
pub const SEARCH_MAX_CHARACTERS: usize = 32;
/// Mensaje en la barra lateral cuando el filtro no encuentra nada.
pub const EMPTY_TEXT: &str = "Sin resultados";
/// Tamaño del mensaje de búsqueda vacía.
pub const EMPTY_TEXT_SIZE: f32 = 13.0;

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
/// Borde izquierdo del cuadrado, con el mismo aire interior que la casilla.
pub const BADGE_LEFT: f32 = SIDEBAR_INSET + SIDEBAR_MARGIN;
/// Tamaño del nombre de cada sección.
pub const ROW_TEXT_SIZE: f32 = 13.0;
/// Tamaño del glifo Lucide blanco sobre el cuadrado.
pub const ROW_GLYPH_SIZE: f32 = dokka_scene::DRAWING_SIDE;
/// Lado del glifo blanco dentro del cuadrado.
pub const ROW_ICON_SIDE: f32 = 18.0;
/// Radio del cuadrado de color en el lienzo de 100, para pintarlo como
/// imagen y que al filtrar cada capa repinte su color (el marcador de un
/// icono no cambia al actualizarse).
pub const BADGE_CANVAS_RADIUS: f32 = 25.0;
/// Margen de la fila resaltada dentro de la barra.
pub const ROW_HIGHLIGHT_MARGIN: f32 = 8.0;
/// Alto de la fila resaltada (selección y hover).
pub const ROW_HIGHLIGHT_HEIGHT: f32 = 36.0;
/// Esquinas de la fila resaltada, concéntricas con el panel:
/// radio interior = radio exterior - separación.
pub const ROW_HIGHLIGHT_RADIUS: f32 = SIDEBAR_RADIUS - ROW_HIGHLIGHT_MARGIN;

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
pub const CARD_RADIUS: f32 = 16.0;
pub const CARD_BORDER_WIDTH: f32 = 1.0;
pub const CARD_TEXT_SIZE: f32 = 14.0;
/// Texto de la tarjeta, único contenido por ahora.
pub const CARD_TEXT: &str = "Próximamente";

/// Contenido real estilo Tahoe: tarjetas claras con filas separadas por
/// líneas finas, etiqueta a la izquierda y control a la derecha. La tarjeta
/// usa el mismo radio y borde que `CARD_*` para no reconstruir capas al
/// cambiar de sección. Radios concéntricos: interior = exterior − separación.
/// Borde izquierdo del contenido, tras la barra lateral.
pub const CONTENT_LEFT: f32 = SIDEBAR_WIDTH + CARD_MARGIN;
/// Ancho del contenido, con el mismo margen a la derecha.
pub const CONTENT_WIDTH: f32 = WIDTH - CONTENT_LEFT - CARD_MARGIN;
/// Borde superior del contenido, como la tarjeta antigua.
pub const CONTENT_TOP: f32 = CARD_TOP;
/// Alto de cada fila de ajustes dentro de su tarjeta.
pub const CONTENT_ROW_HEIGHT: f32 = 48.0;
/// Aire interior de la tarjeta de contenido a izquierda y derecha.
pub const CONTENT_PADDING: f32 = 16.0;
/// Tamaño del texto de cada fila de ajustes.
pub const CONTENT_LABEL_SIZE: f32 = 13.0;
/// Tamaño del texto atenuado (fila con el control apagado).
pub const CONTENT_DIM_SIZE: f32 = 13.0;
/// Ancho del interruptor Tahoe.
pub const SWITCH_WIDTH: f32 = 46.0;
/// Alto del interruptor Tahoe.
pub const SWITCH_HEIGHT: f32 = 26.0;
/// Diámetro del tirador redondo del interruptor.
pub const SWITCH_KNOB: f32 = 22.0;
/// Aire del tirador al borde de la pista.
pub const SWITCH_INSET: f32 = 2.0;
/// Ancho del deslizador de Preferencias, a la derecha de su fila.
pub const SLIDER_WIDTH: f32 = 168.0;
/// Alto de la zona de toque del deslizador, más alta que la barra.
pub const SLIDER_TOUCH_HEIGHT: f32 = 28.0;
/// Separación entre tarjetas de contenido apiladas.
pub const GROUP_GAP: f32 = 16.0;

/// Secciones en orden, como las ve la barra lateral.
pub const SECTIONS: usize = 8;

/// Borde superior de la fila `index`.
pub fn row_top(index: usize) -> f32 {
    ROWS_TOP + index as f32 * ROW_PITCH
}

/// Centro vertical de la fila `index`.
pub fn row_middle(index: usize) -> f32 {
    row_top(index) + ROW_PITCH / 2.0
}

/// Ancho del panel flotante: el borde derecho sigue en `SIDEBAR_WIDTH`.
pub fn sidebar_panel_width() -> f32 {
    SIDEBAR_WIDTH - SIDEBAR_INSET
}

/// Borde izquierdo de la fila resaltada (selección y hover), dentro del panel.
pub fn row_highlight_left() -> f32 {
    SIDEBAR_INSET + ROW_HIGHLIGHT_MARGIN
}

/// Ancho de la casilla de búsqueda dentro del panel flotante.
pub fn search_width() -> f32 {
    sidebar_panel_width() - SIDEBAR_MARGIN * 2.0
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
