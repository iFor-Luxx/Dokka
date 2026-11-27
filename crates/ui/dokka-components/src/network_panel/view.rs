use std::sync::Arc;

use dokka_scene::{Dot, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, network_language as measures};

use super::{NetworkHit, NetworkIcons, NetworkType};

/// Alto fijo del panel de red.
pub fn network_size() -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::network_height(),
    }
}

/// Menú Tahoe: título, cápsula con círculo e internet, y ajustes aparte.
/// El `panel` es invisible; cada pieza va en `tiles` con cristal ahumado.
pub fn network_panel(
    theme: &Theme,
    icons: &NetworkIcons,
    bounds: Rect,
    name: &str,
    kind: NetworkType,
    has_internet: bool,
    highlighted: Option<NetworkHit>,
) -> Overlay {
    let title = title_tile(bounds);
    let status = status_tile(bounds);
    let settings = settings_tile(bounds);
    let is_active = kind != NetworkType::Offline;
    let status_circle = circle_in(status);
    let settings_circle = circle_in(settings);
    let text_x = status_circle.x + status_circle.width + measures::ICON_GAP;
    let chevron_side = measures::CHEVRON_SIDE;
    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::TILE_RADIUS,
            border_width: 0.0,
            tint: theme.tray.clear,
            border: theme.tray.clear,
        },
        tiles: vec![
            glass(theme, title, measures::TILE_RADIUS),
            glass(theme, status, status.height / 2.0),
            glass(theme, settings, measures::TILE_RADIUS),
        ],
        pill: None,
        highlight: Some(settings_highlight(theme, settings, highlighted)),
        flat_panels: Vec::new(),
        dividers: Vec::new(),
        dots: vec![
            Dot {
                x: status_circle.x,
                y: status_circle.y,
                diameter: status_circle.width,
                color: if is_active {
                    theme.tray.circle_active
                } else {
                    theme.tray.circle
                },
            },
            Dot {
                x: settings_circle.x,
                y: settings_circle.y,
                diameter: settings_circle.width,
                color: theme.tray.circle,
            },
        ],
        pointer: None,
        texts: vec![
            Text {
                x: title.x + measures::PADDING,
                center_y: title.y + title.height / 2.0,
                content: measures::NETWORK_TITLE.to_owned(),
                size: measures::TITLE_SIZE,
                is_bold: true,
                color: theme.tray.text,
                align: TextAlign::Left,
            },
            Text {
                x: text_x,
                center_y: status.y + status.height / 2.0 - 11.0,
                content: display_name(name, kind),
                size: measures::TITLE_SIZE,
                is_bold: true,
                color: theme.tray.text,
                align: TextAlign::Left,
            },
            Text {
                x: text_x,
                center_y: status.y + status.height / 2.0 + 11.0,
                content: detail(kind, has_internet),
                size: measures::SUBTITLE_SIZE,
                is_bold: false,
                color: theme.tray.secondary,
                align: TextAlign::Left,
            },
            Text {
                x: settings_circle.x + settings_circle.width + measures::ICON_GAP,
                center_y: settings.y + settings.height / 2.0,
                content: measures::NETWORK_SETTINGS_LABEL.to_owned(),
                size: measures::TITLE_SIZE,
                is_bold: false,
                color: theme.tray.text,
                align: TextAlign::Left,
            },
        ],
        icons: vec![
            Icon {
                x: status_circle.x + (status_circle.width - measures::CIRCLE_ICON) / 2.0,
                y: status_circle.y + (status_circle.height - measures::CIRCLE_ICON) / 2.0,
                side: measures::CIRCLE_ICON,
                image: Some(IconImage::Drawing(icons.for_circle(kind, is_active))),
                placeholder: theme.tray.tile,
                placeholder_radius: 0.0,
            },
            Icon {
                x: settings_circle.x + (settings_circle.width - measures::CIRCLE_ICON) / 2.0,
                y: settings_circle.y + (settings_circle.height - measures::CIRCLE_ICON) / 2.0,
                side: measures::CIRCLE_ICON,
                image: Some(IconImage::Drawing(Arc::clone(icons.settings()))),
                placeholder: theme.tray.tile,
                placeholder_radius: 0.0,
            },
            Icon {
                x: settings.x + settings.width - measures::PADDING - chevron_side,
                y: settings.y + (settings.height - chevron_side) / 2.0,
                side: chevron_side,
                image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
                placeholder: theme.tray.tile,
                placeholder_radius: 0.0,
            },
        ],
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Fila de ajustes que hay en el punto, si cae en ella.
pub fn network_hit(bounds: Rect, x: f32, y: f32) -> Option<NetworkHit> {
    let row = settings_tile(bounds);
    if x >= row.x && x < row.x + row.width && y >= row.y && y < row.y + row.height {
        Some(NetworkHit::Settings)
    } else {
        None
    }
}

fn tile_row(bounds: Rect, top: f32, height: f32) -> Rect {
    Rect {
        x: bounds.x + measures::GAP,
        y: top,
        width: bounds.width - measures::GAP * 2.0,
        height,
    }
}

fn title_tile(bounds: Rect) -> Rect {
    tile_row(bounds, bounds.y + measures::GAP, measures::TITLE_HEIGHT)
}

fn status_tile(bounds: Rect) -> Rect {
    tile_row(
        bounds,
        bounds.y + measures::GAP * 2.0 + measures::TITLE_HEIGHT,
        measures::ROW_HEIGHT,
    )
}

fn settings_tile(bounds: Rect) -> Rect {
    tile_row(
        bounds,
        bounds.y + measures::GAP * 3.0 + measures::TITLE_HEIGHT + measures::ROW_HEIGHT,
        measures::SETTINGS_HEIGHT,
    )
}

fn glass(theme: &Theme, bounds: Rect, radius: f32) -> Panel {
    Panel {
        bounds,
        corner_radius: radius.min(bounds.height / 2.0),
        border_width: measures::TILE_BORDER_WIDTH,
        tint: theme.tray.tile,
        border: theme.tray.tile_border,
    }
}

fn circle_in(tile: Rect) -> Rect {
    Rect {
        x: tile.x + measures::PADDING,
        y: tile.y + (tile.height - measures::CIRCLE) / 2.0,
        width: measures::CIRCLE,
        height: measures::CIRCLE,
    }
}

fn settings_highlight(theme: &Theme, row: Rect, highlighted: Option<NetworkHit>) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    };
    if highlighted != Some(NetworkHit::Settings) {
        return hidden;
    }
    Panel {
        bounds: row,
        corner_radius: measures::TILE_RADIUS.min(row.height / 2.0),
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    }
}

fn display_name(name: &str, kind: NetworkType) -> String {
    if kind == NetworkType::Offline || name.trim().is_empty() {
        return measures::OFFLINE_NAME.to_owned();
    }
    short(name)
}

fn detail(kind: NetworkType, has_internet: bool) -> String {
    let kind_label = match kind {
        NetworkType::Offline => measures::TYPE_OFFLINE,
        NetworkType::Ethernet => measures::TYPE_ETHERNET,
        NetworkType::Wifi => measures::TYPE_WIFI,
        NetworkType::Other => measures::TYPE_OTHER,
    };
    if kind == NetworkType::Offline {
        return kind_label.to_owned();
    }
    let internet = if has_internet {
        measures::WITH_INTERNET
    } else {
        measures::WITHOUT_INTERNET
    };
    format!("{kind_label} · {internet}")
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
