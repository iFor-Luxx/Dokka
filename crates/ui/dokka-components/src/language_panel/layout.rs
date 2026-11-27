//! Posiciones del panel de idioma, compartidas por el dibujo y el hit.
//!
//! Una columna: título, una fila compacta por idioma, una línea fina y la
//! fila de ajustes.

use dokka_scene::Rect;
use dokka_theme::network_language as measures;

use super::LanguageEntry;

/// Fila del título en negrita arriba a la izquierda.
pub(crate) fn title_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y + measures::PADDING,
        width: bounds.width,
        height: measures::TITLE_HEIGHT,
    }
}

/// Fila compacta de un idioma, con su hueco respecto a la anterior.
pub(crate) fn row_rect(bounds: Rect, index: usize) -> Rect {
    let title = title_row(bounds);
    Rect {
        x: bounds.x + measures::PADDING,
        y: title.y
            + title.height
            + measures::TITLE_GAP
            + index as f32 * (measures::ROW_HEIGHT + measures::ROW_GAP),
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Fila compacta de ajustes, bajo la línea y el último idioma.
pub(crate) fn settings_row(bounds: Rect, count: usize) -> Rect {
    let top = if count == 0 {
        let title = title_row(bounds);
        title.y + title.height + measures::TITLE_GAP
    } else {
        let last = row_rect(bounds, count - 1);
        last.y + last.height
    };
    Rect {
        x: bounds.x + measures::PADDING,
        y: top + measures::SECTION_GAP + dokka_theme::tray_glass::SEPARATOR_HEIGHT,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Línea que separa las filas de la de ajustes. Solo geometría; el color
/// lo pone el dibujo.
pub(crate) fn separator_rect(bounds: Rect, count: usize) -> Rect {
    let settings = settings_row(bounds, count);
    Rect {
        x: bounds.x + measures::PADDING,
        y: settings.y - measures::SECTION_GAP,
        width: bounds.width - measures::PADDING * 2.0,
        height: dokka_theme::tray_glass::SEPARATOR_HEIGHT,
    }
}

/// Recuadro pequeño del código a la derecha de la fila.
pub(crate) fn code_rect(bounds: Rect, index: usize, _language: &LanguageEntry) -> Rect {
    let row = row_rect(bounds, index);
    Rect {
        x: row.x + row.width - measures::CODE_WIDTH,
        y: row.y + (row.height - measures::CODE_HEIGHT) / 2.0,
        width: measures::CODE_WIDTH,
        height: measures::CODE_HEIGHT,
    }
}
