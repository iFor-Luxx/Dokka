//! Medidas del Centro de control estilo Tahoe.
//!
//! Rejilla de cuatro columnas: celda cuadrada de lado `CELL` con hueco
//! `GAP` igual en horizontal y vertical, ancho total `4 * CELL + 3 * GAP`.
//! Un círculo mide `CELL`; la cápsula ancha mide `2 * CELL + GAP` de ancho y
//! `CELL` de alto; la música mide `(2 * CELL + GAP)` por `(2 * CELL + GAP)`;
//! el Sonido y la Pantalla miden el ancho total con su alto propio. El alto
//! total lo calcula el reparto (`layout`) según los visibles: suma filas,
//! huecos y aire exterior.

use dokka_scene::DRAWING_SIDE;

/// Lado de la celda cuadrada (≈ 66).
pub const CELL: f32 = 66.0;
/// Hueco igual en horizontal y vertical (≈ 12).
pub const GAP: f32 = 12.0;
/// Aire exterior en los cuatro lados, igual que el hueco.
pub const PADDING: f32 = 12.0;
/// Ancho fijo del panel: rejilla más aire exterior.
pub const WIDTH: f32 = CELL * 4.0 + GAP * 3.0 + PADDING * 2.0;
/// Segundos sin uso con el cursor fuera antes de cerrar solo el centro.
/// El shell cuenta un `Tick` de 1 s por segundo; la isla decide.
pub const IDLE_CLOSE_SECONDS: u32 = 6;
/// Retardo entre piezas al cerrarse: primero las más cercanas al icono de la
/// barra, unas a otras cada ~25 ms, estilo Tahoe.
pub const DISMISS_STAGGER_SECONDS: f32 = 0.025;
/// Cuánto tarda cada pieza en encogerse, desvanecerse y volar a su icono.
/// Con el escalonado, el conjunto termina como mucho en `DISMISS_TOTAL_SECONDS`.
pub const DISMISS_PIECE_SECONDS: f32 = 0.22;
/// Tope total de la salida por piezas: la isla hace el cierre real al llegar
/// aquí, sin repintar por fotograma (las anima Composition).
pub const DISMISS_TOTAL_SECONDS: f32 = 0.45;
/// Escala final de cada pieza al llegar a su icono: se encoge hacia él.
pub const DISMISS_END_SCALE: f32 = 0.2;
/// Alto de la tarjeta de Sonido a todo lo ancho (≈ 68).
pub const SOUND_HEIGHT: f32 = 68.0;
/// Alto de la tarjeta de Pantalla: exactamente igual que la de Sonido.
pub const BRIGHTNESS_HEIGHT: f32 = SOUND_HEIGHT;
/// Esquinas del fondo invisible, como los demás paneles flotantes.
pub const CORNER_RADIUS: f32 = 16.0;

/// Sombra suave bajo cada pieza, para que se distinga sobre un fondo blanco.
pub const SHADOW_BLUR: f32 = 10.0;
/// La sombra cae 2 hacia abajo: desenfoque + caída = 12, igual que `PADDING`,
/// así cabe en el aire exterior sin que la recorte la ventana del panel.
pub const SHADOW_OFFSET_Y: f32 = 2.0;

/// Lado del círculo interior de una cápsula (≈ `CELL - 24`).
pub const CAPSULE_CIRCLE_SIDE: f32 = CELL - 24.0;
/// Margen igual arriba, abajo e izquierda dentro de la cápsula.
pub const CAPSULE_MARGIN: f32 = (CELL - CAPSULE_CIRCLE_SIDE) / 2.0;
/// Título 10 puntos por encima del centro y subtítulo 10 por debajo.
pub const CAPSULE_TEXT_HALF: f32 = 10.0;
/// Título y subtítulo dentro de las cápsulas: un punto menos que la música
/// para que quepan tras el círculo.
pub const CAPSULE_TITLE_SIZE: f32 = 12.0;
pub const CAPSULE_SUBTITLE_SIZE: f32 = 11.0;

