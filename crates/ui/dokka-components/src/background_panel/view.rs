use dokka_scene::{Dot, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, background as measures, tray_glass as glass};

use super::layout::{circle_in, row_rect, title_row};
use super::{BackgroundApp, BackgroundHit};

/// Alto del panel para estas apps (como máximo diez filas).
pub fn background_size(count: usize) -> Size {
    let visible = measures::visible_rows(count);
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::panel_height(visible),
    }
}

/// Un solo cristal: título y una fila por app con círculo suave, icono
/// real y nombre; sin apps, el mensaje de vacío. Capas siempre iguales:
/// un cristal, un resaltado, diez círculos, diez iconos, un título más
/// diez nombres más el vacío (lo que no se ve va de tamaño cero o vacío).
pub fn background_panel(
    theme: &Theme,
    bounds: Rect,
    apps: &[BackgroundApp],
    highlighted: Option<BackgroundHit>,
) -> Overlay {
    let visible = measures::visible_rows(apps.len());
    let title = title_row(bounds);
    let mut dots = Vec::with_capacity(measures::MAX_ROWS);
    let mut icons = Vec::with_capacity(measures::MAX_ROWS);
    let mut texts = Vec::with_capacity(measures::MAX_ROWS + 2);
    texts.push(Text {
        x: title.x + glass::ROW_MARGIN + glass::ROW_INSET,
        center_y: title.y + title.height / 2.0,
        content: measures::TITLE.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Left,
    });
    for slot in 0..measures::MAX_ROWS {
        let Some(app) = apps.get(slot).filter(|_| slot < visible) else {
            dots.push(Dot {
                x: 0.0,
                y: 0.0,
                diameter: 0.0,
                color: theme.tray.circle,
            });
            icons.push(Icon {
                x: 0.0,
                y: 0.0,
                side: 0.0,
                image: None,
                placeholder: theme.tray.panel,
                placeholder_radius: 0.0,
            });
            texts.push(Text {
                x: 0.0,
                center_y: 0.0,
                content: String::new(),
                size: measures::TITLE_SIZE,
                is_bold: false,
                color: theme.tray.text,
                align: TextAlign::Left,
            });
            continue;
        };
        let row = row_rect(bounds, slot);
        let circle = circle_in(row);
        dots.push(Dot {
            x: circle.x,
            y: circle.y,
            diameter: circle.width,
            color: theme.tray.circle,
        });
        icons.push(Icon {
            x: circle.x + (circle.width - measures::CIRCLE_ICON) / 2.0,
            y: circle.y + (circle.height - measures::CIRCLE_ICON) / 2.0,
            side: measures::CIRCLE_ICON,
            image: app.icon.clone().map(IconImage::Pixels),
            placeholder: theme.tray.circle,
            placeholder_radius: 0.0,
        });
        texts.push(Text {
            x: circle.x + circle.width + measures::ICON_GAP,
            center_y: row.y + row.height / 2.0,
            content: short(&app.name),
            size: measures::TITLE_SIZE,
            is_bold: false,
            color: theme.tray.text,
            align: TextAlign::Left,
        });
    }
    texts.push(Text {
        x: bounds.x + bounds.width / 2.0,
        center_y: row_rect(bounds, 0).y + measures::ROW_HEIGHT / 2.0,
        content: if apps.is_empty() {
            measures::EMPTY_TEXT.to_owned()
        } else {
            String::new()
        },
        size: measures::EMPTY_TEXT_SIZE,
        is_bold: false,
        color: theme.tray.secondary,
        align: TextAlign::Center,
    });
    Overlay {
        panel: glass(theme, bounds),
        tiles: Vec::new(),
        panel_shadow: None,
        tile_shadow: None,
        pill: None,
        highlight: Some(highlight_panel(theme, bounds, apps, highlighted, visible)),
        flat_panels: Vec::new(),
        dividers: Vec::new(),
        dots,
        pointer: None,
        texts,
        icons,
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

fn glass(theme: &Theme, bounds: Rect) -> Panel {
    Panel {
        bounds,
        corner_radius: capped(bounds, measures::PANEL_RADIUS),
        border_width: measures::PANEL_BORDER_WIDTH,
        tint: theme.tray.panel,
        border: theme.tray.panel_border,
    }
}

fn capped(tile: Rect, maximum: f32) -> f32 {
    if tile.width <= 0.0 || tile.height <= 0.0 {
        return 0.0;
    }
    maximum
        .min(tile.width / 2.0)
        .min(tile.height / 2.0)
        .max(0.0)
}

fn highlight_panel(
    theme: &Theme,
    bounds: Rect,
    apps: &[BackgroundApp],
    highlighted: Option<BackgroundHit>,
    visible: usize,
) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: 0.0,
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    };
    let Some(BackgroundHit::Row(id)) = highlighted else {
        return hidden;
    };
    let Some(index) = apps.iter().take(visible).position(|app| app.id == id) else {
        return hidden;
    };
    let row = row_rect(bounds, index);
    Panel {
        bounds: row,
        corner_radius: capped(row, measures::HIGHLIGHT_RADIUS),
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    }
}

fn short(name: &str) -> String {
    if name.chars().count() <= measures::MAX_NAME_CHARACTERS {
        return name.to_owned();
    }
    let kept: String = name
        .chars()
        .take(measures::MAX_NAME_CHARACTERS - 1)
        .collect();
    format!("{}…", kept.trim_end())
}
