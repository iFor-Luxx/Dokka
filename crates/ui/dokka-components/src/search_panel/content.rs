use dokka_scene::{Icon, IconImage, Rect, Text, TextAlign};
use dokka_theme::{Theme, search as measures};

use super::{SearchPanelIcons, SearchPanelProps};
use crate::search_field::{search_label_at, search_magnifier_at};

/// Consulta más siete filas más el vacío: siempre nueve textos.
pub(super) fn panel_texts(
    theme: &Theme,
    bounds: Rect,
    search: &Rect,
    props: &SearchPanelProps,
    visible: usize,
) -> Vec<Text> {
    let mut texts = Vec::with_capacity(1 + measures::MAX_RESULTS + 1);
    let query = if props.query.is_empty() {
        measures::SEARCH_PLACEHOLDER.to_owned()
    } else {
        props.query.clone()
    };
    texts.push(search_label_at(
        theme.panel.text,
        search.x + measures::SEARCH_TEXT_INSET,
        search.y + search.height / 2.0,
        &query,
        measures::SEARCH_TEXT_SIZE,
    ));
    for slot in 0..measures::MAX_RESULTS {
        let (content, color) = if slot < visible {
            let highlighted = props.highlighted == Some(slot);
            (
                short_name(&props.results[slot].name),
                if highlighted {
                    theme.menu.highlighted_text
                } else {
                    theme.panel.text
                },
            )
        } else {
            (String::new(), theme.panel.text)
        };
        texts.push(Text {
            x: bounds.x
                + measures::ROW_MARGIN
                + measures::SEARCH_ICON_INSET
                + measures::ROW_ICON_SIDE
                + measures::ROW_TEXT_GAP,
            center_y: measures::row_top(bounds, slot) + measures::ROW_HEIGHT / 2.0,
            content,
            size: measures::ROW_TEXT_SIZE,
            is_bold: false,
            color,
            align: TextAlign::Left,
        });
    }
    texts.push(Text {
        x: bounds.x + bounds.width / 2.0,
        center_y: measures::row_top(bounds, 0) + measures::ROW_HEIGHT / 2.0,
        content: if visible == 0 {
            measures::EMPTY_TEXT.to_owned()
        } else {
            String::new()
        },
        size: measures::EMPTY_TEXT_SIZE,
        is_bold: false,
        color: theme.panel.text,
        align: TextAlign::Center,
    });
    texts
}

/// Lupa más siete iconos de app: siempre ocho iconos.
pub(super) fn panel_icons(
    theme: &Theme,
    drawings: &SearchPanelIcons,
    bounds: Rect,
    search: &Rect,
    props: &SearchPanelProps,
    visible: usize,
) -> Vec<Icon> {
    let mut icons = Vec::with_capacity(1 + measures::MAX_RESULTS);
    icons.push(search_magnifier_at(
        theme.panel.text,
        search.x + measures::SEARCH_ICON_INSET,
        search.y + (search.height - measures::SEARCH_ICON_SIDE) / 2.0,
        measures::SEARCH_ICON_SIDE,
        drawings.field(),
    ));
    for slot in 0..measures::MAX_RESULTS {
        if let Some(result) = props.results.get(slot).filter(|_| slot < visible) {
            icons.push(Icon {
                x: bounds.x + measures::ROW_MARGIN + measures::SEARCH_ICON_INSET,
                y: measures::row_top(bounds, slot)
                    + (measures::ROW_HEIGHT - measures::ROW_ICON_SIDE) / 2.0,
                side: measures::ROW_ICON_SIDE,
                image: result.image.clone().map(IconImage::Pixels),
                placeholder: theme.panel.highlight,
                placeholder_radius: measures::ROW_ICON_RADIUS,
            });
        } else {
            icons.push(Icon {
                x: 0.0,
                y: 0.0,
                side: 0.0,
                image: None,
                placeholder: theme.panel.highlight,
                placeholder_radius: measures::ROW_ICON_RADIUS,
            });
        }
    }
    icons
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
