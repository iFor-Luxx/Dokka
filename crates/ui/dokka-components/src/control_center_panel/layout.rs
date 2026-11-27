//! Posiciones del Centro de control, compartidas por el dibujo y el hit.
//!
//! Rejilla estricta de cuatro columnas: celda cuadrada de lado `CELL` con
//! hueco `GAP` igual en horizontal y vertical. La cápsula ancha ocupa dos
//! celdas más un hueco y la música dos por dos. Filas: Red + música,
//! dos círculos, Preferencias + dos círculos, Sonido y cuatro círculos.

use dokka_scene::Rect;
use dokka_theme::control_center as measures;

/// Borde izquierdo de la rejilla, tras el aire exterior.
fn content_x(bounds: Rect) -> f32 {
    bounds.x + measures::PADDING
}

/// Borde superior de la rejilla, tras el aire exterior.
fn top_y(bounds: Rect) -> f32 {
    bounds.y + measures::PADDING
}

/// Borde superior de la fila `row` (0 a 4): filas 0-2 de alto `CELL`,
/// fila 3 de alto `SOUND_HEIGHT` y fila 4 de alto `CELL`.
fn row_y(bounds: Rect, row: usize) -> f32 {
    let top = top_y(bounds);
    match row {
        0 => top,
        1 => top + measures::CELL + measures::GAP,
        2 => top + (measures::CELL + measures::GAP) * 2.0,
        3 => top + (measures::CELL + measures::GAP) * 3.0,
        _ => top + (measures::CELL + measures::GAP) * 3.0 + measures::SOUND_HEIGHT + measures::GAP,
    }
}

/// Borde izquierdo de la columna `column` (0 a 3).
fn column_x(bounds: Rect, column: usize) -> f32 {
    content_x(bounds) + column as f32 * (measures::CELL + measures::GAP)
}

/// Cápsula de red: fila 1, columnas 1 y 2.
pub(crate) fn network_capsule(bounds: Rect) -> Rect {
    Rect {
        x: column_x(bounds, 0),
        y: row_y(bounds, 0),
        width: measures::SPAN,
        height: measures::CELL,
    }
}

/// Círculo `index`: 0 Escritorio y 1 Bloquear (fila 2);
/// 2 Vista de tareas y 3 Proyectar (fila 3);
/// 4 Calculadora, 5 Recorte, 6 Reloj y 7 Tareas (fila 5).
pub(crate) fn circle_slot(bounds: Rect, index: usize) -> Rect {
    let (column, row) = match index {
        0 => (0, 1),
        1 => (1, 1),
        2 => (2, 2),
        3 => (3, 2),
        4 => (0, 4),
        5 => (1, 4),
        6 => (2, 4),
        _ => (3, 4),
    };
    Rect {
        x: column_x(bounds, column),
        y: row_y(bounds, row),
        width: measures::CELL,
        height: measures::CELL,
    }
}

/// Tarjeta de música: dos columnas por dos filas, a la derecha.
pub(crate) fn music_tile(bounds: Rect) -> Rect {
    Rect {
        x: column_x(bounds, 2),
        y: row_y(bounds, 0),
        width: measures::SPAN,
        height: measures::SPAN,
    }
}

/// Cápsula de Preferencias: fila 3, columnas 1 y 2.
pub(crate) fn prefs_capsule(bounds: Rect) -> Rect {
    Rect {
        x: column_x(bounds, 0),
        y: row_y(bounds, 2),
        width: measures::SPAN,
        height: measures::CELL,
    }
}

/// Tarjeta de Sonido a todo lo ancho, fila 4.
pub(crate) fn sound_card(bounds: Rect) -> Rect {
    Rect {
        x: content_x(bounds),
        y: row_y(bounds, 3),
        width: measures::CELL * 4.0 + measures::GAP * 3.0,
        height: measures::SOUND_HEIGHT,
    }
}

/// Círculo con icono dentro de una cápsula, con margen igual arriba,
/// abajo e izquierda.
pub(crate) fn capsule_circle(capsule: Rect) -> Rect {
    Rect {
        x: capsule.x + measures::CAPSULE_MARGIN,
        y: capsule.y + measures::CAPSULE_MARGIN,
        width: measures::CAPSULE_CIRCLE_SIDE,
        height: measures::CAPSULE_CIRCLE_SIDE,
    }
}

/// La `x` del título y el subtítulo de una cápsula.
pub(crate) fn capsule_text_x(capsule: Rect) -> f32 {
    capsule_circle(capsule).x + measures::CAPSULE_CIRCLE_SIDE + measures::ICON_TEXT_GAP
}

/// Carátula marcadora dentro de la música, arriba a la izquierda.
pub(crate) fn music_art(card: Rect) -> Rect {
    Rect {
        x: card.x + measures::PADDING,
        y: card.y + measures::PADDING,
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
        y: card.y + card.height - measures::PADDING - measures::MUSIC_BUTTON_SIDE,
        width: measures::MUSIC_BUTTON_SIDE,
        height: measures::MUSIC_BUTTON_SIDE,
    }
}

/// Centro vertical de la pista del deslizador, en la parte baja.
fn track_center_y(card: Rect) -> f32 {
    card.y + card.height - measures::SLIDER_TRACK_BOTTOM
}

/// Icono pequeño del extremo izquierdo del deslizador.
pub(crate) fn sound_left_icon(card: Rect) -> Rect {
    Rect {
        x: card.x + measures::PADDING,
        y: track_center_y(card) - measures::END_ICON_SIDE / 2.0,
        width: measures::END_ICON_SIDE,
        height: measures::END_ICON_SIDE,
    }
}

/// Caja del silencio: icono pequeño a la derecha, con aire para pulsarlo.
/// Se conserva para el golpe; lo visible va en `sound_mute_icon`.
pub(crate) fn sound_mute_box(card: Rect) -> Rect {
    Rect {
        x: card.x + card.width - measures::PADDING - measures::MUTE_HIT_SIDE,
        y: track_center_y(card) - measures::MUTE_HIT_SIDE / 2.0,
        width: measures::MUTE_HIT_SIDE,
        height: measures::MUTE_HIT_SIDE,
    }
}

/// Icono visible del silencio: termina a la misma distancia del borde que
/// empieza el izquierdo (`SOUND_ICON_MARGIN`), centrado en la pista.
pub(crate) fn sound_mute_icon(card: Rect) -> Rect {
    Rect {
        x: card.x + card.width - measures::SOUND_ICON_MARGIN - measures::END_ICON_SIDE,
        y: track_center_y(card) - measures::END_ICON_SIDE / 2.0,
        width: measures::END_ICON_SIDE,
        height: measures::END_ICON_SIDE,
    }
}

/// Pista fina del deslizador, entre los dos iconos visibles para que la barra
/// quede simétrica.
pub(crate) fn slider_track(card: Rect) -> Rect {
    let left = sound_left_icon(card);
    let right = sound_mute_icon(card);
    let x = left.x + left.width + measures::ICON_TEXT_GAP;
    let end = right.x - measures::ICON_TEXT_GAP;
    Rect {
        x,
        y: track_center_y(card) - dokka_theme::tray_glass::TRACK_HEIGHT / 2.0,
        width: (end - x).max(0.0),
        height: dokka_theme::tray_glass::TRACK_HEIGHT,
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
