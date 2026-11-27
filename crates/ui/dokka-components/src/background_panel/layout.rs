//! Posiciones del panel de segundo plano, compartidas por dibujo y golpe.
//!
//! Una columna: título y una fila compacta por app con su círculo.

use dokka_scene::Rect;
use dokka_theme::background as measures;

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
        x: bounds.x + measures::PADDING,
        y: title.y
            + title.height
            + measures::TITLE_GAP
            + index as f32 * (measures::ROW_HEIGHT + measures::ROW_GAP),
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

/// Círculo suave dentro de la fila, a la izquierda.
pub(crate) fn circle_in(row: Rect) -> Rect {
    Rect {
        x: row.x,
        y: row.y + (row.height - measures::CIRCLE) / 2.0,
        width: measures::CIRCLE,
        height: measures::CIRCLE,
    }
}
