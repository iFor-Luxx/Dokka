//! Posiciones del panel de segundo plano, compartidas por dibujo y golpe.
//!
//! Una columna: título y una fila compacta por app con su círculo.

use dokka_scene::Rect;
use dokka_theme::{background as measures, tray_glass as glass};

/// Fila del título en negrita arriba a la izquierda.
pub(crate) fn title_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y + measures::PADDING,
        width: bounds.width,
        height: measures::TITLE_HEIGHT,
    }
}

/// Fila compacta de la app con este índice.
pub(crate) fn row_rect(bounds: Rect, index: usize) -> Rect {
    let title = title_row(bounds);
    Rect {
        x: bounds.x + glass::ROW_MARGIN,
        y: title.y
            + title.height
            + measures::TITLE_GAP
            + index as f32 * (measures::ROW_HEIGHT + measures::ROW_GAP),
        width: bounds.width - glass::ROW_MARGIN * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Círculo suave dentro de la fila, a la izquierda.
pub(crate) fn circle_in(row: Rect) -> Rect {
    Rect {
        x: row.x + glass::ROW_INSET,
        y: row.y + (row.height - measures::CIRCLE) / 2.0,
        width: measures::CIRCLE,
        height: measures::CIRCLE,
    }
}