/// Lado de la carátula marcadora dentro de la música.
pub const MUSIC_ART_SIDE: f32 = 36.0;
/// Esquinas de la carátula, menores que su mitad.
pub const ART_RADIUS: f32 = 10.0;
/// Título y artista bajo la carátula, desde lo alto de la tarjeta.
pub const MUSIC_TITLE_TOP: f32 = 66.0;
pub const MUSIC_SUBTITLE_TOP: f32 = 82.0;
/// Lado de cada botón de música (anterior, pausa, siguiente).
pub const MUSIC_BUTTON_SIDE: f32 = 32.0;
/// Separación entre botones de música seguidos.
pub const MUSIC_BUTTON_GAP: f32 = 4.0;
/// Lado del glifo dentro de cada botón de música.
pub const MUSIC_ICON_SIDE: f32 = 16.0;

/// Título del deslizador, centro vertical desde lo alto de su tarjeta.
pub const SLIDER_TITLE_TOP: f32 = 22.0;
/// Centro de la pista, distancia desde abajo: barra en la parte baja.
pub const SLIDER_TRACK_BOTTOM: f32 = 20.0;
/// Lado del icono pequeño de cada extremo del deslizador.
pub const END_ICON_SIDE: f32 = 14.0;
/// Lado de la zona pulsable del silencio, alrededor de su icono.
pub const MUTE_HIT_SIDE: f32 = 28.0;
/// Margen del icono visible de cada extremo al borde de la tarjeta: el
/// derecho termina a la misma distancia que empieza el izquierdo.
pub const SOUND_ICON_MARGIN: f32 = PADDING;
/// Aire vertical del deslizador para no exigir puntería fina.
pub const SLIDER_HIT_HALF: f32 = 14.0;

/// Separación entre el círculo y el texto de la misma cápsula.
pub const ICON_TEXT_GAP: f32 = 8.0;
/// Lado del glifo Lucide dentro de cada círculo (≈ 22), centrado.
pub const CIRCLE_ICON_SIDE: f32 = 19.0;

/// Letras máximas antes de recortar un nombre o título con "…".
pub const TEXT_CHARACTERS: usize = 16;
/// Letras máximas en las cápsulas antes de recortar con "…": "Preferencias",
/// "Desconectada", "Sin internet" y "Sin conexión" tienen 12 y salen enteras.
pub const CAPSULE_TEXT_CHARACTERS: usize = 12;
/// Tamaño del glifo Lucide: llena el lienzo de 100.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Cuánto se desplazan las copias que engrosan cada glifo, en el lienzo de 100.
pub const BOLD_SPREAD: f32 = 2.6;

/// Burbuja "Editar controles" bajo la rejilla, solo sin editar: hueco sobre
/// ella igual al aire exterior y alto justo para que el centro lleno con
/// brillo (484) más hueco y burbuja (524) quepa en la ventana de 560
/// (30 de franja + 6 de separación + 524 = 560).
pub const EDIT_GAP: f32 = 12.0;
/// Ancho fijo de la burbuja, centrada a lo ancho del panel.
pub const EDIT_BUBBLE_WIDTH: f32 = 168.0;
/// Alto de la burbuja de cristal.
pub const EDIT_BUBBLE_HEIGHT: f32 = 28.0;
/// Texto de la burbuja, un punto menos que los títulos para que quepa.
pub const EDIT_TEXT_SIZE: f32 = 12.0;

/// Botón redondo "−" de cada pieza en modo editar, centrado en su esquina
/// superior izquierda (sobresale de la pieza pero queda dentro del panel,
/// a 1 del borde con el aire exterior de 12).
pub const REMOVE_DIAMETER: f32 = 22.0;
/// Grosor de la línea del menos, como rectángulo plano (no es texto).
pub const REMOVE_LINE_WIDTH: f32 = 10.0;
pub const REMOVE_LINE_HEIGHT: f32 = 2.0;
