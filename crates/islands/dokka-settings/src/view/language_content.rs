//! Página "Idioma y región": idiomas, ejemplo vivo y formatos.
//!
//! Tres tarjetas Tahoe: la lista de idiomas del usuario (el primero es el
//! principal), una línea de ejemplo con la fecha larga, la corta y un número
//! con el formato de Windows, y los formatos de solo lectura; debajo, el
//! segmentado del primer día de la semana, propio de Dokka. El botón abre la
//! página de Windows: Dokka no cambia sus ajustes. Si la lista es larga, la
//! página baja con la rueda y lo que queda fuera no se pinta.

use dokka_components::segmented_control::{
    SegmentedProps, segmented_panels, segmented_texts, segmented_touch,
};
use dokka_scene::{Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{
    language_button_top, language_card_top, language_example_top, language_format_rows,
    language_row,
};
use super::super::painted::PaintedControl;
use super::super::state::Settings;
use super::about_buttons::button_pieces;
use super::about_cards::{value_card, value_card_owned};
use super::content::{ContentScene, group_pieces, visible_texts, zero_panels};
use super::general_rows::{hidden_icon, hide_above};

/// Las tres tarjetas, la línea de ejemplo y el botón a la página de Windows.
pub(crate) fn language_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let window_height = settings.window_height();
    let summary = settings.system_summary().unwrap_or_default();
    let count = settings.language_count();
    let monday = settings.preferences().general.week_starts_monday;
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut texts = Vec::new();

    // Idiomas preferidos: el primero lleva "(principal)".
    let (names, marks) = language_names(&summary);
    let card = value_card_owned(
        theme,
        language_card_top(0, count, scroll),
        &names,
        &marks,
        width,
        window_height,
    );
    flats.push(card.0);
    dividers.extend(card.1);
    texts.extend(card.2);

    // Línea de ejemplo con el formato de Windows.
    let example_top = language_example_top(count, scroll);
    let middle = example_top + measures::ABOUT_SECTION_TITLE_HEIGHT / 2.0;
    if middle >= measures::CONTENT_TOP && middle <= window_height {
        texts.push(Text {
            x: measures::CONTENT_LEFT + width / 2.0,
            center_y: middle,
            content: example_line(&summary),
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.row_text,
            align: TextAlign::Center,
        });
    }

    // Formatos de solo lectura.
    let card = value_card(
        theme,
        language_card_top(1, count, scroll),
        &language_format_rows(),
        &language_values(&summary),
        width,
        window_height,
    );
    flats.push(card.0);
    dividers.extend(card.1);
    texts.extend(card.2);

    // Primer día de la semana: segmentado propio de Dokka.
    let top = language_card_top(2, count, scroll);
    let heights = [measures::CONTENT_ROW_HEIGHT; 1];
    let (panel, card_dividers, card_texts) = group_pieces(
        theme,
        top,
        &["Primer día de la semana"],
        &heights,
        &[false; 1],
        width,
    );
    let touch = segmented_touch(language_row(2, 0, count, width, scroll));
    let props = SegmentedProps {
        options: vec!["Domingo".to_owned(), "Lunes".to_owned()],
        selected: usize::from(monday),
    };
    if top + measures::CONTENT_ROW_HEIGHT < measures::CONTENT_TOP || top > window_height {
        flats.push(zeroed(panel));
        flats.extend(zero_panels(
            segmented_panels(theme, touch, &props).into_iter().collect(),
        ));
    } else {
        flats.push(panel);
        dividers.extend(hide_above(card_dividers, window_height));
        texts.extend(visible_texts(card_texts, window_height));
        flats.extend(segmented_panels(theme, touch, &props));
        texts.extend(visible_texts(
            segmented_texts(theme, touch, &props),
            window_height,
        ));
    }

    // Botón a la página de Windows y resaltado al pasar el cursor.
    let (panel, text) = button_pieces(
        theme,
        language_button_top(count, scroll),
        "Ajustes de idioma y región…",
        width,
        window_height,
    );
    flats.push(panel);
    texts.push(text);
    flats.push(button_hover(theme, settings, count, scroll, width));

    texts = visible_texts(texts, window_height);
    let icons = vec![hidden_icon(theme)];
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}

/// Nombres de idiomas y la marca del principal; sin datos, una fila que avisa.
fn language_names(summary: &dokka_model::SystemSummary) -> (Vec<String>, Vec<&'static str>) {
    let Some(languages) = summary.locale.languages.as_ref() else {
        return (vec!["Idiomas".to_owned()], vec!["No disponible"]);
    };
    if languages.is_empty() {
        return (vec!["Idiomas".to_owned()], vec!["No disponible"]);
    }
    let mut marks = vec!["(principal)"];
    marks.extend(vec![""; languages.len().saturating_sub(1)]);
    (languages.clone(), marks)
}

/// Valores de la tarjeta de formatos, o "No disponible".
fn language_values(summary: &dokka_model::SystemSummary) -> [&str; 6] {
    let locale = &summary.locale;
    [
        locale.region_name.as_deref().unwrap_or("No disponible"),
        locale.calendar_name.as_deref().unwrap_or("No disponible"),
        locale
            .temperature_unit
            .as_deref()
            .unwrap_or("No disponible"),
        locale
            .measurement_system
            .as_deref()
            .unwrap_or("No disponible"),
        locale.date_pattern.as_deref().unwrap_or("No disponible"),
        locale.number_format.as_deref().unwrap_or("No disponible"),
    ]
}

/// Fecha larga, corta y número con el formato de Windows.
fn example_line(summary: &dokka_model::SystemSummary) -> String {
    let long = summary
        .locale
        .long_date_example
        .as_deref()
        .unwrap_or("No disponible");
    let short = summary
        .locale
        .short_date_example
        .as_deref()
        .unwrap_or("No disponible");
    let number = summary
        .locale
        .number_example
        .as_deref()
        .unwrap_or("No disponible");
    format!("{long} · {short} · {number}")
}

/// Resaltado del botón bajo el cursor, siempre presente.
fn button_hover(
    theme: &Theme,
    settings: &Settings,
    count: usize,
    scroll: f32,
    width: f32,
) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::ABOUT_BUTTON_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    };
    let Some(PaintedControl::GeneralButton(1)) = settings.values().control else {
        return hidden;
    };
    let x = measures::CONTENT_LEFT + (width - measures::ABOUT_BUTTON_WIDTH) / 2.0;
    Panel {
        bounds: Rect {
            x,
            y: language_button_top(count, scroll),
            width: measures::ABOUT_BUTTON_WIDTH,
            height: measures::ABOUT_BUTTON_HEIGHT,
        },
        corner_radius: measures::ABOUT_BUTTON_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    }
}

/// Panel a tamaño cero con su estilo, para no reconstruir capas al esconder.
fn zeroed(mut panel: Panel) -> Panel {
    panel.bounds = Rect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };
    panel
}
