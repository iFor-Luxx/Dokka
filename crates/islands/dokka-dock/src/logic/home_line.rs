//! Geometría de la línea que queda al borde con el dock oculto.
//! Cápsula centrada donde estaba el dock, a unos 6 del borde; en
//! laterales es vertical. La zona de toque es más generosa y pegada
//! al borde para acercar el cursor sin puntería fina.

use dokka_model::DockPosition;
use dokka_scene::Rect;
use dokka_theme::dock as measures;

use crate::Dock;
use crate::logic::position_or_bottom;

/// Línea visible cuando el dock está escondido, en coordenadas de ventana.
pub(crate) fn home_line(dock: &Dock, window: dokka_scene::Size) -> Rect {
    let length = measures::HOME_LINE_LENGTH;
    let thickness = measures::HOME_LINE_THICKNESS;
    let gap = measures::HOME_LINE_EDGE_GAP;
    let touch = measures::HOME_LINE_TOUCH_THICKNESS;
    match position_or_bottom(dock) {
        DockPosition::Left => Rect {
            x: window.width - touch + gap,
            y: (window.height - length) / 2.0,
            width: thickness,
            height: length,
        },
        DockPosition::Right => Rect {
            x: touch - gap - thickness,
            y: (window.height - length) / 2.0,
            width: thickness,
            height: length,
        },
        _ => Rect {
            x: (window.width - length) / 2.0,
            y: touch - gap - thickness,
            width: length,
            height: thickness,
        },
    }
}

/// Zona de toque de la línea: más ancha que ella y pegada al borde.
pub(crate) fn home_touch(window: dokka_scene::Size, position: DockPosition) -> Rect {
    let along = measures::HOME_LINE_TOUCH_LENGTH;
    let across = measures::HOME_LINE_TOUCH_THICKNESS;
    match position {
        DockPosition::Left => Rect {
            x: window.width - across,
            y: (window.height - along) / 2.0,
            width: across,
            height: along,
        },
        DockPosition::Right => Rect {
            x: 0.0,
            y: (window.height - along) / 2.0,
            width: across,
            height: along,
        },
        _ => Rect {
            x: (window.width - along) / 2.0,
            y: 0.0,
            width: along,
            height: across,
        },
    }
}
