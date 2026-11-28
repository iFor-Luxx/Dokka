//! Ficha "Acerca de este PC": héroe con el modelo y tarjetas con datos reales.
//!
//! Reutiliza `settings_group` para tarjetas; los botones y sus piezas viven
//! en `about_buttons`, las tarjetas en `about_cards` y el dibujo en
//! `about_pc`. Lo que queda fuera de la vista no se pinta: se lleva a tamaño
//! cero para no reconstruir capas al bajar con la rueda.

use std::sync::Arc;

use dokka_model::{SystemSummary, bytes_in_gb};
use dokka_scene::{Divider, Drawing, Icon, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{
    ABOUT_HERO_HEIGHT, ABOUT_MODEL_SIZE, ABOUT_PC_SIDE, about_main_rows, about_top,
};
use super::super::state::Settings;
use super::about_buttons::{button_hover, button_pieces};
use super::about_cards::{section_header, value_card};
use super::about_pc::hidden_icon;
use super::content::ContentScene;

/// Ficha completa; `pc` es el dibujo propio del equipo.
pub(crate) fn about_content(
    theme: &Theme,
    settings: &Settings,
    pc: &Arc<Drawing>,
) -> ContentScene {
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let window_height = settings.window_height();
    let summary = settings.system_summary().unwrap_or_default();
    let top = about_top(scroll);
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut icons = Vec::new();
    let mut texts = Vec::new();

    // Héroe: PC, modelo y línea gris.
    let (hero_dividers, hero_icons, hero_texts) =
        hero_pieces(theme, top, width, window_height, &summary, pc);
    dividers.extend(hero_dividers);
    icons.extend(hero_icons);
    texts.extend(hero_texts);

    // Tarjeta principal con los cinco valores.
    let main_top = top + ABOUT_HERO_HEIGHT + measures::GROUP_GAP;
    let main_labels = about_main_rows();
    let main_owned = main_values(&summary);
    let main_refs: Vec<&str> = main_owned.iter().map(|value| value.as_str()).collect();
    let (panel, mut card_dividers, mut card_texts) = value_card(
        theme,
        main_top,
        &main_labels,
        &main_refs,
        width,
        window_height,
    );
    flats.push(panel);
    dividers.append(&mut card_dividers);
    texts.append(&mut card_texts);

    // Secciones con su cabecera y su tarjeta.
    let mut cursor = main_top + 5.0 * measures::CONTENT_ROW_HEIGHT + measures::GROUP_GAP;
    for section in about_sections(&summary) {
        let header = section_header(theme, cursor, section.title, width, window_height);
        texts.extend(header);
        cursor += super::super::logic::ABOUT_SECTION_TITLE;
        let labels: Vec<&'static str> = section.labels.iter().copied().collect();
        let values: Vec<String> = section.values.clone();
        let refs: Vec<&str> = values.iter().map(|value| value.as_str()).collect();
        let (panel, mut card_dividers, mut card_texts) =
            value_card(theme, cursor, &labels, &refs, width, window_height);
        flats.push(panel);
        dividers.append(&mut card_dividers);
        texts.append(&mut card_texts);
        cursor += labels.len() as f32 * measures::CONTENT_ROW_HEIGHT;
        if let Some(button) = section.button {
            cursor += super::super::logic::ABOUT_BUTTON_GAP;
            let (panel, text) = button_pieces(theme, cursor, button, width, window_height);
            flats.push(panel);
            texts.push(text);
            cursor += super::super::logic::ABOUT_BUTTON_HEIGHT;
        }
        cursor += measures::GROUP_GAP;
    }

    // Informe del sistema, centrado abajo.
    let (panel, text) = button_pieces(
        theme,
        cursor,
        "Informe del sistema…",
        width,
        window_height,
    );
    flats.push(panel);
    texts.push(text);

    // Resaltado del botón bajo el cursor: siempre existe.
    flats.push(button_hover(theme, settings, top, width));
    icons.push(hidden_icon());
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}

struct AboutSection {
    title: &'static str,
    labels: Vec<&'static str>,
    values: Vec<String>,
    button: Option<&'static str>,
}

fn about_sections(summary: &SystemSummary) -> Vec<AboutSection> {
    // Muestra "123,4 GB libres de 476,9 GB (C:)".
    let storage_value = match (
        &summary.storage_drive,
        summary.storage_free_bytes,
        summary.storage_total_bytes,
    ) {
        (Some(drive), Some(free), Some(total)) => {
            format!(
                "{} libres de {} ({drive})",
                bytes_in_gb(free),
                bytes_in_gb(total)
            )
        }
        _ => "No disponible".to_owned(),
    };
    vec![
        AboutSection {
            title: "Windows",
            labels: vec!["Edición", "Versión"],
            values: vec![
                SystemSummary::display(&summary.windows_edition).to_owned(),
                SystemSummary::display(&summary.windows_version).to_owned(),
            ],
            button: None,
        },
        AboutSection {
            title: "Pantalla",
            labels: vec!["Nombre", "Resolución"],
            values: vec![
                SystemSummary::display(&summary.display_name).to_owned(),
                SystemSummary::display(&summary.display_size).to_owned(),
            ],
            button: Some("Ajustes de pantalla…"),
        },
        AboutSection {
            title: "Almacenamiento",
            labels: vec!["Disco del sistema"],
            values: vec![storage_value],
            button: Some("Ajustes de almacenamiento…"),
        },
    ]
}

fn main_values(summary: &SystemSummary) -> Vec<String> {
    let memory = summary
        .memory_bytes
        .map(bytes_in_gb)
        .unwrap_or_else(|| "No disponible".to_owned());
    vec![
        SystemSummary::display(&summary.computer_name).to_owned(),
        SystemSummary::display(&summary.processor).to_owned(),
        SystemSummary::display(&summary.graphics).to_owned(),
        memory,
        SystemSummary::display(&summary.serial).to_owned(),
    ]
}

/// Héroe: dibujo del PC, modelo en grande y línea gris.
fn hero_pieces(
    theme: &Theme,
    top: f32,
    width: f32,
    window_height: f32,
    summary: &SystemSummary,
    pc: &Arc<Drawing>,
) -> (Vec<Divider>, Vec<Icon>, Vec<Text>) {
    let center = measures::CONTENT_LEFT + width / 2.0;
    let pc_y = top + 12.0;
    let model_y = pc_y + ABOUT_PC_SIDE + 12.0 + ABOUT_MODEL_SIZE / 2.0;
    let line_y = model_y + ABOUT_MODEL_SIZE / 2.0 + 12.0;
    let visible = |y: f32| y >= measures::CONTENT_TOP && y <= window_height;
    let mut icons = Vec::new();
    if visible(pc_y + ABOUT_PC_SIDE / 2.0) {
        icons.push(Icon {
            x: center - ABOUT_PC_SIDE / 2.0,
            y: pc_y,
            side: ABOUT_PC_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(pc))),
            placeholder: theme.settings.card_text,
            placeholder_radius: 0.0,
        });
    } else {
        icons.push(hidden_icon());
    }
    let mut texts = Vec::new();
    if visible(model_y) {
        texts.push(Text {
            x: center,
            center_y: model_y,
            content: SystemSummary::display(&summary.model).to_owned(),
            size: ABOUT_MODEL_SIZE,
            is_bold: true,
            color: theme.settings.row_text,
            align: TextAlign::Center,
        });
    }
    let mut dividers = Vec::new();
    if visible(line_y) {
        dividers.push(Divider {
            bounds: dokka_scene::Rect {
                x: center - 60.0,
                y: line_y,
                width: 120.0,
                height: 1.0,
            },
            color: theme.settings.divider,
        });
    }
    (dividers, icons, texts)
}
