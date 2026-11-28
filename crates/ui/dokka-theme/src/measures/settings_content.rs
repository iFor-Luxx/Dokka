//! Medidas del contenido de Preferencias: grupos, filas y controles.

use super::settings::{CARD_TOP, HEADER_DIVIDER_LEFT, HEADER_DIVIDER_RIGHT};

/// Contenido real estilo Tahoe: tarjetas claras con filas separadas por
/// líneas finas, etiqueta a la izquierda y control a la derecha. La tarjeta
/// usa el mismo radio y borde que `CARD_*` para no reconstruir capas al
/// cambiar de sección. Radios concéntricos: interior = exterior − separación.
/// Borde izquierdo del contenido, alineado con la cabecera
/// (`HEADER_DIVIDER_LEFT`) para que las tarjetas empiecen bajo la píldora.
pub const CONTENT_LEFT: f32 = HEADER_DIVIDER_LEFT;
/// Ancho del contenido, alineado con la cabecera: de su borde izquierdo a su
/// borde derecho para que las tarjetas queden bajo su línea.
pub const CONTENT_WIDTH: f32 = HEADER_DIVIDER_RIGHT - HEADER_DIVIDER_LEFT;
/// Borde superior del contenido, como la tarjeta antigua.
pub const CONTENT_TOP: f32 = CARD_TOP;
/// Alto de cada fila de ajustes dentro de su tarjeta.
pub const CONTENT_ROW_HEIGHT: f32 = 44.0;
/// Alto de la fila del deslizador, con sus textos bajo la barra.
pub const CONTENT_SLIDER_ROW_HEIGHT: f32 = 56.0;
/// Aire interior de la tarjeta de contenido a izquierda y derecha.
pub const CONTENT_PADDING: f32 = 16.0;
/// Tamaño del texto de cada fila de ajustes.
pub const CONTENT_LABEL_SIZE: f32 = 13.0;
/// Ancho del interruptor Tahoe.
pub const SWITCH_WIDTH: f32 = 40.0;
/// Alto del interruptor Tahoe.
pub const SWITCH_HEIGHT: f32 = 22.0;
/// Tirador alargado del interruptor, en cápsula blanca.
pub const SWITCH_KNOB_WIDTH: f32 = 24.0;
/// Alto del tirador alargado del interruptor.
pub const SWITCH_KNOB_HEIGHT: f32 = 18.0;
/// Aire del tirador al borde de la pista.
pub const SWITCH_INSET: f32 = 2.0;
/// Ancho del deslizador de Preferencias, a la derecha de su fila.
pub const SLIDER_WIDTH: f32 = 168.0;
/// Alto de la zona de toque del deslizador, más alta que la barra.
pub const SLIDER_TOUCH_HEIGHT: f32 = 28.0;
/// Grosor de la pista fina del deslizador Tahoe.
pub const SLIDER_TRACK_HEIGHT: f32 = 4.0;
/// Tirador alargado del deslizador, en cápsula blanca.
pub const SLIDER_KNOB_WIDTH: f32 = 22.0;
/// Alto del tirador alargado del deslizador.
pub const SLIDER_KNOB_HEIGHT: f32 = 15.0;
/// Borde muy fino del tirador para verlo sobre la tarjeta.
pub const SLIDER_KNOB_BORDER_WIDTH: f32 = 1.0;
/// Margen superior del área del deslizador dentro de su fila alta.
pub const SLIDER_AREA_TOP: f32 = 12.0;
/// Tamaño de "Pequeño" y "Grande" bajo la barra.
pub const SLIDER_CAPTION_SIZE: f32 = 11.0;
/// Ancho aproximado de "Grande" para alinearlo al borde derecho.
pub const SLIDER_CAPTION_RIGHT_WIDTH: f32 = 34.0;
/// Aire entre la barra y los textos bajo ella.
pub const SLIDER_CAPTION_GAP: f32 = 4.0;
/// Ancho del segmentado de posición, a la derecha de su fila con el mismo
/// aire que el texto a la izquierda. Con tres segmentos de ~68 cada uno
/// "Izquierda" cabe centrado sin salirse.
pub const SEGMENTED_WIDTH: f32 = 204.0;
/// Alto del segmentado Tahoe, centrado en su fila de 44.
pub const SEGMENTED_HEIGHT: f32 = 28.0;
/// Aire interior del segmentado: la pastilla elegida encoge esto por lado,
/// con radio concéntrico (exterior menos separación).
pub const SEGMENTED_INSET: f32 = 2.0;
/// Tamaño del texto de cada segmento, centrado en su tercio.
pub const SEGMENTED_TEXT_SIZE: f32 = 12.0;
/// Ancho de la marca del valor de fábrica en el deslizador.
pub const SLIDER_TICK_WIDTH: f32 = 2.0;
/// Alto de la marca del valor de fábrica, algo más alta que la pista fina.
pub const SLIDER_TICK_HEIGHT: f32 = 10.0;
/// Separación entre tarjetas de contenido apiladas.
pub const GROUP_GAP: f32 = 16.0;
/// Umbral para empezar a reordenar la barra: un clic no mueve nada.
pub const REORDER_THRESHOLD: f32 = 4.0;
/// Lado del agarre de cada fila reordenable.
pub const REORDER_GRIP_SIDE: f32 = 16.0;
/// El glifo llena el lienzo de 100, como el resto de Lucide.
pub const REORDER_GRIP_GLYPH_SIZE: f32 = dokka_scene::DRAWING_SIDE;
/// Aire entre el agarre y el nombre de la fila.
pub const REORDER_GRIP_GAP: f32 = 8.0;
/// Texto pequeño bajo la tarjeta que explica el arrastre.
pub const REORDER_HINT_SIZE: f32 = 12.0;
/// Aire entre la tarjeta y el texto que explica el arrastre.
pub const REORDER_HINT_GAP: f32 = 8.0;
/// Radio del resaltado de la fila arrastrada.
pub const REORDER_HIGHLIGHT_RADIUS: f32 = 8.0;

