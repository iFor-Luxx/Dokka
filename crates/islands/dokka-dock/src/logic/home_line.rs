//! Geometría de la línea que queda al borde con el dock oculto.
//! Cápsula centrada de 280 × 10 a 10 del borde; en laterales es
//! vertical. La zona de toque (380 de largo, desde el borde hasta
//! 24 hacia dentro) llega hasta el píxel 0. La banda de retorno
//! cubre todo el cruce durante la subida para no perder el cursor
//! a medio camino.

use dokka_model::DockPosition;
use dokka_scene::{Rect, Size};
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
/// Si la ventana es menor que el toque, lo recorta para no salirse.
pub(crate) fn home_touch(window: Size, position: DockPosition) -> Rect {
    let along = measures::HOME_LINE_TOUCH_LENGTH.min(along_size(window, position));
    let across = measures::HOME_LINE_TOUCH_THICKNESS;
    match position {
        DockPosition::Left => Rect {
            x: window.width - across.min(window.width),
            y: (window.height - along) / 2.0,
            width: across.min(window.width),
            height: along,
        },
        DockPosition::Right => Rect {
            x: 0.0,
            y: (window.height - along) / 2.0,
            width: across.min(window.width),
            height: along,
        },
        _ => Rect {
            x: (window.width - along) / 2.0,
            y: 0.0,
            width: along,
            height: across.min(window.height),
        },
    }
}

/// Banda que acompaña al cursor mientras el dock vuelve con una app
/// ocupando: todo el cruce (del borde a la barra) con el largo del
/// toque, centrada. Sin ella la zona activa saltaría de la tira de
/// arriba a la barra y el cursor quedaría fuera a medio deslizamiento,
/// con lo que el dock recaería antes de llegar.
pub(crate) fn return_band(window: Size, position: DockPosition) -> Rect {
    let along = measures::HOME_LINE_TOUCH_LENGTH.min(along_size(window, position));
    match position {
        DockPosition::Left | DockPosition::Right => Rect {
            x: 0.0,
            y: (window.height - along) / 2.0,
            width: window.width,
            height: along,
        },
        _ => Rect {
            x: (window.width - along) / 2.0,
            y: 0.0,
            width: along,
            height: window.height,
        },
    }
}

/// Tamaño de la ventana a lo largo del borde donde está el dock.
fn along_size(window: Size, position: DockPosition) -> f32 {
    match position {
        DockPosition::Left | DockPosition::Right => window.height,
        _ => window.width,
    }
}
