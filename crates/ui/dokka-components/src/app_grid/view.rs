use dokka_scene::{Icon, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, launchpad as measures};

use super::AppGridProps;

/// Celda visible: icono, centro del texto y zona de resaltado.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCell {
    pub icon: Rect,
    pub text_x: f32,
    pub text_y: f32,
    pub highlight: Rect,
}

/// Celdas que caben en la pantalla, centradas y sin pasar bajo el dock.
pub fn grid_cells(props: &AppGridProps) -> Vec<GridCell> {
    let total = props.apps.len();
    if total == 0 {
        return Vec::new();
    }
    let columns = grid_columns(total, props.screen.width);
    if columns == 0 {
        return Vec::new();
    }
    let usable_height = props.screen.height - measures::TOP_PADDING - measures::BOTTOM_RESERVE;
    if usable_height < measures::ICON_SIDE {
        return Vec::new();
    }
    let row_height = measures::ICON_SIDE + measures::TEXT_GAP + measures::TEXT_HEIGHT;
    let rows_fit =
        ((usable_height + measures::ROW_GAP) / (row_height + measures::ROW_GAP)) as usize;
    if rows_fit == 0 {
        return Vec::new();
    }
    let visible = total.min(columns * rows_fit);
    let rows_used = visible.div_ceil(columns);
    let total_width =
        columns as f32 * measures::ICON_SIDE + (columns as f32 - 1.0) * measures::COLUMN_GAP;
    let grid_height = rows_used as f32 * row_height + (rows_used as f32 - 1.0) * measures::ROW_GAP;
    let start_x = (props.screen.width - total_width) / 2.0;
    let start_y = measures::TOP_PADDING + (usable_height - grid_height) / 2.0;

    let mut cells = Vec::with_capacity(visible);
    for index in 0..visible {
        let column = index % columns;
        let row = index / columns;
        let x = start_x + column as f32 * (measures::ICON_SIDE + measures::COLUMN_GAP);
        let y = start_y + row as f32 * (row_height + measures::ROW_GAP);
        let highlight = Rect {
            x: x - measures::HIGHLIGHT_PADDING,
            y: y - measures::HIGHLIGHT_PADDING,
            width: measures::ICON_SIDE + measures::HIGHLIGHT_PADDING * 2.0,
            height: row_height + measures::HIGHLIGHT_PADDING * 2.0,
        };
        cells.push(GridCell {
            icon: Rect {
                x,
                y,
                width: measures::ICON_SIDE,
                height: measures::ICON_SIDE,
            },
            text_x: x + measures::ICON_SIDE / 2.0,
            text_y: y + measures::ICON_SIDE + measures::TEXT_GAP + measures::TEXT_HEIGHT / 2.0,
            highlight,
        });
    }
    cells
}

/// Iconos grandes de las celdas visibles.
pub fn app_icons(theme: &Theme, props: &AppGridProps) -> Vec<Icon> {
    let cells = grid_cells(props);
    props
        .apps
        .iter()
        .zip(cells.iter())
        .map(|(app, cell)| Icon {
            x: cell.icon.x,
            y: cell.icon.y,
            side: measures::ICON_SIDE,
            image: app.image.clone(),
            placeholder: theme.launchpad.highlight,
            placeholder_radius: measures::HIGHLIGHT_RADIUS,
        })
        .collect()
}

/// Nombres bajo los iconos, recortados con "…".
pub fn app_texts(theme: &Theme, props: &AppGridProps) -> Vec<Text> {
    let cells = grid_cells(props);
    props
        .apps
        .iter()
        .zip(cells.iter())
        .map(|(app, cell)| Text {
            x: cell.text_x,
            center_y: cell.text_y,
            content: short_name(&app.name),
            size: measures::TEXT_SIZE,
            is_bold: false,
            color: theme.launchpad.text,
            align: TextAlign::Center,
        })
        .collect()
}

/// Resaltado suave detrás de la app bajo el cursor.
pub fn app_highlight(theme: &Theme, props: &AppGridProps) -> Option<Panel> {
    let cells = grid_cells(props);
    let index = props.highlighted?;
    let cell = cells.get(index)?;
    Some(Panel {
        bounds: cell.highlight,
        corner_radius: measures::HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.launchpad.highlight,
        border: theme.launchpad.highlight,
    })
}

/// Qué app hay en el punto, si es visible.
pub fn app_at(props: &AppGridProps, x: f32, y: f32) -> Option<usize> {
    for (index, cell) in grid_cells(props).iter().enumerate() {
        let bounds = cell.highlight;
        if x >= bounds.x
            && x < bounds.x + bounds.width
            && y >= bounds.y
            && y < bounds.y + bounds.height
        {
            return Some(index);
        }
    }
    None
}

fn grid_columns(total: usize, screen_width: f32) -> usize {
    let usable = screen_width - measures::SIDE_PADDING * 2.0;
    if usable < measures::ICON_SIDE {
        return 0;
    }
    let by_width =
        ((usable + measures::COLUMN_GAP) / (measures::ICON_SIDE + measures::COLUMN_GAP)) as usize;
    total.min(measures::MAX_COLUMNS).min(by_width.max(1))
}

fn short_name(name: &str) -> String {
    if name.chars().count() <= measures::MAX_NAME_CHARACTERS {
        return name.to_owned();
    }
    let kept: String = name
        .chars()
        .take(measures::MAX_NAME_CHARACTERS - 1)
        .collect();
    format!("{}…", kept.trim_end())
}
