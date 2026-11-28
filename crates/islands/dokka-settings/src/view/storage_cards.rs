//! Tarjeta de resumen del apartado "Almacenamiento": disco y uso.
//!
//! Arriba el disco del sistema con su total; debajo "X usados de Y". La
//! barra vive en `storage_bar`. Lo que queda fuera de la vista se lleva a
//! tamaño cero para no reconstruir capas con la rueda.

use dokka_components::settings_group::{SettingsGroupProps, group_dividers, group_panel};
use dokka_model::StorageBreakdown;
use dokka_scene::{Divider, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

/// Tarjeta de resumen: disco, "X usados de Y" y la barra.
pub(crate) fn header_pieces(
    theme: &Theme,
    layout: &super::super::logic::StorageLayout,
    heights: &[f32; 3],
    width: f32,
    window_height: f32,
    breakdown: Option<&StorageBreakdown>,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let total_height: f32 = heights.iter().sum();
    if layout.header + total_height < measures::CONTENT_TOP || layout.header > window_height {
        return (hidden_card(theme), Vec::new(), Vec::new());
    }
    let props = SettingsGroupProps {
        top: layout.header,
        rows: ["".to_owned(), "".to_owned(), "".to_owned()].to_vec(),
        heights: heights.to_vec(),
        width,
    };
    let panel = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);
    let mut texts = Vec::new();
    let drive_name = breakdown
        .and_then(|known| known.drive.clone())
        .map(|drive| format!("Disco del sistema ({drive})"))
        .unwrap_or_else(|| "Disco del sistema".to_owned());
    let total_text = breakdown
        .and_then(|known| known.total_bytes)
        .map(display_size)
        .unwrap_or_else(|| "Calculando…".to_owned());
    let used = breakdown.and_then(|known| known.used_bytes());
    let total = breakdown.and_then(|known| known.total_bytes);
    let used_text = match (used, total) {
        (Some(used), Some(total)) => {
            format!("{} usados de {}", display_size(used), display_size(total))
        }
        _ => "Calculando…".to_owned(),
    };
    let name_middle = layout.header + heights[0] / 2.0;
    if name_middle >= measures::CONTENT_TOP && name_middle <= window_height {
        texts.push(Text {
            x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
            center_y: name_middle,
            content: drive_name,
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: true,
            color: theme.settings.row_text,
            align: TextAlign::Left,
        });
        texts.push(Text {
            x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
            center_y: name_middle,
            content: total_text,
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Right,
        });
    }
    let used_middle = layout.header + heights[0] + heights[1] / 2.0;
    if used_middle >= measures::CONTENT_TOP && used_middle <= window_height {
        texts.push(Text {
            x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
            center_y: used_middle,
            content: used_text,
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Left,
        });
    }
    (panel, dividers, texts)
}

/// Tamaño legible: GB con un decimal, si no MB o KB enteros.
pub(crate) fn display_size(bytes: u64) -> String {
    use dokka_model::bytes_in_gb;
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;
    if bytes >= GB {
        bytes_in_gb(bytes)
    } else if bytes >= MB {
        format!("{} MB", bytes / MB)
    } else {
        format!("{} KB", bytes / KB)
    }
}

/// Líneas fuera de la vista, a tamaño cero para no reconstruir capas.
pub(crate) fn hide_dividers(dividers: Vec<Divider>, window_height: f32) -> Vec<Divider> {
    dividers
        .into_iter()
        .map(|mut divider| {
            let middle = divider.bounds.y + divider.bounds.height / 2.0;
            if middle < measures::CONTENT_TOP || middle > window_height {
                divider.bounds = Rect {
                    x: 0.0,
                    y: -1000.0,
                    width: 0.0,
                    height: 0.0,
                };
            }
            divider
        })
        .collect()
}

/// Tarjeta de tamaño cero con el estilo de las tarjetas.
pub(crate) fn hidden_card(theme: &Theme) -> Panel {
    Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::CARD_RADIUS,
        border_width: measures::CARD_BORDER_WIDTH,
        tint: theme.settings.card,
        border: theme.settings.card_border,
    }
}
