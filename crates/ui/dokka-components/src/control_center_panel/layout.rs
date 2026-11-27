//! Posiciones del Centro de control, compartidas por el dibujo y el hit.
//!
//! Rejilla de dos columnas sobre el rectángulo total: a la izquierda dos
//! cápsulas (Red y Preferencias), a la derecha la música cuadrada; debajo
//! tres círculos repartidos a todo lo ancho y la tarjeta de Sonido. Los
//! huecos usan `tray_glass::GAP` y el aire exterior `tray_glass::PADDING`.

use dokka_scene::Rect;
use dokka_theme::{control_center as measures, tray_glass as glass};

/// Ancho útil una vez quitado el aire exterior.
fn content_width(bounds: Rect) -> f32 {
    bounds.width - glass::PADDING * 2.0
}

fn content_x(bounds: Rect) -> f32 {
    bounds.x + glass::PADDING
}

fn top_y(bounds: Rect) -> f32 {
    bounds.y + glass::PADDING
}

/// Alto de la fila superior: dos cápsulas + hueco, igual que la música.
fn top_height() -> f32 {
    measures::MUSIC_WIDTH
}

fn circles_y(bounds: Rect) -> f32 {
    top_y(bounds) + top_height() + glass::GAP
}

/// Cápsula de red: primera de la columna izquierda.
pub(crate) fn network_capsule(bounds: Rect) -> Rect {
    Rect {
        x: content_x(bounds),
        y: top_y(bounds),
        width: measures::LEFT_WIDTH,
        height: glass::CAPSULE_HEIGHT,
    }
}

/// Cápsula de Preferencias, bajo la de red.
pub(crate) fn prefs_capsule(bounds: Rect) -> Rect {
    let above = network_capsule(bounds);
    Rect {
        x: above.x,
        y: above.y + above.height + glass::GAP,
        width: above.width,
        height: glass::CAPSULE_HEIGHT,
    }
}

/// Tarjeta cuadrada de música, a la derecha de las cápsulas.
pub(crate) fn music_tile(bounds: Rect) -> Rect {
    Rect {
        x: content_x(bounds) + measures::LEFT_WIDTH + glass::GAP,
        y: top_y(bounds),
        width: measures::MUSIC_WIDTH,
        height: top_height(),
    }
}

/// Botón circular `index` (0 escritorio, 1 bloqueo, 2 tareas), repartidos.
pub(crate) fn round_button(bounds: Rect, index: usize) -> Rect {
    let step =
        (content_width(bounds) - measures::ROUND_DIAMETER) / (measures::ROUND_COUNT as f32 - 1.0);
    Rect {
        x: content_x(bounds) + step * index as f32,
        y: circles_y(bounds),
        width: measures::ROUND_DIAMETER,
        height: measures::ROUND_DIAMETER,
    }
}

/// Tarjeta de Sonido a todo lo ancho, bajo los círculos.
pub(crate) fn sound_card(bounds: Rect) -> Rect {
    Rect {
        x: content_x(bounds),
        y: circles_y(bounds) + measures::ROUND_DIAMETER + glass::GAP,
        width: content_width(bounds),
        height: measures::SOUND_HEIGHT,
    }
}

/// Círculo con icono dentro de una cápsula, centrado en alto.
pub(crate) fn capsule_circle(capsule: Rect) -> Rect {
    let inset = (capsule.height - glass::CIRCLE) / 2.0;
    Rect {
        x: capsule.x + inset,
        y: capsule.y + inset,
        width: glass::CIRCLE,
        height: glass::CIRCLE,
    }
}

/// La `x` del título y el subtítulo de una cápsula.
pub(crate) fn capsule_text_x(capsule: Rect) -> f32 {
    capsule_circle(capsule).x + glass::CIRCLE + measures::ICON_TEXT_GAP
}

/// Carátula marcadora dentro de la música, arriba a la izquierda.
pub(crate) fn music_art(card: Rect) -> Rect {
    Rect {
        x: card.x + glass::PADDING,
        y: card.y + glass::PADDING,
        width: measures::MUSIC_ART_SIDE,
        height: measures::MUSIC_ART_SIDE,
    }
}

/// Botón de música `index` (0 anterior, 1 pausa, 2 siguiente).
pub(crate) fn music_button(card: Rect, index: usize) -> Rect {
    let total = measures::MUSIC_BUTTON_SIDE * 3.0 + measures::MUSIC_BUTTON_GAP * 2.0;
    Rect {
        x: card.x
            + (card.width - total) / 2.0
            + (measures::MUSIC_BUTTON_SIDE + measures::MUSIC_BUTTON_GAP) * index as f32,
        y: card.y + card.height - glass::PADDING - measures::MUSIC_BUTTON_SIDE,
        width: measures::MUSIC_BUTTON_SIDE,
        height: measures::MUSIC_BUTTON_SIDE,
    }
}

/// Centro vertical de la pista del deslizador dentro de su tarjeta.
fn track_center_y(card: Rect) -> f32 {
    card.y + measures::TRACK_CENTER_TOP
}

/// Icono pequeño del extremo izquierdo del deslizador.
pub(crate) fn sound_left_icon(card: Rect) -> Rect {
    Rect {
        x: card.x + glass::PADDING,
        y: track_center_y(card) - measures::END_ICON_SIDE / 2.0,
        width: measures::END_ICON_SIDE,
        height: measures::END_ICON_SIDE,
    }
}

/// Caja del silencio: icono pequeño a la derecha, con aire para pulsarlo.
pub(crate) fn sound_mute_box(card: Rect) -> Rect {
    Rect {
        x: card.x + card.width - glass::PADDING - measures::MUTE_HIT_SIDE,
        y: track_center_y(card) - measures::MUTE_HIT_SIDE / 2.0,
        width: measures::MUTE_HIT_SIDE,
        height: measures::MUTE_HIT_SIDE,
    }
}

/// Pista fina del deslizador, entre los dos iconos.
pub(crate) fn slider_track(card: Rect) -> Rect {
    let left = sound_left_icon(card);
    let mute = sound_mute_box(card);
    let x = left.x + left.width + measures::ICON_TEXT_GAP;
    let end = mute.x - measures::ICON_TEXT_GAP;
    Rect {
        x,
        y: track_center_y(card) - glass::TRACK_HEIGHT / 2.0,
        width: (end - x).max(0.0),
        height: glass::TRACK_HEIGHT,
    }
}

/// Zona pulsable del deslizador, con aire vertical para no exigir puntería.
pub(crate) fn slider_area(card: Rect) -> Rect {
    let track = slider_track(card);
    Rect {
        x: track.x - 4.0,
        y: track.y + track.height / 2.0 - measures::SLIDER_HIT_HALF,
        width: track.width + 8.0,
        height: measures::SLIDER_HIT_HALF * 2.0,
    }
}

/// Valor del deslizador (0 a 1) para un punto `x` sobre la pista.
pub(crate) fn slider_value(track: Rect, x: f32) -> f32 {
    ((x - track.x) / track.width).clamp(0.0, 1.0)
}
