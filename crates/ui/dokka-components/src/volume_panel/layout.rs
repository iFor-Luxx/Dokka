//! Posiciones del panel de volumen, compartidas por el dibujo, el golpe y
//! el arrastre.
//!
//! Una columna: título con porcentaje, fila del deslizador (altavoz bajo,
//! barra fina y altavoz con ondas), sección de salidas si hay, dos líneas
//! finas y dos filas compactas (silenciar y ajustes). El valor siempre sale
//! de la barra, más corta que la zona de toque por los iconos de sus extremos.

use dokka_model::AudioOutput;
use dokka_scene::Rect;
use dokka_theme::{tray_glass as glass, volume as measures};

/// Fila del título con el porcentaje, arriba a la izquierda y a la derecha.
pub(crate) fn title_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y + measures::PADDING,
        width: bounds.width,
        height: measures::TITLE_HEIGHT,
    }
}

/// Fila del deslizador, más alta que la barra para pulsar con comodidad.
pub(crate) fn slider_touch(bounds: Rect) -> Rect {
    let title = title_row(bounds);
    Rect {
        x: bounds.x + glass::ROW_MARGIN + glass::ROW_INSET,
        y: title.y + title.height + measures::TITLE_GAP,
        width: bounds.width - (glass::ROW_MARGIN + glass::ROW_INSET) * 2.0,
        height: measures::SLIDER_ROW_HEIGHT,
    }
}

/// Altavoz bajo a la izquierda de la barra, centrado en la fila.
pub(crate) fn slider_left_icon(touch: Rect) -> Rect {
    Rect {
        x: touch.x,
        y: touch.y + (touch.height - measures::SLIDER_ICON_SIDE) / 2.0,
        width: measures::SLIDER_ICON_SIDE,
        height: measures::SLIDER_ICON_SIDE,
    }
}

/// Altavoz con ondas a la derecha de la barra, centrado en la fila.
pub(crate) fn slider_right_icon(touch: Rect) -> Rect {
    Rect {
        x: touch.x + touch.width - measures::SLIDER_ICON_SIDE,
        y: touch.y + (touch.height - measures::SLIDER_ICON_SIDE) / 2.0,
        width: measures::SLIDER_ICON_SIDE,
        height: measures::SLIDER_ICON_SIDE,
    }
}

/// Barra fina entre los dos altavoces, centrada en vertical.
/// Es la geometría que usan el dibujo, el golpe y el arrastre.
pub(crate) fn slider_bar(touch: Rect) -> Rect {
    let left = slider_left_icon(touch);
    let right = slider_right_icon(touch);
    let x = left.x + left.width + measures::SLIDER_ICON_GAP;
    let end = right.x - measures::SLIDER_ICON_GAP;
    Rect {
        x,
        y: touch.y + (touch.height - measures::TRACK_HEIGHT) / 2.0,
        width: (end - x).max(0.0),
        height: measures::TRACK_HEIGHT,
    }
}

/// Recorrido del centro del tirador: medio diámetro dentro de cada extremo
/// para que no se salga de la barra ni en 0% ni en 100%.
fn knob_travel(bar: Rect) -> (f32, f32) {
    let radius = measures::THUMB_DIAMETER / 2.0;
    if bar.width <= measures::THUMB_DIAMETER {
        let middle = bar.x + bar.width / 2.0;
        return (middle, middle);
    }
    (bar.x + radius, bar.x + bar.width - radius)
}

/// Centro del tirador para el nivel, dentro de la barra.
/// El relleno llega hasta aquí para que ambos coincidan.
pub(crate) fn knob_center(bar: Rect, level: f32) -> f32 {
    let (start, end) = knob_travel(bar);
    start + level01(level) * (end - start)
}

/// Nivel que corresponde a `x` sobre la barra: se queda en 0 o en 1 aunque
/// el cursor se pase. Fuera de la barra no hay recorrido que repartir.
pub(crate) fn slider_value_at(bar: Rect, x: f32) -> f32 {
    let (start, end) = knob_travel(bar);
    if end <= start {
        return if x < bar.x + bar.width / 2.0 {
            0.0
        } else {
            1.0
        };
    }
    level01((x - start) / (end - start))
}

/// Recorta el nivel a 0-1: el silencio (`NaN`) queda en 0.
pub(crate) fn level01(value: f32) -> f32 {
    if value.is_nan() || value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

/// Salidas visibles de la lista (como máximo seis).
pub(crate) fn visible_count(outputs: &[AudioOutput]) -> usize {
    measures::visible_outputs(outputs.len())
}

/// Título pequeño de la sección de salidas, bajo el deslizador.
pub(crate) fn output_title_row(bounds: Rect) -> Rect {
    let touch = slider_touch(bounds);
    Rect {
        x: bounds.x,
        y: touch.y + touch.height + measures::TITLE_GAP,
        width: bounds.width,
        height: measures::TITLE_HEIGHT,
    }
}

/// Fila compacta de la salida con este índice visible.
pub(crate) fn output_row(bounds: Rect, index: usize) -> Rect {
    let title = output_title_row(bounds);
    Rect {
        x: bounds.x + glass::ROW_MARGIN,
        y: title.y + title.height + index as f32 * measures::ROW_HEIGHT,
        width: bounds.width - glass::ROW_MARGIN * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Alto que ocupa la sección de salidas (cero sin salidas).
fn outputs_block(outputs: &[AudioOutput]) -> f32 {
    let visible = visible_count(outputs) as f32;
    if visible <= 0.0 {
        0.0
    } else {
        measures::TITLE_GAP + measures::TITLE_HEIGHT + visible * measures::ROW_HEIGHT
    }
}

/// Fila compacta que silencia, bajo la primera línea.
pub(crate) fn mute_row(bounds: Rect, outputs: &[AudioOutput]) -> Rect {
    let touch = slider_touch(bounds);
    let top = touch.y + touch.height
        + outputs_block(outputs)
        + measures::SECTION_GAP
        + dokka_theme::tray_glass::SEPARATOR_HEIGHT;
    Rect {
        x: bounds.x + glass::ROW_MARGIN,
        y: top,
        width: bounds.width - glass::ROW_MARGIN * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Fila compacta de ajustes, bajo la segunda línea.
pub(crate) fn settings_row(bounds: Rect, outputs: &[AudioOutput]) -> Rect {
    let mute = mute_row(bounds, outputs);
    Rect {
        x: bounds.x + glass::ROW_MARGIN,
        y: mute.y + mute.height + dokka_theme::tray_glass::SEPARATOR_HEIGHT,
        width: bounds.width - glass::ROW_MARGIN * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Línea que separa secciones: la primera sobre silenciar, la segunda
/// bajo él. Solo geometría; el color lo pone el dibujo.
pub(crate) fn separator_rect(bounds: Rect, outputs: &[AudioOutput], index: usize) -> Rect {
    let mute = mute_row(bounds, outputs);
    let y = if index == 0 {
        mute.y - measures::SECTION_GAP
    } else {
        mute.y + mute.height
    };
    Rect {
        x: bounds.x + glass::ROW_MARGIN + glass::ROW_INSET,
        y,
        width: bounds.width - (glass::ROW_MARGIN + glass::ROW_INSET) * 2.0,
        height: dokka_theme::tray_glass::SEPARATOR_HEIGHT,
    }
}
