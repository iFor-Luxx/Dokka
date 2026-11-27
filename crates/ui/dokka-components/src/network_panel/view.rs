use std::sync::Arc;

use dokka_scene::{Divider, Dot, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, network_language as measures, tray_glass as glass};

use super::layout::{circle_in, separator_rect, settings_row, status_row, title_row};
use super::{NetworkHit, NetworkIcons, NetworkType};

/// Alto fijo del panel de red.
pub fn network_size() -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::network_height(),
    }
}

/// Un solo cristal oscuro: título, estado con círculo e internet, línea
/// y ajustes. `tiles` vacío y cantidad constante: sin planos, una línea,
/// un punto, cuatro textos y dos iconos.
pub fn network_panel(
    theme: &Theme,
    icons: &NetworkIcons,
    bounds: Rect,
    name: &str,
    kind: NetworkType,
    has_internet: bool,
    highlighted: Option<NetworkHit>,
) -> Overlay {
    let title = title_row(bounds);
    let status = status_row(bounds);
    let settings = settings_row(bounds);
    let is_active = kind != NetworkType::Offline;
    let circle = circle_in(status);
    Overlay {
        panel: glass(theme, bounds),
        tiles: Vec::new(),
        panel_shadow: None,
        tile_shadow: None,
        pill: None,
        highlight: Some(settings_highlight(theme, settings, highlighted)),
        flat_panels: Vec::new(),
        dividers: vec![separator(theme, bounds)],
        dots: vec![Dot {
            x: circle.x,
            y: circle.y,
            diameter: circle.width,
            color: if is_active {
                theme.tray.circle_active
            } else {
                theme.tray.circle
            },
        }],
        pointer: None,
        texts: vec![
            Text {
                x: title.x + glass::ROW_MARGIN + glass::ROW_INSET,
                center_y: title.y + title.height / 2.0,
                content: measures::NETWORK_TITLE.to_owned(),
                size: measures::TITLE_SIZE,
                is_bold: true,
                color: theme.tray.text,
                align: TextAlign::Left,
            },
            Text {
                x: circle.x + circle.width + measures::ICON_GAP,
                center_y: status.y + status.height / 2.0 - 11.0,
                content: kind_label(kind),
                size: measures::TITLE_SIZE,
                is_bold: true,
                color: theme.tray.text,
                align: TextAlign::Left,
            },
            Text {
                x: circle.x + circle.width + measures::ICON_GAP,
                center_y: status.y + status.height / 2.0 + 11.0,
                content: display_name(name, kind, has_internet),
                size: measures::SUBTITLE_SIZE,
                is_bold: false,
                color: theme.tray.secondary,
                align: TextAlign::Left,
            },
            Text {
                x: settings.x + glass::ROW_INSET,
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
                x: circle.x + (circle.width - measures::CIRCLE_ICON) / 2.0,
                y: circle.y + (circle.height - measures::CIRCLE_ICON) / 2.0,
                side: measures::CIRCLE_ICON,
                image: Some(IconImage::Drawing(icons.for_circle(kind, is_active))),
                placeholder: theme.tray.panel,
                placeholder_radius: 0.0,
            },
            Icon {
                x: settings.x + settings.width - glass::ROW_INSET - measures::CHEVRON_SIDE,
                y: settings.y + (settings.height - measures::CHEVRON_SIDE) / 2.0,
                side: measures::CHEVRON_SIDE,
                image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
                placeholder: theme.tray.panel,
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

/// Línea fina entre el estado y los ajustes, con el color del cristal.
fn separator(theme: &Theme, bounds: Rect) -> Divider {
    Divider {
        bounds: separator_rect(bounds),
        color: theme.tray.separator,
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

fn settings_highlight(theme: &Theme, row: Rect, highlighted: Option<NetworkHit>) -> Panel {
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
    if highlighted != Some(NetworkHit::Settings) {
        return hidden;
    }
    Panel {
        bounds: row,
        corner_radius: capped(row, measures::HIGHLIGHT_RADIUS),
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    }
}

/// Tipo de conexión como título de la fila (Cable/Wi-Fi).
fn kind_label(kind: NetworkType) -> String {
    match kind {
        NetworkType::Offline => measures::TYPE_OFFLINE.to_owned(),
        NetworkType::Ethernet => measures::TYPE_ETHERNET.to_owned(),
        NetworkType::Wifi => measures::TYPE_WIFI.to_owned(),
        NetworkType::Other => measures::TYPE_OTHER.to_owned(),
    }
}

fn display_name(name: &str, kind: NetworkType, has_internet: bool) -> String {
    if kind == NetworkType::Offline || name.trim().is_empty() {
        return measures::OFFLINE_NAME.to_owned();
    }
    let short = short(name);
    if has_internet {
        short
    } else {
        format!("{short} · {}", measures::WITHOUT_INTERNET)
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