/// Alto de la tarjeta de cabecera de General: engranaje grande, título y ayuda.
pub const GENERAL_HEADER_HEIGHT: f32 = 176.0;
/// Lado del engranaje grande de la cabecera de General.
pub const GENERAL_GEAR_SIDE: f32 = 56.0;
/// Aire entre el engranaje y el título de General, y entre el título y la ayuda.
pub const GENERAL_HEADER_GAP: f32 = 8.0;
/// Tamaño del título "General" en su tarjeta.
pub const GENERAL_HEADER_TITLE_SIZE: f32 = 20.0;
/// Tamaño de la ayuda bajo el título de General.
pub const GENERAL_HEADER_SUBTITLE_SIZE: f32 = 13.0;
/// Lado del chevrón derecho de las filas de General.
pub const GENERAL_CHEVRON_SIDE: f32 = 16.0;

/// Alto del héroe de "Acerca de este PC": dibujo del PC, modelo y línea gris.
pub const ABOUT_HERO_HEIGHT: f32 = 148.0;
/// Lado del dibujo del PC en la ficha.
pub const ABOUT_PC_SIDE: f32 = 72.0;
/// Tamaño del nombre del modelo en grande.
pub const ABOUT_MODEL_SIZE: f32 = 20.0;
/// Alto de cada cabecera de sección de la ficha ("Windows", "Pantalla"...).
pub const ABOUT_SECTION_TITLE_HEIGHT: f32 = 26.0;
/// Tamaño del texto de cada cabecera de sección.
pub const ABOUT_SECTION_TITLE_SIZE: f32 = 13.0;
/// Alto de cada botón de la ficha.
pub const ABOUT_BUTTON_HEIGHT: f32 = 32.0;
/// Radio de cada botón, concéntrico con las tarjetas.
pub const ABOUT_BUTTON_RADIUS: f32 = 16.0;
/// Aire entre la tarjeta y su botón.
pub const ABOUT_BUTTON_GAP: f32 = 8.0;
/// Ancho fijo de cada botón de la ficha, centrado.
pub const ABOUT_BUTTON_WIDTH: f32 = 220.0;
/// Aire sobre el dibujo del PC dentro del héroe.
pub const ABOUT_HERO_PC_TOP_GAP: f32 = 12.0;
/// Aire entre el dibujo del PC y el modelo.
pub const ABOUT_HERO_MODEL_GAP: f32 = 12.0;
/// Aire entre el modelo y la línea gris.
pub const ABOUT_HERO_LINE_GAP: f32 = 12.0;
/// Mitad del ancho de la línea gris bajo el modelo.
pub const ABOUT_HERO_LINE_HALF: f32 = 60.0;
/// Ancho de la línea gris bajo el modelo.
pub const ABOUT_HERO_LINE_WIDTH: f32 = 120.0;
