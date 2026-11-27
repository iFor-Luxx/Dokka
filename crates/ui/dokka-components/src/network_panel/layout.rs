//! Posiciones del panel de red, compartidas por el dibujo y el hit.
//!
//! Una columna: título, fila de estado con círculo, una línea fina y la
//! fila de ajustes.

use dokka_scene::Rect;
use dokka_theme::network_language as measures;

/// Fila del título en negrita arriba a la izquierda.
pub(crate) fn title_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y + measures::PADDING,
        width: bounds.width,
        height: measures::TITLE_HEIGHT,
    }
}

/// Fila de estado (tipo y nombre, dos líneas) bajo el título.
pub(crate) fn status_row(bounds: Rect) -> Rect {
    let title = title_row(bounds);
    Rect {
        x: bounds.x + measures::PADDING,
        y: title.y + title.height + measures::TITLE_GAP,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::STATUS_HEIGHT,
    }
}

/// Fila compacta de ajustes, bajo la línea.
pub(crate) fn settings_row(bounds: Rect) -> Rect {
    let status = status_row(bounds);
    let top = status.y
        + status.height
        + measures::SECTION_GAP
        + dokka_theme::tray_glass::SEPARATOR_HEIGHT;
    Rect {
        x: bounds.x + measures::PADDING,
        y: top,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Línea que separa el estado de los ajustes. Solo geometría; el color
/// lo pone el dibujo.
pub(crate) fn separator_rect(bounds: Rect) -> Rect {
    let settings = settings_row(bounds);
    Rect {
        x: bounds.x + measures::PADDING,
        y: settings.y - measures::SECTION_GAP,
        width: bounds.width - measures::PADDING * 2.0,
        height: dokka_theme::tray_glass::SEPARATOR_HEIGHT,
    }
}

/// Círculo de icono dentro de la fila de estado, a la izquierda.
pub(crate) fn circle_in(status: Rect) -> Rect {
    Rect {
        x: status.x,
        y: status.y + (status.height - measures::CIRCLE) / 2.0,
        width: measures::CIRCLE,
        height: measures::CIRCLE,
    }
}
