use dokka_scene::TextAlign;
use dokka_scene::{Divider, Overlay, Panel, Pointer, Rect, Size, Text};
use dokka_theme::{Theme, menu as measures, typography};

use super::MenuRow;
use super::check::{MenuChecks, check_icons};
use super::chevron::{MenuChevrons, chevron_icons};

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

/// Panel con resaltado, líneas, punta, textos e iconos.
pub fn menu_overlay(
    theme: &Theme,
    chevrons: &MenuChevrons,
    checks: &MenuChecks,
    bounds: Rect,
    tip_x: f32,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Overlay {
    let pointer = Some(menu_pointer(bounds, tip_x));
    overlay_inner(theme, chevrons, checks, bounds, pointer, rows, highlighted)
}

/// Mismo cristal y resaltado que el menú, sin punta.
pub fn submenu_overlay(
    theme: &Theme,
    chevrons: &MenuChevrons,
    checks: &MenuChecks,
    bounds: Rect,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Overlay {
    overlay_inner(theme, chevrons, checks, bounds, None, rows, highlighted)
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

/// Borde superior de la fila, para alinear el submenú con su madre.
pub fn menu_row_top(rows: &[MenuRow], bounds: Rect, index: usize) -> f32 {
    let mut top = bounds.y + measures::PADDING;
    for row in rows.iter().take(index) {
        top += match row {
            MenuRow::Item(_) => measures::ROW_HEIGHT,
            MenuRow::Separator => measures::SEPARATOR_HEIGHT,
        };
    }
    top
}

fn overlay_inner(
    theme: &Theme,
    chevrons: &MenuChevrons,
    checks: &MenuChecks,
    bounds: Rect,
    pointer: Option<Pointer>,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Overlay {
    let with_checks = rows
        .iter()
        .any(|row| matches!(row, MenuRow::Item(item) if item.checked));
    let label_x = bounds.x
        + measures::PADDING
        + measures::TEXT_INSET
        + if with_checks {
            measures::CHECK_WIDTH
        } else {
            0.0
        };
    let mut top = bounds.y + measures::PADDING;
    let mut texts = Vec::new();
    let mut dividers = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        match row {
            MenuRow::Item(item) => {
                let color = if highlighted == Some(index) {
                    theme.menu.highlighted_text
                } else {
                    theme.menu.text
                };
                texts.push(mark_text(label_x, top, &item.label, color));
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
        pointer,
        texts,
        icons: {
            let mut icons = chevron_icons(theme, chevrons, bounds, rows, highlighted);
            icons.extend(check_icons(theme, checks, bounds, rows, highlighted));
            icons
        },
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

fn mark_text(x: f32, top: f32, content: &str, color: dokka_scene::Color) -> Text {
    Text {
        x,
        center_y: top + measures::ROW_HEIGHT / 2.0,
        content: content.to_owned(),
        size: typography::MENU_SIZE,
        is_bold: false,
        color,
        align: TextAlign::Left,
    }
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
            y: menu_row_top(rows, bounds, index),
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

fn menu_pointer(bounds: Rect, tip_x: f32) -> Pointer {
    // La base (2 × altura) debe quedar sobre el tramo recto del borde inferior.
    let half = measures::POINTER_HEIGHT;
    let min = bounds.x + measures::CORNER_RADIUS + half;
    let max = bounds.x + bounds.width - measures::CORNER_RADIUS - half;
    Pointer {
        center_x: tip_x.clamp(min, max),
        height: measures::POINTER_HEIGHT,
    }
}
