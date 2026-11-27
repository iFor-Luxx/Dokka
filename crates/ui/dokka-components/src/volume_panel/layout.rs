//! Posiciones del panel de volumen, compartidas por el dibujo y el hit.
//!
//! Una columna: título, zona de toque del deslizador, dos líneas finas y
//! dos filas compactas (silenciar y ajustes).

use dokka_scene::Rect;
use dokka_theme::volume as measures;

/// Fila del título en negrita arriba a la izquierda.
pub(crate) fn title_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y + measures::PADDING,
        width: bounds.width,
        height: measures::TITLE_HEIGHT,
    }
}

/// Zona de toque del deslizador, más alta que la cápsula.
pub(crate) fn slider_touch(bounds: Rect) -> Rect {
    let title = title_row(bounds);
    Rect {
        x: bounds.x + measures::PADDING,
        y: title.y + title.height + measures::TITLE_GAP,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::SLIDER_ROW_HEIGHT,
    }
}

/// Fila compacta que silencia, bajo la primera línea.
pub(crate) fn mute_row(bounds: Rect) -> Rect {
    let touch = slider_touch(bounds);
    let top =
        touch.y + touch.height + measures::SECTION_GAP + dokka_theme::tray_glass::SEPARATOR_HEIGHT;
    Rect {
        x: bounds.x + measures::PADDING,
        y: top,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Fila compacta de ajustes, bajo la segunda línea.
pub(crate) fn settings_row(bounds: Rect) -> Rect {
    let mute = mute_row(bounds);
    Rect {
        x: bounds.x + measures::PADDING,
        y: mute.y + mute.height + dokka_theme::tray_glass::SEPARATOR_HEIGHT,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Línea que separa secciones: la primera sobre silenciar, la segunda
/// bajo él. Solo geometría; el color lo pone el dibujo.
pub(crate) fn separator_rect(bounds: Rect, index: usize) -> Rect {
    let mute = mute_row(bounds);
    let y = if index == 0 {
        mute.y - measures::SECTION_GAP
    } else {
        mute.y + mute.height
    };
    Rect {
        x: bounds.x + measures::PADDING,
        y,
        width: bounds.width - measures::PADDING * 2.0,
        height: dokka_theme::tray_glass::SEPARATOR_HEIGHT,
    }
}
