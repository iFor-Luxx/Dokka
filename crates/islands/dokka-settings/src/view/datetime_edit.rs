//! Tarjetas editables de "Fecha y hora": interruptores, casilla y lista.
//!
//! Tres tarjetas Tahoe con lo protegido atenuado y su nota. La isla solo
//! emite acciones; el shell conecta y pide releer tras cada cambio.

use std::sync::Arc;

use dokka_scene::{Divider, Drawing, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::content::{ContentScene, switch_pieces, visible_texts, zero_panels};
use super::datetime_rows::{apply_cancel_texts, live_text, manual_fields, server_texts, zone_texts};
use super::general_rows::{hidden_icon, hide_above};
use crate::datetime_edit::visible_zones;
use crate::logic::{edit_button_top, edit_card_tops, edit_row};
use crate::state::Settings;

/// Nota bajo cada tarjeta protegida sin elevar.
const ADMIN_NOTE: &str = "Requiere permisos de administrador";

/// Las tres tarjetas editables, la lista de zonas y el botón final.
pub(crate) fn datetime_edit_content(
    theme: &Theme,
    settings: &Settings,
    minus: &Arc<Drawing>,
    plus: &Arc<Drawing>,
) -> ContentScene {
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let window_height = settings.window_height();
    let summary = settings.system_summary().unwrap_or_default();
    let edit = settings.datetime_edit();
    let tops = edit_card_tops(edit, scroll);
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut texts = Vec::new();
    let mut icons = Vec::new();

    let auto_time = summary.datetime.auto_time;
    let auto_zone = summary.datetime.auto_timezone;
    let dim0 = !edit.can_change;
    let dim1 = !edit.can_change || auto_time == Some(true);
    let dim2switch = !edit.can_change;
    let dim2list = !edit.can_change || auto_zone == Some(true);

    // Tarjeta 0: automáticas y servidor.
    let rows0 = if edit.server_editing { 3 } else { 2 };
    let labels0: Vec<String> = if edit.server_editing {
        vec![
            "Fecha y hora automáticas".to_owned(),
            "Servidor".to_owned(),
            String::new(),
        ]
    } else {
        vec![
            "Fecha y hora automáticas".to_owned(),
            "Servidor".to_owned(),
        ]
    };
    let heights0 = vec![measures::CONTENT_ROW_HEIGHT; rows0];
    let card0 = group_card(theme, tops[0], &labels0, &heights0, &[dim0, dim0, dim0], width);
    push_card(&mut flats, &mut dividers, &mut texts, card0, tops[0], &heights0, window_height);
    flats.extend(switch_row(theme, tops[0], &heights0, 0, auto_time.unwrap_or(false), width, window_height));
    server_texts(theme, settings, &mut texts, window_height, width, scroll);
    if edit.server_editing {
        apply_cancel_texts(theme, &mut texts, edit_row(edit, 0, 2, width, scroll), window_height);
    }
    admin_note(theme, &mut texts, tops[0], &heights0, edit.can_change, width, window_height);

    // Tarjeta 1: fecha viva y 24 horas (+ editor).
    let hour_24 = settings.preferences().general.hour_24;
    let rows1 = if edit.manual_editing { 4 } else { 2 };
    let labels1: Vec<String> = if edit.manual_editing {
        vec![
            "Fecha y hora".to_owned(),
            "Día · Mes · Año · Hora · Minuto".to_owned(),
            String::new(),
            "Formato de 24 horas".to_owned(),
        ]
    } else {
        vec!["Fecha y hora".to_owned(), "Formato de 24 horas".to_owned()]
    };
    let heights1 = vec![measures::CONTENT_ROW_HEIGHT; rows1];
    let card1 = group_card(theme, tops[1], &labels1, &heights1, &[dim1, dim1, dim1, false], width);
    push_card(&mut flats, &mut dividers, &mut texts, card1, tops[1], &heights1, window_height);
    live_text(theme, settings, &mut texts, hour_24, width, scroll, window_height);
    if edit.manual_editing {
        manual_fields(theme, settings, &mut texts, &mut icons, (minus, plus), width, scroll);
        apply_cancel_texts(theme, &mut texts, edit_row(edit, 1, 2, width, scroll), window_height);
        flats.extend(switch_row(theme, tops[1], &heights1, 3, hour_24, width, window_height));
    } else {
        flats.extend(switch_row(theme, tops[1], &heights1, 1, hour_24, width, window_height));
    }
    admin_note(theme, &mut texts, tops[1], &heights1, edit.can_change, width, window_height);

    // Tarjeta 2: zona auto y zona (+ lista).
    let list = edit.zones.len().min(visible_zones());
    let rows2 = 2 + if edit.zone_open { list } else { 0 };
    let mut labels2 = vec![
        "Zona horaria automática".to_owned(),
        "Zona horaria".to_owned(),
    ];
    labels2.extend(vec![String::new(); list]);
    let heights2 = vec![measures::CONTENT_ROW_HEIGHT; rows2];
    let mut dim_list = vec![dim2switch, dim2list];
    dim_list.extend(vec![dim2list; list]);
    let card2 = group_card(theme, tops[2], &labels2, &heights2, &dim_list, width);
    push_card(&mut flats, &mut dividers, &mut texts, card2, tops[2], &heights2, window_height);
    flats.extend(switch_row(
        theme,
        tops[2],
        &heights2,
        0,
        auto_zone.unwrap_or(false),
        width,
        window_height,
    ));
    zone_texts(theme, settings, &mut texts, width, scroll, window_height);
    admin_note(theme, &mut texts, tops[2], &heights2, edit.can_change, width, window_height);

    // Botón final a la página de Windows.
    let (panel, text) = super::about_buttons::button_pieces(
        theme,
        edit_button_top(edit, scroll),
        "Ajustes de fecha y hora…",
        width,
        window_height,
    );
    flats.push(panel);
    texts.push(text);
    texts = visible_texts(texts, window_height);
    icons.push(hidden_icon(theme));
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}

/// Tarjeta con etiquetas propias y atenuadas.
fn group_card(
    theme: &Theme,
    top: f32,
    labels: &[String],
    heights: &[f32],
    dim: &[bool],
    width: f32,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    use dokka_components::settings_group::{SettingsGroupProps, group_dividers, group_labels, group_panel};
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
fn push_card(
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
fn switch_row(
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
fn admin_note(
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
