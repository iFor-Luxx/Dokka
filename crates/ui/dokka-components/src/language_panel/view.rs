use dokka_scene::{Divider, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, network_language as measures, typography};

use super::{LanguageEntry, LanguageHit, LanguageIcons};

/// Alto del panel para estos idiomas.
pub fn language_size(languages: &[LanguageEntry]) -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::language_height(languages.len()),
    }
}

/// Panel con la lista de idiomas y la fila de ajustes.
pub fn language_panel(
    theme: &Theme,
    icons: &LanguageIcons,
    bounds: Rect,
    languages: &[LanguageEntry],
    highlighted: Option<LanguageHit>,
) -> Overlay {
    let label_x = bounds.x + measures::PADDING + measures::TEXT_INSET + measures::CHECK_WIDTH;
    let mut texts = Vec::with_capacity(languages.len() + 1);
    for (index, language) in languages.iter().enumerate() {
        let is_highlighted = highlighted == Some(LanguageHit::Language(language.id));
        texts.push(Text {
            x: label_x,
            center_y: row_top(bounds, index) + measures::ROW_HEIGHT / 2.0,
            content: short(&language.name),
            size: typography::MENU_SIZE,
            is_bold: false,
            color: if is_highlighted {
                theme.menu.highlighted_text
            } else {
                theme.menu.text
            },
            align: TextAlign::Left,
        });
    }
    let settings_top = settings_row_top(bounds, languages.len());
    texts.push(Text {
        x: label_x,
        center_y: settings_top + measures::ROW_HEIGHT / 2.0,
        content: measures::LANGUAGE_SETTINGS_LABEL.to_owned(),
        size: typography::MENU_SIZE,
        is_bold: false,
        color: if highlighted == Some(LanguageHit::Settings) {
            theme.menu.highlighted_text
        } else {
            theme.menu.text
        },
        align: TextAlign::Left,
    });

    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::CORNER_RADIUS,
            border_width: measures::BORDER_WIDTH,
            tint: theme.menu.background,
            border: theme.menu.border,
        },
        pill: None,
        highlight: Some(highlight_panel(theme, bounds, languages, highlighted)),
        flat_panels: Vec::new(),
        dividers: vec![Divider {
            bounds: separator(bounds, languages.len()),
            color: theme.menu.separator,
        }],
        dots: Vec::new(),
        pointer: None,
        texts,
        icons: check_icons(theme, icons, bounds, languages, highlighted),
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
    if x < bounds.x + measures::PADDING
        || x >= bounds.x + bounds.width - measures::PADDING
    {
        return None;
    }
    for (index, language) in languages.iter().enumerate() {
        let top = row_top(bounds, index);
        if y >= top && y < top + measures::ROW_HEIGHT {
            return Some(LanguageHit::Language(language.id));
        }
    }
    let top = settings_row_top(bounds, languages.len());
    if y >= top && y < top + measures::ROW_HEIGHT {
        return Some(LanguageHit::Settings);
    }
    None
}

fn row_top(bounds: Rect, index: usize) -> f32 {
    bounds.y + measures::PADDING + index as f32 * measures::ROW_HEIGHT
}

fn settings_row_top(bounds: Rect, count: usize) -> f32 {
    bounds.y + measures::PADDING + count as f32 * measures::ROW_HEIGHT + measures::SEPARATOR_HEIGHT
}

fn row_bounds(bounds: Rect, top: f32) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: top,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
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
        tint: theme.menu.highlight,
        border: theme.menu.highlight,
    };
    let top = match highlighted {
        Some(LanguageHit::Settings) => settings_row_top(bounds, languages.len()),
        Some(LanguageHit::Language(id)) => {
            let Some(index) = languages.iter().position(|entry| entry.id == id) else {
                return hidden;
            };
            row_top(bounds, index)
        }
        None => return hidden,
    };
    Panel {
        bounds: row_bounds(bounds, top),
        ..hidden
    }
}

fn check_icons(
    theme: &Theme,
    icons: &LanguageIcons,
    bounds: Rect,
    languages: &[LanguageEntry],
    highlighted: Option<LanguageHit>,
) -> Vec<Icon> {
    languages
        .iter()
        .enumerate()
        .map(|(index, language)| {
            if !language.is_active {
                return Icon {
                    x: bounds.x + measures::PADDING + measures::TEXT_INSET,
                    y: row_top(bounds, index),
                    side: 0.0,
                    image: None,
                    placeholder: theme.menu.background,
                    placeholder_radius: 0.0,
                };
            }
            let is_highlighted = highlighted == Some(LanguageHit::Language(language.id));
            Icon {
                x: bounds.x
                    + measures::PADDING
                    + measures::TEXT_INSET
                    + (measures::CHECK_WIDTH - measures::CHECK_SIDE) / 2.0,
                y: row_top(bounds, index) + (measures::ROW_HEIGHT - measures::CHECK_SIDE) / 2.0,
                side: measures::CHECK_SIDE,
                image: Some(IconImage::Drawing(icons.for_row(is_highlighted))),
                placeholder: theme.menu.background,
                placeholder_radius: 0.0,
            }
        })
        .collect()
}

fn separator(bounds: Rect, count: usize) -> Rect {
    let middle = bounds.y
        + measures::PADDING
        + count as f32 * measures::ROW_HEIGHT
        + measures::SEPARATOR_HEIGHT / 2.0;
    Rect {
        x: bounds.x + measures::PADDING + measures::TEXT_INSET,
        y: middle - measures::DIVIDER_HEIGHT / 2.0,
        width: bounds.width - (measures::PADDING + measures::TEXT_INSET) * 2.0,
        height: measures::DIVIDER_HEIGHT,
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
