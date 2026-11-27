//! Posiciones del Centro de control, compartidas por el dibujo y el hit.

use dokka_scene::Rect;
use dokka_theme::control_center as measures;

/// Tarjeta de red: primera del panel.
pub(crate) fn network_card(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: bounds.y + measures::PADDING,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::NETWORK_HEIGHT,
    }
}

/// Tarjeta de volumen, bajo la de red.
pub(crate) fn volume_card(bounds: Rect) -> Rect {
    let above = network_card(bounds);
    Rect {
        x: above.x,
        y: above.y + above.height + measures::GAP,
        width: above.width,
        height: measures::VOLUME_HEIGHT,
    }
}

/// Tarjeta de música, bajo la de volumen.
pub(crate) fn music_card(bounds: Rect) -> Rect {
    let above = volume_card(bounds);
    Rect {
        x: above.x,
        y: above.y + above.height + measures::GAP,
        width: above.width,
        height: measures::MUSIC_HEIGHT,
    }
}

/// Botón redondo `index` (0 Preferencias, 1 escritorio, 2 bloqueo, 3 tareas).
pub(crate) fn round_button(bounds: Rect, index: usize) -> Rect {
    let music = music_card(bounds);
    let y = music.y + music.height + measures::GAP;
    let step = (bounds.width - measures::PADDING * 2.0 - measures::BUTTON_DIAMETER) / 3.0;
    Rect {
        x: bounds.x + measures::PADDING + step * index as f32,
        y,
        width: measures::BUTTON_DIAMETER,
        height: measures::BUTTON_DIAMETER,
    }
}

/// Icono de red o silencio: cuadrado de 20 con aire de la tarjeta.
pub(crate) fn card_icon(card: Rect) -> Rect {
    Rect {
        x: card.x + measures::TEXT_INSET,
        y: card.y + (card.height - measures::ICON_SIDE) / 2.0,
        width: measures::ICON_SIDE,
        height: measures::ICON_SIDE,
    }
}

/// Zona pulsable del silencio, alrededor de su icono.
pub(crate) fn mute_area(card: Rect) -> Rect {
    let icon = card_icon(card);
    let pad = (measures::MUTE_HIT_SIDE - measures::ICON_SIDE) / 2.0;
    Rect {
        x: icon.x - pad,
        y: icon.y - pad,
        width: measures::MUTE_HIT_SIDE,
        height: measures::MUTE_HIT_SIDE,
    }
}

/// Pista del deslizador de volumen, dentro de su tarjeta.
pub(crate) fn slider_track(card: Rect) -> Rect {
    let height = measures::TRACK_HEIGHT;
    Rect {
        x: card.x + measures::TEXT_INSET,
        y: card.y + card.height - measures::TRACK_BOTTOM - height / 2.0,
        width: card.width - measures::TEXT_INSET * 2.0,
        height,
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

/// Botón de música `index` (0 anterior, 1 pausa, 2 siguiente).
pub(crate) fn music_button(card: Rect, index: usize) -> Rect {
    Rect {
        x: card.x
            + measures::TEXT_INSET
            + (measures::MUSIC_BUTTON_WIDTH + measures::MUSIC_BUTTON_GAP) * index as f32,
        y: card.y + card.height - measures::PADDING - measures::MUSIC_BUTTON_HEIGHT,
        width: measures::MUSIC_BUTTON_WIDTH,
        height: measures::MUSIC_BUTTON_HEIGHT,
    }
}
