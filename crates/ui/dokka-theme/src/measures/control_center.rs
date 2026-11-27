//! Medidas del Centro de control estilo Tahoe.
//!
//! Rejilla estricta de cuatro columnas: celda cuadrada de lado `CELL` con
//! hueco `GAP` igual en horizontal y vertical, ancho total `4 * CELL + 3 * GAP`.
//! Un círculo mide `CELL`; la cápsula ancha mide `2 * CELL + GAP` de ancho y
//! `CELL` de alto; la música mide `(2 * CELL + GAP)` por `(2 * CELL + GAP)`;
//! el Sonido mide el ancho total y `SOUND_HEIGHT` de alto.
//! Fila 1: cápsula Red + música (filas 1 y 2). Fila 2: dos círculos.
//! Fila 3: cápsula Preferencias + dos círculos. Fila 4: Sonido.
//! Fila 5: cuatro círculos. El alto total suma filas, huecos y aire exterior.

use dokka_scene::DRAWING_SIDE;

/// Lado de la celda cuadrada (≈ 66).
pub const CELL: f32 = 66.0;
/// Hueco igual en horizontal y vertical (≈ 12).
pub const GAP: f32 = 12.0;
/// Aire exterior en los cuatro lados, igual que el hueco.
pub const PADDING: f32 = 12.0;
/// Ancho de una pieza doble: dos celdas más el hueco entre ellas.
pub const SPAN: f32 = CELL * 2.0 + GAP;
/// Ancho fijo del panel: rejilla más aire exterior.
pub const WIDTH: f32 = CELL * 4.0 + GAP * 3.0 + PADDING * 2.0;
/// Alto de la tarjeta de Sonido a todo lo ancho (≈ 68).
pub const SOUND_HEIGHT: f32 = 68.0;
/// Alto fijo: aire + cuatro celdas + Sonido + cuatro huecos entre cinco filas.
pub const HEIGHT: f32 = PADDING * 2.0 + CELL * 4.0 + SOUND_HEIGHT + GAP * 4.0;
/// Esquinas del fondo invisible, como los demás paneles flotantes.
pub const CORNER_RADIUS: f32 = 16.0;

/// Lado del círculo interior de una cápsula (≈ `CELL - 24`).
pub const CAPSULE_CIRCLE_SIDE: f32 = CELL - 24.0;
/// Margen igual arriba, abajo e izquierda dentro de la cápsula.
pub const CAPSULE_MARGIN: f32 = (CELL - CAPSULE_CIRCLE_SIDE) / 2.0;
/// Título 10 puntos por encima del centro y subtítulo 10 por debajo.
pub const CAPSULE_TEXT_HALF: f32 = 10.0;

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
/// Aire vertical del deslizador para no exigir puntería fina.
pub const SLIDER_HIT_HALF: f32 = 14.0;

/// Separación entre el círculo y el texto de la misma cápsula.
pub const ICON_TEXT_GAP: f32 = 8.0;
/// Lado del glifo Lucide dentro de cada círculo (≈ 22), centrado.
pub const CIRCLE_ICON_SIDE: f32 = 19.0;

/// Letras máximas antes de recortar un nombre o título con "…".
pub const TEXT_CHARACTERS: usize = 16;
/// Tamaño del glifo Lucide: llena el lienzo de 100.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Cuánto se desplazan las copias que engrosan cada glifo, en el lienzo de 100.
pub const BOLD_SPREAD: f32 = 2.6;
