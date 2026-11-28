//! Piezas de las tarjetas editables de "Fecha y hora": montaje, esconder lo
//! fuera de vista, interruptores, nota de administrador y fecha en vivo.

use super::content::{switch_pieces, visible_texts, zero_panels};
use super::general_rows::hide_above;
use dokka_scene::{Divider, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

/// Nota bajo cada tarjeta protegida sin elevar.
const ADMIN_NOTE: &str = "Requiere permisos de administrador";

/// Tarjeta con etiquetas propias y atenuadas.
pub(super) fn group_card(
    theme: &Theme,
    top: f32,
    labels: &[String],
    heights: &[f32],
    dim: &[bool],
    width: f32,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    use dokka_components::settings_group::{
        SettingsGroupProps, group_dividers, group_labels, group_panel,
    };
    let props = SettingsGroupProps {
        top,
        rows: labels.to_vec(),
        heights: heights.to_vec(),
        width,
    };
    (
        group_panel(theme, &props),
        group_dividers(theme, &props),
        group_labels(theme, &props, dim),
    )
}

/// Esconde lo fuera de vista y añade la tarjeta.
pub(super) fn push_card(
    flats: &mut Vec<Panel>,
    dividers: &mut Vec<Divider>,
    texts: &mut Vec<Text>,
    card: (Panel, Vec<Divider>, Vec<Text>),
    top: f32,
    heights: &[f32],
    window_height: f32,
) {
    let height: f32 = heights.iter().sum();
    if top + height < measures::CONTENT_TOP || top > window_height {
        let mut panel = card.0;
        panel.bounds = Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        };
        flats.push(panel);
        flats.extend(zero_panels(Vec::new()));
    } else {
        flats.push(card.0);
        dividers.extend(hide_above(card.1, window_height));
        texts.extend(visible_texts(card.2, window_height));
    }
}

/// Interruptor de una fila dinámica, escondido fuera de vista.
pub(super) fn switch_row(
    theme: &Theme,
    top: f32,
    heights: &[f32],
    index: usize,
    on: bool,
    width: f32,
    window_height: f32,
) -> Vec<Panel> {
    if top + measures::CONTENT_ROW_HEIGHT < measures::CONTENT_TOP || top > window_height {
        return zero_panels(switch_pieces(theme, top, heights, index, on, width));
    }
    switch_pieces(theme, top, heights, index, on, width)
}

/// Línea bajo la tarjeta cuando lo protegido no responde.
pub(super) fn admin_note(
    theme: &Theme,
    texts: &mut Vec<Text>,
    top: f32,
    heights: &[f32],
    can_change: bool,
    width: f32,
    window_height: f32,
) {
    if can_change {
        return;
    }
    let height: f32 = heights.iter().sum();
    let middle = top + height + measures::ABOUT_SECTION_TITLE_HEIGHT / 2.0;
    if middle < measures::CONTENT_TOP || middle > window_height {
        return;
    }
    let _ = width;
    texts.push(Text {
        x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
        center_y: middle,
        content: ADMIN_NOTE.to_owned(),
        size: measures::REORDER_HINT_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Left,
    });
}

/// Fecha en vivo compartida con la tarjeta fina.
pub(crate) fn format_live_shared(now: Option<crate::general::PageNow>, hour_24: bool) -> String {
    const WEEKDAYS: [&str; 7] = [
        "Domingo",
        "Lunes",
        "Martes",
        "Miércoles",
        "Jueves",
        "Viernes",
        "Sábado",
    ];
    const MONTHS: [&str; 12] = [
        "enero",
        "febrero",
        "marzo",
        "abril",
        "mayo",
        "junio",
        "julio",
        "agosto",
        "septiembre",
        "octubre",
        "noviembre",
        "diciembre",
    ];
    let Some(now) = now else {
        return "—".to_owned();
    };
    let weekday = WEEKDAYS[now.weekday.min(6) as usize];
    let month = MONTHS[now.month.saturating_sub(1).min(11) as usize];
    let time = if hour_24 {
        format!(
            "{:02}:{:02}:{:02}",
            now.hour % 24,
            now.minute % 60,
            now.second % 60
        )
    } else {
        let hour = now.hour % 24;
        let (hour, period) = match hour {
            0 => (12, "a. m."),
            1..=11 => (hour, "a. m."),
            12 => (12, "p. m."),
            _ => (hour - 12, "p. m."),
        };
        format!(
            "{hour}:{:02}:{:02} {period}",
            now.minute % 60,
            now.second % 60
        )
    };
    format!("{weekday}, {} de {month} de {}, {time}", now.day, now.year)
}
