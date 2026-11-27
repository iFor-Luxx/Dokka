//! Posiciones del Centro de control, compartidas por el dibujo y el hit.
//!
//! Rejilla de cuatro columnas: la cápsula ancha ocupa dos columnas más un
//! hueco y el círculo una; la música ocupa dos columnas por dos filas. Los
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

/// Ancho de una columna: cuatro columnas y tres huecos en lo útil.
fn column_width(bounds: Rect) -> f32 {
    (content_width(bounds) - glass::GAP * 3.0) / 4.0
}

/// Ancho de una pieza doble: dos columnas más el hueco entre ellas.
fn span_width(bounds: Rect) -> f32 {
    column_width(bounds) * 2.0 + glass::GAP
}

/// Alto del bloque superior: cápsula + hueco + círculo, igual que la música.
fn top_height(bounds: Rect) -> f32 {
    glass::CAPSULE_HEIGHT + glass::GAP + column_width(bounds)
}

fn music_bottom(bounds: Rect) -> f32 {
    top_y(bounds) + top_height(bounds)
}

/// Cápsula de red: fila 1, dos columnas de la izquierda.
pub(crate) fn network_capsule(bounds: Rect) -> Rect {
    Rect {
        x: content_x(bounds),
        y: top_y(bounds),
        width: span_width(bounds),
        height: glass::CAPSULE_HEIGHT,
    }
}

/// Círculo `index`: 0 y 1 bajo la red (fila 2), 2 junto a Preferencias.
pub(crate) fn circle_slot(bounds: Rect, index: usize) -> Rect {
    let side = column_width(bounds);
    let (x, y) = match index {
        0 => (
            content_x(bounds),
            top_y(bounds) + glass::CAPSULE_HEIGHT + glass::GAP,
        ),
        1 => (
            content_x(bounds) + side + glass::GAP,
            top_y(bounds) + glass::CAPSULE_HEIGHT + glass::GAP,
        ),
        _ => (
            content_x(bounds) + span_width(bounds) + glass::GAP,
            music_bottom(bounds) + glass::GAP,
        ),
    };
    Rect {
        x,
        y,
        width: side,
        height: side,
    }
}

/// Tarjeta de música: dos columnas por dos filas, a la derecha.
pub(crate) fn music_tile(bounds: Rect) -> Rect {
    Rect {
        x: content_x(bounds) + span_width(bounds) + glass::GAP,
        y: top_y(bounds),
        width: span_width(bounds),
        height: top_height(bounds),
    }
}

/// Cápsula de Preferencias: fila 3, dos columnas de la izquierda.
pub(crate) fn prefs_capsule(bounds: Rect) -> Rect {
    Rect {
        x: content_x(bounds),
        y: music_bottom(bounds) + glass::GAP,
        width: span_width(bounds),
        height: glass::CAPSULE_HEIGHT,
    }
}

/// Tarjeta de Sonido a todo lo ancho, bajo la fila 3.
pub(crate) fn sound_card(bounds: Rect) -> Rect {
    let prefs = prefs_capsule(bounds);
    Rect {
        x: content_x(bounds),
        y: prefs.y + prefs.height + glass::GAP,
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
