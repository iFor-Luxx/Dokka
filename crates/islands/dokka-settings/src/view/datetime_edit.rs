//! Tarjetas editables de "Fecha y hora": interruptores, casilla y lista.
//!
//! Tres tarjetas Tahoe con lo protegido atenuado y su nota. La isla solo
//! emite acciones; el shell conecta y pide releer tras cada cambio.

use std::sync::Arc;

use dokka_scene::Drawing;
use dokka_theme::{Theme, settings as measures};

use super::content::{ContentScene, visible_texts};
use super::datetime_cards::{admin_note, group_card, push_card, switch_row};
use super::datetime_rows::{
    apply_cancel_texts, live_text, manual_fields, server_texts, zone_texts,
};
use super::general_rows::hidden_icon;
use crate::datetime_edit::visible_zones;
use crate::logic::{edit_button_top, edit_card_tops, edit_row};
use crate::state::Settings;

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
        vec!["Fecha y hora automáticas".to_owned(), "Servidor".to_owned()]
    };
    let heights0 = vec![measures::CONTENT_ROW_HEIGHT; rows0];
    let card0 = group_card(
        theme,
        tops[0],
        &labels0,
        &heights0,
        &[dim0, dim0, dim0],
        width,
    );
    push_card(
        &mut flats,
        &mut dividers,
        &mut texts,
        card0,
        tops[0],
        &heights0,
        window_height,
    );
    flats.extend(switch_row(
        theme,
        tops[0],
        &heights0,
        0,
        auto_time.unwrap_or(false),
        width,
        window_height,
    ));
    server_texts(theme, settings, &mut texts, window_height, width, scroll);
    if edit.server_editing {
        apply_cancel_texts(
            theme,
            &mut texts,
            edit_row(edit, 0, 2, width, scroll),
            window_height,
        );
    }
    admin_note(
        theme,
        &mut texts,
        tops[0],
        &heights0,
        edit.can_change,
        width,
        window_height,
    );

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
    let card1 = group_card(
        theme,
        tops[1],
        &labels1,
        &heights1,
        &[dim1, dim1, dim1, false],
        width,
    );
    push_card(
        &mut flats,
        &mut dividers,
        &mut texts,
        card1,
        tops[1],
        &heights1,
        window_height,
    );
    live_text(
        theme,
        settings,
        &mut texts,
        hour_24,
        width,
        scroll,
        window_height,
    );
    if edit.manual_editing {
        manual_fields(
            theme,
            settings,
            &mut texts,
            &mut icons,
            (minus, plus),
            width,
            scroll,
        );
        apply_cancel_texts(
            theme,
            &mut texts,
            edit_row(edit, 1, 2, width, scroll),
            window_height,
        );
        flats.extend(switch_row(
            theme,
            tops[1],
            &heights1,
            3,
            hour_24,
            width,
            window_height,
        ));
    } else {
        flats.extend(switch_row(
            theme,
            tops[1],
            &heights1,
            1,
            hour_24,
            width,
            window_height,
        ));
    }
    admin_note(
        theme,
        &mut texts,
        tops[1],
        &heights1,
        edit.can_change,
        width,
        window_height,
    );

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
    push_card(
        &mut flats,
        &mut dividers,
        &mut texts,
        card2,
        tops[2],
        &heights2,
        window_height,
    );
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
    admin_note(
        theme,
        &mut texts,
        tops[2],
        &heights2,
        edit.can_change,
        width,
        window_height,
    );

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
