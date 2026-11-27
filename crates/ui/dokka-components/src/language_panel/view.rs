use std::sync::Arc;

use dokka_scene::{Divider, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, network_language as measures, tray_glass as glass};

use super::layout::{code_rect, row_rect, separator_rect, settings_row, title_row};
use super::{LanguageEntry, LanguageHit, LanguageIcons};

/// Alto del panel para estos idiomas.
pub fn language_size(languages: &[LanguageEntry]) -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::language_height(languages.len()),
    }
}

/// Un solo cristal oscuro: título, una fila compacta por idioma con marca
/// y código, línea y ajustes. `tiles` vacío; planos y textos crecen con
/// las filas, el resto es constante.
pub fn language_panel(
    theme: &Theme,
    icons: &LanguageIcons,
    bounds: Rect,
    languages: &[LanguageEntry],
    highlighted: Option<LanguageHit>,
) -> Overlay {
    let title = title_row(bounds);
    let mut flat_panels = Vec::with_capacity(languages.len());
    for (index, language) in languages.iter().enumerate() {
        flat_panels.push(code_box(theme, code_rect(bounds, index, language)));
    }
    let mut texts = Vec::with_capacity(languages.len() * 2 + 2);
    texts.push(Text {
        x: title.x + glass::ROW_MARGIN + glass::ROW_INSET,
        center_y: title.y + title.height / 2.0,
        content: measures::LANGUAGE_TITLE.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Left,
    });
    for (index, language) in languages.iter().enumerate() {
        let row = row_rect(bounds, index);
        texts.push(Text {
            x: row.x + glass::ROW_INSET + measures::CHECK_SIDE + measures::ICON_GAP,
            center_y: row.y + row.height / 2.0,
            content: short(&language.name),
            size: measures::TITLE_SIZE,
            is_bold: language.is_active,
            color: theme.tray.text,
            align: TextAlign::Left,
        });
        let code = code_rect(bounds, index, language);
        texts.push(Text {
            x: code.x + code.width / 2.0,
            center_y: code.y + code.height / 2.0,
            content: language_code(language),
            size: measures::SUBTITLE_SIZE,
            is_bold: false,
            color: theme.tray.secondary,
            align: TextAlign::Center,
        });
    }
    let settings = settings_row(bounds, languages.len());
    texts.push(Text {
        x: settings.x + glass::ROW_INSET,
        center_y: settings.y + settings.height / 2.0,
        content: measures::LANGUAGE_SETTINGS_LABEL.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: theme.tray.text,
        align: TextAlign::Left,
    });

    let mut panel_icons = Vec::with_capacity(languages.len() + 1);
    for (index, language) in languages.iter().enumerate() {
        let row = row_rect(bounds, index);
        panel_icons.push(check_icon(theme, icons, row, language));
    }
    let chevron_side = measures::CHEVRON_SIDE;
    panel_icons.push(Icon {
        x: settings.x + settings.width - glass::ROW_INSET - chevron_side,
        y: settings.y + (settings.height - chevron_side) / 2.0,
        side: chevron_side,
        image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
        placeholder: theme.tray.panel,
        placeholder_radius: 0.0,
    });

    Overlay {
        panel: glass(theme, bounds),
        tiles: Vec::new(),
        panel_shadow: None,
        tile_shadow: None,
        pill: None,
        highlight: Some(highlight_panel(theme, bounds, languages, highlighted)),
        flat_panels,
        dividers: vec![separator(theme, bounds, languages.len())],
        dots: Vec::new(),
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

/// Línea fina entre las filas y los ajustes, con el color del cristal.
fn separator(theme: &Theme, bounds: Rect, count: usize) -> Divider {
    Divider {
        bounds: separator_rect(bounds, count),
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

fn code_box(theme: &Theme, code: Rect) -> Panel {
    Panel {
        bounds: code,
        corner_radius: capped(code, measures::CODE_RADIUS),
        border_width: 1.0,
        tint: theme.tray.circle,
        border: theme.tray.tile_border,
    }
}

/// Marca de verificación a la izquierda solo en el idioma activo.
/// Existe siempre (lado cero si no está activo) para no reconstruir capas.
fn check_icon(theme: &Theme, icons: &LanguageIcons, row: Rect, language: &LanguageEntry) -> Icon {
    if language.is_active {
        Icon {
            x: row.x + glass::ROW_INSET,
            y: row.y + (row.height - measures::CHECK_SIDE) / 2.0,
            side: measures::CHECK_SIDE,
            image: Some(IconImage::Drawing(icons.check())),
            placeholder: theme.tray.panel,
            placeholder_radius: 0.0,
        }
    } else {
        Icon {
            x: row.x + glass::ROW_INSET,
            y: row.y + row.height / 2.0,
            side: 0.0,
            image: None,
            placeholder: theme.tray.panel,
            placeholder_radius: 0.0,
        }
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
        corner_radius: 0.0,
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    };
    let row = match highlighted {
        Some(LanguageHit::Settings) => settings_row(bounds, languages.len()),
        Some(LanguageHit::Language(id)) => {
            let Some(index) = languages.iter().position(|entry| entry.id == id) else {
                return hidden;
            };
            row_rect(bounds, index)
        }
        None => return hidden,
    };
    Panel {
        bounds: row,
        corner_radius: capped(row, measures::HIGHLIGHT_RADIUS),
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    }
}

/// Código corto del idioma para su recuadro (dos primeras letras).
fn language_code(language: &LanguageEntry) -> String {
    language
        .name
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase()
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
