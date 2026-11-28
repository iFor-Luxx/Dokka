//! Lista de categorías del apartado "Almacenamiento".
//!
//! Cada fila trae su cuadrito de color (el mismo que su tramo de la barra,
//! y por eso sirve de leyenda), el nombre y el tamaño a la derecha
//! ("Calculando…" mientras se calcula). Lo que queda fuera de la vista se
//! lleva a tamaño cero para no reconstruir capas con la rueda.

use std::sync::Arc;

use dokka_components::settings_group::{SettingsGroupProps, group_dividers, group_panel};
use dokka_model::StorageBreakdown;
use dokka_scene::{Divider, Drawing, Icon, Panel, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{storage_category_color, storage_list_rows};
use super::general_rows::hidden_icon;
use super::storage_cards::{display_size, hidden_card, hide_dividers};

/// Tarjeta de categorías con cuadrito, nombre y tamaño a la derecha.
pub(crate) fn category_card(
    theme: &Theme,
    layout: &super::super::logic::StorageLayout,
    badges: &[Arc<Drawing>],
    glyphs: &[Arc<Drawing>],
    breakdown: Option<&StorageBreakdown>,
    width: f32,
    window_height: f32,
) -> (Panel, Vec<Divider>, Vec<Text>, Vec<Icon>) {
    use super::super::storage_state::storage_category_names;
    let names = storage_category_names();
    let heights = vec![measures::CONTENT_ROW_HEIGHT; storage_list_rows()];
    let props = SettingsGroupProps {
        top: layout.list,
        rows: names.iter().map(|name| name.to_string()).collect(),
        heights: heights.clone(),
        width,
    };
    if layout.list + heights.iter().sum::<f32>() < measures::CONTENT_TOP
        || layout.list > window_height
    {
        return (hidden_card(theme), Vec::new(), Vec::new(), Vec::new());
    }
    let panel = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);
    let sizes = category_sizes(breakdown);
    let mut texts = Vec::new();
    let mut icons = Vec::new();
    for (index, name) in names.iter().enumerate() {
        let row_top = layout.list + index as f32 * measures::CONTENT_ROW_HEIGHT;
        let middle = row_top + measures::CONTENT_ROW_HEIGHT / 2.0;
        if middle < measures::CONTENT_TOP || middle > window_height {
            icons.push(hidden_icon(theme));
            icons.push(hidden_icon(theme));
            continue;
        }
        let color = storage_category_color(theme, index);
        let badge = badges.get(index).cloned().unwrap_or_else(|| badges[0].clone());
        let glyph = glyphs.get(index).cloned().unwrap_or_else(|| glyphs[0].clone());
        let badge_x = measures::CONTENT_LEFT + measures::CONTENT_PADDING;
        let badge_y = row_top + (measures::CONTENT_ROW_HEIGHT - measures::BADGE_SIDE) / 2.0;
        let inset = (measures::BADGE_SIDE - measures::ROW_ICON_SIDE) / 2.0;
        icons.push(Icon {
            x: badge_x,
            y: badge_y,
            side: measures::BADGE_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(badge)),
            placeholder: color,
            placeholder_radius: measures::BADGE_RADIUS,
        });
        icons.push(Icon {
            x: badge_x + inset,
            y: badge_y + inset,
            side: measures::ROW_ICON_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(glyph)),
            placeholder: color,
            placeholder_radius: 0.0,
        });
        texts.push(Text {
            x: badge_x + measures::BADGE_SIDE + measures::BADGE_TEXT_GAP,
            center_y: middle,
            content: name.to_string(),
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.row_text,
            align: TextAlign::Left,
        });
        texts.push(Text {
            x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
            center_y: middle,
            content: sizes[index]
                .map(display_size)
                .unwrap_or_else(|| "Calculando…".to_owned()),
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Right,
        });
    }
    let dividers = hide_dividers(dividers, window_height);
    (panel, dividers, texts, icons)
}

/// Tamaños de las nueve categorías en orden; `None` es "Calculando…".
fn category_sizes(breakdown: Option<&StorageBreakdown>) -> [Option<u64>; 9] {
    let get = |pick: fn(&StorageBreakdown) -> Option<u64>| breakdown.and_then(pick);
    [
        get(|known| known.apps_bytes),
        get(|known| known.documents_bytes),
        get(|known| known.images_bytes),
        get(|known| known.music_bytes),
        get(|known| known.videos_bytes),
        get(|known| known.downloads_bytes),
        get(|known| known.recycle_bytes),
        get(|known| known.windows_bytes),
        breakdown.and_then(|known| known.other_bytes()),
    ]
}
