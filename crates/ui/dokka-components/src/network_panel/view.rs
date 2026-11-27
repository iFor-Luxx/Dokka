use dokka_scene::{Divider, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, network_language as measures, typography};

use super::{NetworkHit, NetworkIcons, NetworkType};

/// Alto fijo del panel de red.
pub fn network_size() -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::network_height(),
    }
}

/// Panel con tarjeta de estado y fila de ajustes.
pub fn network_panel(
    theme: &Theme,
    icons: &NetworkIcons,
    bounds: Rect,
    name: &str,
    kind: NetworkType,
    has_internet: bool,
    highlighted: Option<NetworkHit>,
) -> Overlay {
    let card = header_card(bounds);
    let settings = settings_row(bounds);
    let settings_color = if highlighted == Some(NetworkHit::Settings) {
        theme.menu.highlighted_text
    } else {
        theme.menu.text
    };
    let icon_x = card.x + measures::PADDING + measures::TEXT_INSET;
    let text_x = icon_x + measures::HEADER_ICON_SIDE + measures::HEADER_GAP;
    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::CORNER_RADIUS,
            border_width: measures::BORDER_WIDTH,
            tint: theme.menu.background,
            border: theme.menu.border,
        },
        pill: None,
        highlight: Some(settings_highlight(theme, settings, highlighted)),
        flat_panels: vec![Panel {
            bounds: card,
            corner_radius: measures::CARD_RADIUS,
            border_width: 0.0,
            tint: theme.panel.highlight,
            border: theme.panel.highlight,
        }],
        dividers: vec![Divider {
            bounds: separator(bounds),
            color: theme.menu.separator,
        }],
        dots: Vec::new(),
        pointer: None,
        texts: vec![
            Text {
                x: text_x,
                center_y: card.y + measures::HEADER_HEIGHT / 2.0 - 11.0,
                content: display_name(name, kind),
                size: typography::TITLE_SIZE,
                is_bold: true,
                color: theme.menu.text,
                align: TextAlign::Left,
            },
            Text {
                x: text_x,
                center_y: card.y + measures::HEADER_HEIGHT / 2.0 + 11.0,
                content: detail(kind, has_internet),
                size: typography::BODY_SIZE,
                is_bold: false,
                color: theme.menu.text,
                align: TextAlign::Left,
            },
            Text {
                x: bounds.x + measures::PADDING + measures::TEXT_INSET,
                center_y: settings.y + measures::ROW_HEIGHT / 2.0,
                content: measures::NETWORK_SETTINGS_LABEL.to_owned(),
                size: typography::MENU_SIZE,
                is_bold: false,
                color: settings_color,
                align: TextAlign::Left,
            },
        ],
        icons: vec![Icon {
            x: icon_x,
            y: card.y + (measures::HEADER_HEIGHT - measures::HEADER_ICON_SIDE) / 2.0,
            side: measures::HEADER_ICON_SIDE,
            image: Some(IconImage::Drawing(icons.for_panel(kind))),
            placeholder: theme.panel.highlight,
            placeholder_radius: 0.0,
        }],
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Fila de ajustes que hay en el punto, si cae en ella.
pub fn network_hit(bounds: Rect, x: f32, y: f32) -> Option<NetworkHit> {
    let row = settings_row(bounds);
    if x >= row.x && x < row.x + row.width && y >= row.y && y < row.y + row.height {
        Some(NetworkHit::Settings)
    } else {
        None
    }
}

fn header_card(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: bounds.y + measures::PADDING,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::HEADER_HEIGHT,
    }
}

fn settings_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: bounds.y + measures::PADDING + measures::HEADER_HEIGHT + measures::SEPARATOR_HEIGHT,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

fn separator(bounds: Rect) -> Rect {
    let middle =
        bounds.y + measures::PADDING + measures::HEADER_HEIGHT + measures::SEPARATOR_HEIGHT / 2.0;
    Rect {
        x: bounds.x + measures::PADDING + measures::TEXT_INSET,
        y: middle - measures::DIVIDER_HEIGHT / 2.0,
        width: bounds.width - (measures::PADDING + measures::TEXT_INSET) * 2.0,
        height: measures::DIVIDER_HEIGHT,
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
        tint: theme.menu.highlight,
        border: theme.menu.highlight,
    };
    if highlighted != Some(NetworkHit::Settings) {
        return hidden;
    }
    Panel {
        bounds: row,
        ..hidden
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
