use dokka_scene::{Divider, Panel, Rect};
use dokka_theme::{Theme, search as measures};

use super::SearchPanelProps;

/// Fila visible que hay en el punto, siguiendo lo mostrado.
pub(super) fn row_at(bounds: Rect, x: f32, y: f32, visible: usize) -> Option<usize> {
    (0..visible).find_map(|index| {
        let top = measures::row_top(bounds, index);
        let inside_x = (bounds.x + measures::ROW_MARGIN
            ..bounds.x + bounds.width - measures::ROW_MARGIN)
            .contains(&x);
        let inside_y = (top..top + measures::ROW_HEIGHT).contains(&y);
        (inside_x && inside_y).then_some(index)
    })
}

/// Fondo azul de la fila resaltada; de tamaño cero sin fila válida.
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
        tint: theme.menu.highlight,
        border: theme.menu.highlight,
    };
    let Some(index) = props.highlighted else {
        return hidden;
    };
    if index >= visible {
        return hidden;
    }
    Panel {
        bounds: Rect {
            x: bounds.x + measures::ROW_MARGIN,
            y: measures::row_top(bounds, index),
            width: bounds.width - measures::ROW_MARGIN * 2.0,
            height: measures::ROW_HEIGHT,
        },
        corner_radius: measures::ROW_RADIUS,
        border_width: 0.0,
        tint: theme.menu.highlight,
        border: theme.menu.highlight,
    }
}

/// Cursor tras lo escrito; de ancho cero cuando no parpadea.
pub(super) fn field_cursor(theme: &Theme, search: &Rect, props: &SearchPanelProps) -> Divider {
    if !props.cursor_visible {
        return Divider {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            color: theme.panel.text,
        };
    }
    Divider {
        bounds: Rect {
            x: text_end(search, &props.query),
            y: search.y + (search.height - measures::SEARCH_CURSOR_HEIGHT) / 2.0,
            width: measures::SEARCH_CURSOR_WIDTH,
            height: measures::SEARCH_CURSOR_HEIGHT,
        },
        color: theme.panel.text,
    }
}

/// Línea bajo la casilla, siempre visible para no reconstruir capas.
pub(super) fn row_separator(theme: &Theme, bounds: Rect) -> Divider {
    let search = measures::search_box(bounds);
    Divider {
        bounds: Rect {
            x: bounds.x + measures::SEARCH_MARGIN,
            y: search.y + search.height + measures::GAP / 2.0 - measures::SEPARATOR_HEIGHT / 2.0,
            width: bounds.width - measures::SEARCH_MARGIN * 2.0,
            height: measures::SEPARATOR_HEIGHT,
        },
        color: theme.menu.separator,
    }
}

/// Borde derecho de lo escrito, con aire antes del cursor.
fn text_end(search: &Rect, query: &str) -> f32 {
    let advance = query.chars().count() as f32 * measures::SEARCH_CHAR_ADVANCE;
    let gap = if query.is_empty() {
        0.0
    } else {
        measures::SEARCH_CURSOR_GAP
    };
    search.x + measures::SEARCH_TEXT_INSET + advance + gap
}
