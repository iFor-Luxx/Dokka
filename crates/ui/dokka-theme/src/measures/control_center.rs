//! Medidas del panel del Centro de control estilo Tahoe.
//!
//! Panel de cristal de 320 × 338 con tres tarjetas planas (red, volumen con
//! su deslizador mínimo y música) y una fila de cuatro botones redondos
//! (Preferencias, escritorio, bloqueo y tareas). Las tarjetas son
//! concéntricas con el panel (radio interior = exterior - separación).

use dokka_scene::DRAWING_SIDE;

/// Ancho fijo del panel.
pub const WIDTH: f32 = 320.0;
/// Alto fijo del panel.
pub const HEIGHT: f32 = 338.0;
/// Aire alrededor de las tarjetas y los botones, dentro del panel.
pub const PADDING: f32 = 12.0;
/// Separación vertical entre tarjetas y la fila de botones.
pub const GAP: f32 = 10.0;
/// Esquinas del panel, como el resto de paneles flotantes.
pub const CORNER_RADIUS: f32 = 16.0;
pub const BORDER_WIDTH: f32 = 1.0;
/// Esquinas de cada tarjeta, concéntricas con las del panel.
pub const CARD_RADIUS: f32 = 12.0;
pub const CARD_BORDER_WIDTH: f32 = 1.0;

/// Alto de la tarjeta de red.
pub const NETWORK_HEIGHT: f32 = 64.0;
/// Alto de la tarjeta de volumen (texto arriba, deslizador abajo).
pub const VOLUME_HEIGHT: f32 = 76.0;
/// Alto de la tarjeta de música (título, artista y botones).
pub const MUSIC_HEIGHT: f32 = 96.0;

/// Diámetro de cada botón redondo de la fila inferior.
pub const BUTTON_DIAMETER: f32 = 48.0;

/// Lado de los iconos oscuros dentro de las tarjetas claras.
pub const ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide: llena el lienzo de 100, fuente efectiva 20 a 20 de lado.
pub const GLYPH_SIZE: f32 = DRAWING_SIDE;
/// Aire entre el borde de la tarjeta y su icono o texto.
pub const TEXT_INSET: f32 = 14.0;
/// Separación entre el icono y el texto de la misma fila.
pub const ICON_TEXT_GAP: f32 = 8.0;

/// Alto de la pista del deslizador de volumen.
pub const TRACK_HEIGHT: f32 = 6.0;
/// Distancia del centro de la pista al borde inferior de su tarjeta.
pub const TRACK_BOTTOM: f32 = 22.0;
/// Esquinas de la pista y su relleno: la mitad del alto.
pub const TRACK_RADIUS: f32 = TRACK_HEIGHT / 2.0;
/// Diámetro del pulgar del deslizador.
pub const THUMB_DIAMETER: f32 = 14.0;
/// Lado de la zona pulsable del silencio, alrededor de su icono de 20.
pub const MUTE_HIT_SIDE: f32 = 28.0;
/// Aire vertical del deslizador para pulsarlo sin puntería fina.
pub const SLIDER_HIT_HALF: f32 = 14.0;

/// Ancho de cada botón de música (anterior, pausa, siguiente).
pub const MUSIC_BUTTON_WIDTH: f32 = 48.0;
/// Alto de cada botón de música.
pub const MUSIC_BUTTON_HEIGHT: f32 = 32.0;
/// Separación entre botones de música seguidos.
pub const MUSIC_BUTTON_GAP: f32 = 4.0;

/// Letras máximas antes de recortar un nombre o título con "…".
pub const TEXT_CHARACTERS: usize = 32;
