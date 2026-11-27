//! Medidas del Centro de control estilo Tahoe.
//!
//! Rejilla de cuatro columnas sobre 320 de ancho: filas 1-2 con la cápsula
//! de red y dos círculos a la izquierda y la música a la derecha (dos
//! columnas por dos filas); fila 3 con la cápsula de Preferencias y un
//! círculo; tarjeta de Sonido a todo lo ancho. La columna sale de
//! `(296 − 30) / 4 = 66.5`; la cápsula ancha son dos columnas más un hueco.
//! El alto total es 324.5 = 12 + 136.5 + 10 + 60 + 10 + 84 + 12.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo del panel, como el resto de la barra.
pub const WIDTH: f32 = 320.0;
/// Alto fijo: exterior + bloque superior + hueco + fila 3 + hueco + sonido.
pub const HEIGHT: f32 = 324.5;
/// Esquinas del fondo invisible, como los demás paneles flotantes.
pub const CORNER_RADIUS: f32 = 16.0;

/// Lado de la carátula marcadora dentro de la música.
pub const MUSIC_ART_SIDE: f32 = 36.0;
/// Esquinas de la carátula, menores que su mitad.
pub const ART_RADIUS: f32 = 10.0;
/// Lado de cada botón de música (anterior, pausa, siguiente).
pub const MUSIC_BUTTON_SIDE: f32 = 32.0;
/// Separación entre botones de música seguidos.
pub const MUSIC_BUTTON_GAP: f32 = 4.0;
/// Lado del glifo dentro de cada botón de música.
pub const MUSIC_ICON_SIDE: f32 = 20.0;

/// Alto de la tarjeta de Sonido a todo lo ancho.
pub const SOUND_HEIGHT: f32 = 84.0;
/// Del borde superior de la tarjeta al centro de su pista.
pub const TRACK_CENTER_TOP: f32 = 56.0;
/// Lado del icono pequeño de cada extremo del deslizador.
pub const END_ICON_SIDE: f32 = 16.0;
/// Lado de la zona pulsable del silencio, alrededor de su icono.
pub const MUTE_HIT_SIDE: f32 = 28.0;
/// Aire vertical del deslizador para no exigir puntería fina.
pub const SLIDER_HIT_HALF: f32 = 14.0;

/// Separación entre el círculo y el texto de la misma cápsula.
pub const ICON_TEXT_GAP: f32 = 8.0;

/// Letras máximas antes de recortar un nombre o título con "…".
pub const TEXT_CHARACTERS: usize = 16;
/// Tamaño del glifo Lucide: llena el lienzo de 100.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
