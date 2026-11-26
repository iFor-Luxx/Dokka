use std::sync::Arc;

use dokka_scene::TextAlign;
use dokka_scene::{Divider, Drawing, Icon, IconImage, Overlay, Panel, Rect, Shape, Size, Text};
use dokka_theme::{Theme, menu as measures, typography};

use super::MenuRow;

/// Alto del panel para estas filas.
pub fn menu_size(rows: &[MenuRow]) -> Size {
    let mut height = measures::PADDING * 2.0;
    for row in rows {
        height += match row {
            MenuRow::Item(_) => measures::ROW_HEIGHT,
            MenuRow::Separator => measures::SEPARATOR_HEIGHT,
        };
    }
    Size {
        width: measures::WIDTH,
        height,
    }
}

/// Panel con resaltado, líneas, punta y textos.
pub fn menu_overlay(
    theme: &Theme,
    bounds: Rect,
    tip_x: f32,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Overlay {
    let mut top = bounds.y + measures::PADDING;
    let mut texts = Vec::new();
    let mut dividers = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        match row {
            MenuRow::Item(label) => {
                let color = if highlighted == Some(index) {
                    theme.menu.highlighted_text
                } else {
                    theme.menu.text
                };
                texts.push(Text {
                    x: bounds.x + measures::PADDING + measures::TEXT_INSET,
                    center_y: top + measures::ROW_HEIGHT / 2.0,
                    content: label.clone(),
                    size: typography::MENU_SIZE,
                    is_bold: false,
                    color,
                    align: TextAlign::Left,
                });
                top += measures::ROW_HEIGHT;
            }
            MenuRow::Separator => {
                let middle = top + measures::SEPARATOR_HEIGHT / 2.0;
                dividers.push(Divider {
                    bounds: Rect {
                        x: bounds.x + measures::PADDING + measures::TEXT_INSET,
                        y: middle - measures::DIVIDER_HEIGHT / 2.0,
                        width: bounds.width - (measures::PADDING + measures::TEXT_INSET) * 2.0,
                        height: measures::DIVIDER_HEIGHT,
                    },
                    color: theme.menu.separator,
                });
                top += measures::SEPARATOR_HEIGHT;
            }
        }
    }
    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::CORNER_RADIUS,
            border_width: measures::BORDER_WIDTH,
            tint: theme.menu.background,
            border: theme.menu.border,
        },
        highlight: Some(highlight_panel(theme, bounds, rows, highlighted)),
        dividers,
        dots: Vec::new(),
        icons: vec![tip_icon(theme, bounds, tip_x)],
        texts,
        thumbnails: Vec::new(),
        controls: None,
    }
}

/// Fila de texto que hay en el punto, si es elegible.
pub fn menu_row_at(rows: &[MenuRow], bounds: Rect, x: f32, y: f32) -> Option<usize> {
    if x < bounds.x || x > bounds.x + bounds.width {
        return None;
    }
    let mut top = bounds.y + measures::PADDING;
    for (index, row) in rows.iter().enumerate() {
        let height = match row {
            MenuRow::Item(_) => measures::ROW_HEIGHT,
            MenuRow::Separator => measures::SEPARATOR_HEIGHT,
        };
        if y >= top && y < top + height {
            return match row {
                MenuRow::Item(_) => Some(index),
                MenuRow::Separator => None,
            };
        }
        top += height;
    }
    None
}

fn highlight_panel(
    theme: &Theme,
    bounds: Rect,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Panel {
    let visible =
        highlighted.is_some_and(|index| matches!(rows.get(index), Some(MenuRow::Item(_))));
    let bounds = if visible {
        let index = highlighted.unwrap_or(0);
        Rect {
            x: bounds.x + measures::PADDING,
            y: row_top(rows, bounds, index),
            width: bounds.width - measures::PADDING * 2.0,
            height: measures::ROW_HEIGHT,
        }
    } else {
        Rect {
            x: bounds.x + measures::PADDING,
            y: bounds.y + measures::PADDING,
            width: 0.0,
            height: 0.0,
        }
    };
    Panel {
        bounds,
        corner_radius: measures::HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.menu.highlight,
        border: theme.menu.highlight,
    }
}

fn row_top(rows: &[MenuRow], bounds: Rect, index: usize) -> f32 {
    let mut top = bounds.y + measures::PADDING;
    for row in rows.iter().take(index) {
        top += match row {
            MenuRow::Item(_) => measures::ROW_HEIGHT,
            MenuRow::Separator => measures::SEPARATOR_HEIGHT,
        };
    }
    top
}

fn tip_icon(theme: &Theme, bounds: Rect, tip_x: f32) -> Icon {
    let side = measures::POINTER_WIDTH;
    // El lienzo del dibujo mide 100 unidades: la altura de la punta sale de las medidas.
    let apex = measures::POINTER_HEIGHT * 100.0 / measures::POINTER_WIDTH;
    let drawing = Drawing {
        shapes: vec![Shape::Triangle {
            points: [(0.0, 0.0), (100.0, 0.0), (50.0, apex)],
            color: theme.menu.background,
        }],
    };
    Icon {
        x: (tip_x - side / 2.0).clamp(bounds.x, bounds.x + bounds.width - side),
        y: bounds.y + bounds.height,
        side,
        image: Some(IconImage::Drawing(Arc::new(drawing))),
        placeholder: theme.menu.background,
        placeholder_radius: 0.0,
    }
}
