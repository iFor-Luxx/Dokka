//! Página "Fecha y hora": automáticas, en vivo y zona horaria.
//!
//! Tres tarjetas Tahoe: la primera dice si Windows pone la hora sola y con
//! qué servidor; la segunda trae la fecha en vivo (el shell la pasa en cada
//! `Tick`) y el interruptor de 24 horas, propio de Dokka; la tercera dice si
//! la zona es automática y su nombre. El botón abre la página de Windows:
//! Dokka no cambia sus ajustes. Lo que queda fuera de la vista no se pinta.

use dokka_scene::{Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{datetime_button_top, datetime_card_top, datetime_labels, datetime_row};
use super::super::painted::PaintedControl;
use super::super::state::Settings;
use super::about_buttons::button_pieces;
use super::about_cards::value_card;
use super::content::{ContentScene, group_pieces, switch_pieces, visible_texts, zero_panels};
use super::general_rows::{hidden_icon, hide_above};

/// Días y meses para la fecha en vivo, en español como el resto de Dokka.
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

/// Las tres tarjetas, la hora en vivo y el botón a la página de Windows.
pub(crate) fn datetime_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let window_height = settings.window_height();
    let summary = settings.system_summary().unwrap_or_default();
    let hour_24 = settings.preferences().general.hour_24;
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut texts = Vec::new();

    // Automáticas: si Windows pone la hora sola y con qué servidor.
    let automatic = on_off_text(summary.datetime.auto_time);
    let server = summary
        .datetime
        .ntp_server
        .as_deref()
        .unwrap_or("No disponible");
    let card = value_card(
        theme,
        datetime_card_top(0, scroll),
        &datetime_labels(0),
        &[automatic, server],
        width,
        window_height,
    );
    flats.push(card.0);
    dividers.extend(card.1);
    texts.extend(card.2);

    // En vivo: fecha y hora que pasa el shell más el interruptor propio.
    let top = datetime_card_top(1, scroll);
    let heights = [measures::CONTENT_ROW_HEIGHT; 2];
    let (panel, card_dividers, card_texts) = group_pieces(
        theme,
        top,
        &datetime_labels(1),
        &heights,
        &[false; 2],
        width,
    );
    let live = format_live(settings.page_now(), hour_24);
    if top + 2.0 * measures::CONTENT_ROW_HEIGHT < measures::CONTENT_TOP || top > window_height {
        flats.push(zeroed(panel));
        flats.extend(zero_panels(switch_pieces(
            theme, top, &heights, 1, hour_24, width,
        )));
    } else {
        flats.push(panel);
        dividers.extend(hide_above(card_dividers, window_height));
        texts.extend(visible_texts(card_texts, window_height));
        let row = datetime_row(1, 0, width, scroll);
        let middle = row.y + row.height / 2.0;
        if middle >= measures::CONTENT_TOP && middle <= window_height {
            texts.push(Text {
                x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
                center_y: middle,
                content: live,
                size: measures::CONTENT_LABEL_SIZE,
                is_bold: false,
                color: theme.settings.card_text,
                align: TextAlign::Right,
            });
        }
        flats.extend(switch_pieces(theme, top, &heights, 1, hour_24, width));
    }

    // Zona: si es automática y su nombre.
    let auto_zone = on_off_text(summary.datetime.auto_timezone);
    let zone = summary
        .datetime
        .timezone_name
        .as_deref()
        .unwrap_or("No disponible");
    let card = value_card(
        theme,
        datetime_card_top(2, scroll),
        &datetime_labels(2),
        &[auto_zone, zone],
        width,
        window_height,
    );
    flats.push(card.0);
    dividers.extend(card.1);
    texts.extend(card.2);

    // Botón a la página de Windows y resaltado al pasar el cursor.
    let (panel, text) = button_pieces(
        theme,
        datetime_button_top(scroll),
        "Ajustes de fecha y hora…",
        width,
        window_height,
    );
    flats.push(panel);
    texts.push(text);
    flats.push(button_hover(theme, settings, scroll, width));

    texts = visible_texts(texts, window_height);
    let icons = vec![hidden_icon(theme)];
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}

/// "Activado" o "Desactivado"; "No disponible" si no se pudo leer.
fn on_off_text(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "Activado",
        Some(false) => "Desactivado",
        None => "No disponible",
    }
}

/// Fecha en vivo para la tarjeta, con segundos para que se vea pasar.
fn format_live(now: Option<super::super::general::PageNow>, hour_24: bool) -> String {
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

/// Resaltado del botón bajo el cursor, siempre presente.
fn button_hover(theme: &Theme, settings: &Settings, scroll: f32, width: f32) -> Panel {
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
    let Some(PaintedControl::GeneralButton(0)) = settings.values().control else {
        return hidden;
    };
    let x = measures::CONTENT_LEFT + (width - measures::ABOUT_BUTTON_WIDTH) / 2.0;
    Panel {
        bounds: Rect {
            x,
            y: datetime_button_top(scroll),
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
