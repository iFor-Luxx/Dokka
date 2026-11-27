//! Medidas del Centro de control estilo Tahoe.
//!
//! Rejilla de dos columnas sobre 320 de ancho: a la izquierda dos cápsulas
//! de 156 × 60 (Red y Preferencias), a la derecha música cuadrada de
//! 130 × 130; debajo tres círculos de 60 repartidos a todo lo ancho y la
//! tarjeta de Sonido de 296 × 84. Los huecos (10) y el aire exterior (12)
//! salen de `tray_glass`; aquí solo van los tamaños propios. El alto total
//! es 318 = 12 + 130 + 10 + 60 + 10 + 84 + 12.

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo del panel, como el resto de la barra.
pub const WIDTH: f32 = 320.0;
/// Alto fijo: exterior + música + hueco + círculos + hueco + sonido + exterior.
pub const HEIGHT: f32 = 318.0;
/// Esquinas del fondo invisible, como los demás paneles flotantes.
pub const CORNER_RADIUS: f32 = 16.0;

/// Ancho de cada cápsula izquierda (296 − 10 − 130).
pub const LEFT_WIDTH: f32 = 156.0;
/// Lado de la tarjeta cuadrada de música (dos cápsulas + hueco).
pub const MUSIC_WIDTH: f32 = 130.0;

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

/// Botones circulares bajo la rejilla (Escritorio, Bloqueo, Tareas).
pub const ROUND_COUNT: usize = 3;
/// Diámetro de cada círculo, igual que la cápsula.
pub const ROUND_DIAMETER: f32 = 60.0;

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
pub const TEXT_CHARACTERS: usize = 24;
/// Tamaño del glifo Lucide: llena el lienzo de 100.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
