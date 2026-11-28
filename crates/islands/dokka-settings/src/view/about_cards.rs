//! Tarjetas de "Acerca de este PC": cabeceras y valores a la derecha.
//!
//! Reutiliza `settings_group` para paneles, líneas y etiquetas; los valores
//! van a la derecha en gris. Lo que queda fuera de la vista se lleva a
//! tamaño cero para no reconstruir capas al bajar con la rueda.

use dokka_components::settings_group::{
    SettingsGroupProps, group_dividers, group_labels, group_panel,
};
use dokka_scene::{Divider, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::content::group_pieces;

/// Cabecera de sección ("Windows", "Pantalla"...).
pub(crate) fn section_header(
    theme: &Theme,
    top: f32,
    title: &str,
    width: f32,
    window_height: f32,
) -> Vec<Text> {
    let middle = top + measures::ABOUT_SECTION_TITLE_HEIGHT / 2.0;
    if middle < measures::CONTENT_TOP || middle > window_height {
        return Vec::new();
    }
    let _ = width;
    vec![Text {
        x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
        center_y: middle,
        content: title.to_owned(),
        size: measures::ABOUT_SECTION_TITLE_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Left,
    }]
}

/// Tarjeta con etiquetas a la izquierda y valores a la derecha en gris.
pub(crate) fn value_card(
    theme: &Theme,
    top: f32,
    labels: &[&'static str],
    values: &[&str],
    width: f32,
    window_height: f32,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let heights = vec![measures::CONTENT_ROW_HEIGHT; labels.len()];
    let card = group_pieces(theme, top, labels, &heights, &[false; 0], width);
    with_values(theme, card, top, height_of(&heights), values, width, window_height)
}

/// Tarjeta con etiquetas dinámicas a la izquierda y valores a la derecha.
/// Igual que `value_card` pero con nombres que llegan de Windows.
pub(crate) fn value_card_owned(
    theme: &Theme,
    top: f32,
    labels: &[String],
    values: &[&str],
    width: f32,
    window_height: f32,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let heights = vec![measures::CONTENT_ROW_HEIGHT; labels.len()];
    let props = SettingsGroupProps {
        top,
        rows: labels.to_vec(),
        heights: heights.clone(),
        width,
    };
    let panel = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);
    let texts = group_labels(theme, &props, &[false; 0]);
    with_values(
        theme,
        (panel, dividers, texts),
        top,
        height_of(&heights),
        values,
        width,
        window_height,
    )
}

fn height_of(heights: &[f32]) -> f32 {
    heights.iter().sum()
}

/// Esconde lo que queda fuera de la vista y añade los valores a la derecha.
/// Los paneles escondidos quedan a tamaño cero para no reconstruir capas.
fn with_values(
    theme: &Theme,
    card: (Panel, Vec<Divider>, Vec<Text>),
    top: f32,
    height: f32,
    values: &[&str],
    width: f32,
    window_height: f32,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let (mut panel, mut dividers, mut texts) = card;
    if top + height < measures::CONTENT_TOP || top > window_height {
        panel.bounds = Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        };
        dividers.clear();
        texts.clear();
    } else {
        // Valores a la derecha, en gris.
        for (index, value) in values.iter().enumerate() {
            let y = top
                + index as f32 * measures::CONTENT_ROW_HEIGHT
                + measures::CONTENT_ROW_HEIGHT / 2.0;
            if y < measures::CONTENT_TOP || y > window_height {
                continue;
            }
            texts.push(Text {
                x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
                center_y: y,
                content: value.to_string(),
                size: measures::CONTENT_LABEL_SIZE,
                is_bold: false,
                color: theme.settings.card_text,
                align: TextAlign::Right,
            });
        }
        dividers = dividers
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
            .collect();
    }
    (panel, dividers, texts)
}
