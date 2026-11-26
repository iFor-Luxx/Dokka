use dokka_scene::{Icon, Panel, Rect, Size, Text, TextAlign};
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

/// Celdas de la página: 7 columnas fijas que empiezan arriba y se llenan de
/// izquierda a derecha. La cuadrícula entera queda centrada, pero las apps que
/// no llenan la fila ocupan las primeras columnas sin centrarse.
pub fn grid_cells(props: &AppGridProps) -> Vec<GridCell> {
    let total = props.apps.len();
    if total == 0 {
        return Vec::new();
    }
    let icon = measures::icon_side(props.screen.width);
    if icon <= 0.0 {
        return Vec::new();
    }
    let rows_fit = rows_that_fit(props.screen.width, props.screen.height, icon);
    if rows_fit == 0 {
        return Vec::new();
    }
    let visible = total.min(measures::COLUMNS * rows_fit);
    let start_x = measures::first_center_x(props.screen.width) - icon / 2.0;
    let start_y = measures::grid_top(props.screen.height);
    let pitch = measures::column_pitch(props.screen.width);
    let row_step =
        measures::row_height(props.screen.width) + measures::row_gap(props.screen.height);

    let mut cells = Vec::with_capacity(visible);
    for index in 0..visible {
        let column = index % measures::COLUMNS;
        let row = index / measures::COLUMNS;
        let x = start_x + column as f32 * pitch;
        let y = start_y + row as f32 * row_step;
        let highlight = Rect {
            x: x - measures::HIGHLIGHT_PADDING,
            y: y - measures::HIGHLIGHT_PADDING,
            width: icon + measures::HIGHLIGHT_PADDING * 2.0,
            height: measures::row_height(props.screen.width) + measures::HIGHLIGHT_PADDING * 2.0,
        };
        cells.push(GridCell {
            icon: Rect {
                x,
                y,
                width: icon,
                height: icon,
            },
            text_x: x + icon / 2.0,
            text_y: y + icon + measures::TEXT_GAP + measures::TEXT_HEIGHT / 2.0,
            highlight,
        });
    }
    cells
}

/// Iconos grandes de las celdas visibles.
pub fn app_icons(theme: &Theme, props: &AppGridProps) -> Vec<Icon> {
    let cells = grid_cells(props);
    let side = measures::icon_side(props.screen.width);
    props
        .apps
        .iter()
        .zip(cells.iter())
        .map(|(app, cell)| Icon {
            x: cell.icon.x,
            y: cell.icon.y,
            side,
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

/// Resaltado suave detrás de la app bajo el cursor. Siempre entrega un panel con
/// el mismo estilo: sin app resaltada tiene tamaño cero y no se ve, como el menú.
pub fn app_highlight(theme: &Theme, props: &AppGridProps) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.launchpad.highlight,
        border: theme.launchpad.highlight,
    };
    let cells = grid_cells(props);
    let Some(index) = props.highlighted else {
        return hidden;
    };
    let Some(cell) = cells.get(index) else {
        return hidden;
    };
    Panel {
        bounds: cell.highlight,
        corner_radius: measures::HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.launchpad.highlight,
        border: theme.launchpad.highlight,
    }
}

/// Cuántas apps caben en una página para esta pantalla.
pub fn page_capacity(screen: Size) -> usize {
    let icon = measures::icon_side(screen.width);
    if icon <= 0.0 {
        return 0;
    }
    rows_that_fit(screen.width, screen.height, icon) * measures::COLUMNS
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

/// Filas que caben entre la primera fila y los puntos de página.
fn rows_that_fit(screen_width: f32, screen_height: f32, icon: f32) -> usize {
    let bottom =
        measures::dots_center_y(screen_height) - screen_height * measures::GRID_BOTTOM_MARGIN_RATIO;
    let usable = bottom - measures::grid_top(screen_height);
    let row = measures::row_height(screen_width) + measures::row_gap(screen_height);
    if usable < icon || row <= 0.0 {
        return 0;
    }
    (((usable + measures::row_gap(screen_height)) / row) as usize).max(1)
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
