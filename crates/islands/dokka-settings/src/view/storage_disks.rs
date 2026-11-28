//! Tarjeta de otros discos del apartado "Almacenamiento".
//!
//! Cada fila trae el disco a la izquierda y "X libres de Y" a la derecha.
//! Solo existe si hay otros discos fijos. Lo que queda fuera de la vista se
//! lleva a tamaño cero para no reconstruir capas con la rueda.

use dokka_components::settings_group::{SettingsGroupProps, group_dividers, group_panel};
use dokka_model::StorageBreakdown;
use dokka_scene::{Divider, Panel, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::storage_cards::{display_size, hidden_card, hide_dividers};

/// Tarjeta de otros discos con "X libres de Y" a la derecha.
pub(crate) fn disks_card(
    theme: &Theme,
    layout: &super::super::logic::StorageLayout,
    breakdown: Option<&StorageBreakdown>,
    width: f32,
    window_height: f32,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let Some(known) = breakdown else {
        return (hidden_card(theme), Vec::new(), Vec::new());
    };
    let heights = vec![measures::CONTENT_ROW_HEIGHT; known.other_disks.len()];
    let rows: Vec<String> = known
        .other_disks
        .iter()
        .map(|disk| format!("Disco local ({})", disk.drive))
        .collect();
    let props = SettingsGroupProps {
        top: layout.disks,
        rows,
        heights: heights.clone(),
        width,
    };
    if layout.disks + heights.iter().sum::<f32>() < measures::CONTENT_TOP
        || layout.disks > window_height
    {
        return (hidden_card(theme), Vec::new(), Vec::new());
    }
    let panel = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);
    let mut texts = Vec::new();
    for (index, disk) in known.other_disks.iter().enumerate() {
        let middle = layout.disks
            + index as f32 * measures::CONTENT_ROW_HEIGHT
            + measures::CONTENT_ROW_HEIGHT / 2.0;
        if middle < measures::CONTENT_TOP || middle > window_height {
            continue;
        }
        texts.push(Text {
            x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
            center_y: middle,
            content: format!("Disco local ({})", disk.drive),
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.row_text,
            align: TextAlign::Left,
        });
        texts.push(Text {
            x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
            center_y: middle,
            content: format!(
                "{} libres de {}",
                display_size(disk.free_bytes),
                display_size(disk.total_bytes)
            ),
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Right,
        });
    }
    let dividers = hide_dividers(dividers, window_height);
    (panel, dividers, texts)
}
