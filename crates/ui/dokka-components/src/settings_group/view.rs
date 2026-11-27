use dokka_scene::{Divider, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::SettingsGroupProps;
use super::layout::{divider_rect_for, group_height, group_panel_bounds_for, row_bounds_for};

/// Radio recortado para no superar la mitad del lado menor.
fn capped(maximum: f32, width: f32, height: f32) -> f32 {
    if width <= 0.0 || height <= 0.0 {
        return 0.0;
    }
    maximum.min(width / 2.0).min(height / 2.0).max(0.0)
}

/// Tarjeta clara con el mismo radio y borde de siempre: cambiar de sección
/// solo cambia la cantidad de planos, no su estilo.
pub fn group_panel(theme: &Theme, props: &SettingsGroupProps) -> Panel {
    let bounds = group_panel_bounds_for(props.top, group_height(&props.heights), props.width);
    Panel {
        bounds,
        corner_radius: capped(measures::CARD_RADIUS, bounds.width, bounds.height),
        border_width: measures::CARD_BORDER_WIDTH,
        tint: theme.settings.card,
        border: theme.settings.card_border,
    }
}

/// Líneas finas entre filas, con el divisor del tema.
pub fn group_dividers(theme: &Theme, props: &SettingsGroupProps) -> Vec<Divider> {
    let last = props.rows.len().saturating_sub(1);
    (0..last)
        .map(|index| Divider {
            bounds: divider_rect_for(props.top, &props.heights, index, props.width),
            color: theme.settings.divider,
        })
        .collect()
}

/// Etiquetas a la izquierda de cada fila. Las atenuadas (`dim`) van en el
/// gris de la tarjeta, como el texto "Próximamente".
pub fn group_labels(theme: &Theme, props: &SettingsGroupProps, dim: &[bool]) -> Vec<Text> {
    props
        .rows
        .iter()
        .enumerate()
        .map(|(index, label)| {
            let row: Rect = row_bounds_for(props.top, &props.heights, index, props.width);
            let dimmed = dim.get(index).copied().unwrap_or(false);
            Text {
                x: row.x + measures::CONTENT_PADDING,
                center_y: row.y + row.height / 2.0,
                content: label.clone(),
                size: measures::CONTENT_LABEL_SIZE,
                is_bold: false,
                color: if dimmed {
                    theme.settings.card_text
                } else {
                    theme.settings.row_text
                },
                align: TextAlign::Left,
            }
        })
        .collect()
}
