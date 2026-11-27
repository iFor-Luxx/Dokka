use std::sync::Arc;

use dokka_scene::{Dot, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, network_language as measures};

use super::{LanguageEntry, LanguageHit, LanguageIcons};

/// Alto del panel para estos idiomas.
pub fn language_size(languages: &[LanguageEntry]) -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::language_height(languages.len()),
    }
}

/// Menú Tahoe: título, una cápsula por idioma con círculo y ajustes aparte.
/// El `panel` es invisible; cada pieza va en `tiles` con cristal ahumado.
/// Sin marca de verificación: el círculo activo ya indica el idioma en uso.
pub fn language_panel(
    theme: &Theme,
    icons: &LanguageIcons,
    bounds: Rect,
    languages: &[LanguageEntry],
    highlighted: Option<LanguageHit>,
) -> Overlay {
    let title = title_tile(bounds);
    let mut tiles = Vec::with_capacity(languages.len() + 2);
    tiles.push(glass(theme, title, measures::TILE_RADIUS));
    for index in 0..languages.len() {
        let row = row_tile(bounds, index);
        tiles.push(glass(theme, row, row.height / 2.0));
    }
    let settings = settings_tile(bounds, languages.len());
    tiles.push(glass(theme, settings, measures::TILE_RADIUS));

    let mut dots = Vec::with_capacity(languages.len() + 1);
    for (index, language) in languages.iter().enumerate() {
        let circle = circle_in(row_tile(bounds, index));
        dots.push(Dot {
            x: circle.x,
            y: circle.y,
            diameter: circle.width,
            color: if language.is_active {
                theme.tray.circle_active
            } else {
                theme.tray.circle
            },
        });
    }
    let settings_circle = circle_in(settings);
    dots.push(Dot {
        x: settings_circle.x,
        y: settings_circle.y,
        diameter: settings_circle.width,
        color: theme.tray.circle,
    });

    let mut texts = Vec::with_capacity(languages.len() * 2 + 2);
    texts.push(Text {
        x: title.x + measures::PADDING,
        center_y: title.y + title.height / 2.0,
        content: measures::LANGUAGE_TITLE.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Left,
    });
    for (index, language) in languages.iter().enumerate() {
        let row = row_tile(bounds, index);
        let text_x = row.x + measures::PADDING + measures::CIRCLE + measures::ICON_GAP;
        texts.push(Text {
            x: text_x,
            center_y: row.y + row.height / 2.0 - 11.0,
            content: short(&language.name),
            size: measures::TITLE_SIZE,
            is_bold: true,
            color: theme.tray.text,
            align: TextAlign::Left,
        });
        texts.push(Text {
            x: text_x,
            center_y: row.y + row.height / 2.0 + 11.0,
            content: active_subtitle(language),
            size: measures::SUBTITLE_SIZE,
            is_bold: false,
            color: theme.tray.secondary,
            align: TextAlign::Left,
        });
    }
    texts.push(Text {
        x: settings_circle.x + settings_circle.width + measures::ICON_GAP,
        center_y: settings.y + settings.height / 2.0,
        content: measures::LANGUAGE_SETTINGS_LABEL.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: theme.tray.text,
        align: TextAlign::Left,
    });

    let mut panel_icons = Vec::with_capacity(languages.len() + 2);
    for (index, language) in languages.iter().enumerate() {
        let circle = circle_in(row_tile(bounds, index));
        panel_icons.push(Icon {
            x: circle.x + (circle.width - measures::CIRCLE_ICON) / 2.0,
            y: circle.y + (circle.height - measures::CIRCLE_ICON) / 2.0,
            side: measures::CIRCLE_ICON,
            image: Some(IconImage::Drawing(icons.for_row(language.is_active))),
            placeholder: theme.tray.tile,
            placeholder_radius: 0.0,
        });
    }
    panel_icons.push(Icon {
        x: settings_circle.x + (settings_circle.width - measures::CIRCLE_ICON) / 2.0,
        y: settings_circle.y + (settings_circle.height - measures::CIRCLE_ICON) / 2.0,
        side: measures::CIRCLE_ICON,
        image: Some(IconImage::Drawing(Arc::clone(icons.settings()))),
        placeholder: theme.tray.tile,
        placeholder_radius: 0.0,
    });
    let chevron_side = measures::CHEVRON_SIDE;
    panel_icons.push(Icon {
        x: settings.x + settings.width - measures::PADDING - chevron_side,
        y: settings.y + (settings.height - chevron_side) / 2.0,
        side: chevron_side,
        image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
        placeholder: theme.tray.tile,
        placeholder_radius: 0.0,
    });

    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::TILE_RADIUS,
            border_width: 0.0,
            tint: theme.tray.clear,
            border: theme.tray.clear,
        },
        tiles,
        pill: None,
        highlight: Some(highlight_panel(theme, bounds, languages, highlighted)),
        flat_panels: Vec::new(),
        dividers: Vec::new(),
        dots,
        pointer: None,
        texts,
        icons: panel_icons,
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Fila que hay en el punto, si es elegible.
pub fn language_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    languages: &[LanguageEntry],
) -> Option<LanguageHit> {
    if x < bounds.x + measures::GAP || x >= bounds.x + bounds.width - measures::GAP {
        return None;
    }
    for (index, language) in languages.iter().enumerate() {
        let row = row_tile(bounds, index);
        if y >= row.y && y < row.y + row.height {
            return Some(LanguageHit::Language(language.id));
        }
    }
    let settings = settings_tile(bounds, languages.len());
    if y >= settings.y && y < settings.y + settings.height {
        return Some(LanguageHit::Settings);
    }
    None
}

fn title_tile(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::GAP,
        y: bounds.y + measures::GAP,
        width: bounds.width - measures::GAP * 2.0,
        height: measures::TITLE_HEIGHT,
    }
}

fn row_tile(bounds: Rect, index: usize) -> Rect {
    Rect {
        x: bounds.x + measures::GAP,
        y: bounds.y + measures::GAP * 2.0 + measures::TITLE_HEIGHT + index as f32 * (measures::ROW_HEIGHT + measures::GAP),
        width: bounds.width - measures::GAP * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

fn settings_tile(bounds: Rect, count: usize) -> Rect {
    Rect {
        x: bounds.x + measures::GAP,
        y: bounds.y
            + measures::GAP * 2.0
            + measures::TITLE_HEIGHT
            + count as f32 * (measures::ROW_HEIGHT + measures::GAP),
        width: bounds.width - measures::GAP * 2.0,
        height: measures::SETTINGS_HEIGHT,
    }
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

fn highlight_panel(
    theme: &Theme,
    bounds: Rect,
    languages: &[LanguageEntry],
    highlighted: Option<LanguageHit>,
) -> Panel {
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
    let row = match highlighted {
        Some(LanguageHit::Settings) => settings_tile(bounds, languages.len()),
        Some(LanguageHit::Language(id)) => {
            let Some(index) = languages.iter().position(|entry| entry.id == id) else {
                return hidden;
            };
            row_tile(bounds, index)
        }
        None => return hidden,
    };
    Panel {
        bounds: row,
        corner_radius: glass_radius(row),
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    }
}

fn glass_radius(row: Rect) -> f32 {
    if (row.height - measures::ROW_HEIGHT).abs() < 0.01 {
        row.height / 2.0
    } else {
        measures::TILE_RADIUS.min(row.height / 2.0)
    }
}

fn active_subtitle(language: &LanguageEntry) -> String {
    if language.is_active {
        measures::ACTIVE_SUBTITLE.to_owned()
    } else {
        String::new()
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
