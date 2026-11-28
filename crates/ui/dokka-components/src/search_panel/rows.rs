use dokka_scene::{Panel, Rect, TextCursor};
use dokka_theme::{Theme, search as measures};

use super::SearchPanelProps;
use crate::search_field::text_cursor;

/// Fila visible que hay en el punto, siguiendo lo mostrado.
pub(super) fn row_at(bounds: Rect, x: f32, y: f32, visible: usize) -> Option<usize> {
    let results = measures::results_box(bounds);
    (0..visible).find(|&index| {
        let top = measures::row_top(bounds, index);
        let inside_x = (results.x + measures::ROW_MARGIN
            ..results.x + results.width - measures::ROW_MARGIN)
            .contains(&x);
        let inside_y = (top..top + measures::ROW_HEIGHT).contains(&y);
        inside_x && inside_y
    })
}

/// Cápsula translúcida de la fila resaltada; de tamaño cero sin fila válida.
pub(super) fn result_highlight(
    theme: &Theme,
    bounds: Rect,
    props: &SearchPanelProps,
    visible: usize,
) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::ROW_RADIUS,
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    };
    let Some(index) = props.highlighted else {
        return hidden;
    };
    if index >= visible {
        return hidden;
    }
    Panel {
        bounds: Rect {
            x: measures::results_box(bounds).x + measures::ROW_MARGIN,
            y: measures::row_top(bounds, index),
            width: measures::results_box(bounds).width - measures::ROW_MARGIN * 2.0,
            height: measures::ROW_HEIGHT,
        },
        corner_radius: measures::ROW_RADIUS.min(measures::ROW_HEIGHT / 2.0),
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    }
}

/// Cursor tras lo escrito medido; oculto cuando no parpadea.
pub(super) fn field_cursor(theme: &Theme, search: &Rect, props: &SearchPanelProps) -> TextCursor {
    text_cursor(TextCursor {
        text_x: search.x + measures::SEARCH_TEXT_INSET,
        center_y: search.y + search.height / 2.0,
        content: props.query.clone(),
        size: measures::SEARCH_TEXT_SIZE,
        is_bold: false,
        max_x: search.x + search.width - measures::SEARCH_TEXT_INSET,
        gap: measures::SEARCH_CURSOR_GAP,
        width: measures::SEARCH_CURSOR_WIDTH,
        height: measures::SEARCH_CURSOR_HEIGHT,
        color: theme.tray.text,
        visible: props.cursor_visible,
    })
}
