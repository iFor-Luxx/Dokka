//! Filas de "Fecha y hora": servidor, fecha viva, campos y zonas.
//!
//! Textos e iconos de cada tarjeta editable con su atenuado.

use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use crate::datetime_edit::DateTimeEdit;
use crate::datetime_edit::visible_zones as zone_capacity;
use crate::logic::{edit_row, stepper_rects};
use crate::state::Settings;

use super::datetime_edit::format_live_shared;

/// Valor del servidor o su borrador con "Cambiar…".
pub(crate) fn server_texts(
    theme: &Theme,
    settings: &Settings,
    texts: &mut Vec<Text>,
    window_height: f32,
    width: f32,
    scroll: f32,
) {
    let edit = settings.datetime_edit();
    let summary = settings.system_summary().unwrap_or_default();
    let row = edit_row(edit, 0, 1, width, scroll);
    let middle = row.y + row.height / 2.0;
    if middle < measures::CONTENT_TOP || middle > window_height {
        return;
    }
    if edit.server_editing {
        texts.push(Text {
            x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
            center_y: middle,
            content: edit.server_draft.clone() + "▏",
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.row_text,
            align: TextAlign::Left,
        });
        return;
    }
    let current = summary
        .datetime
        .ntp_server
        .clone()
        .unwrap_or_else(|| "No disponible".to_owned());
    let suffix = if edit.can_change { " · Cambiar…" } else { "" };
    texts.push(Text {
        x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
        center_y: middle,
        content: current + suffix,
        size: measures::CONTENT_LABEL_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Right,
    });
}

/// "Aplicar" a la izquierda y "Cancelar" a la derecha de la fila.
pub(crate) fn apply_cancel_texts(
    theme: &Theme,
    texts: &mut Vec<Text>,
    row: Rect,
    window_height: f32,
) {
    let middle = row.y + row.height / 2.0;
    if middle < measures::CONTENT_TOP || middle > window_height {
        return;
    }
    texts.push(Text {
        x: row.x + measures::CONTENT_PADDING,
        center_y: middle,
        content: "Aplicar".to_owned(),
        size: measures::CONTENT_LABEL_SIZE,
        is_bold: false,
        color: theme.settings.row_text,
        align: TextAlign::Left,
    });
    texts.push(Text {
        x: row.x + row.width - measures::CONTENT_PADDING,
        center_y: middle,
        content: "Cancelar".to_owned(),
        size: measures::CONTENT_LABEL_SIZE,
        is_bold: false,
        color: theme.settings.row_text,
        align: TextAlign::Right,
    });
}

/// Fecha en vivo con "Cambiar…" cuando responde.
pub(crate) fn live_text(
    theme: &Theme,
    settings: &Settings,
    texts: &mut Vec<Text>,
    hour_24: bool,
    width: f32,
    scroll: f32,
    window_height: f32,
) {
    let edit = settings.datetime_edit();
    let row = edit_row(edit, 1, 0, width, scroll);
    let middle = row.y + row.height / 2.0;
    if middle < measures::CONTENT_TOP || middle > window_height {
        return;
    }
    let mut content = format_live_shared(settings.page_now(), hour_24);
    let summary = settings.system_summary().unwrap_or_default();
    if edit.can_change && summary.datetime.auto_time != Some(true) && !edit.manual_editing {
        content += " · Cambiar…";
    }
    texts.push(Text {
        x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
        center_y: middle,
        content,
        size: measures::CONTENT_LABEL_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Right,
    });
}

/// Campos de día a minuto con sus valores y flechas propias.
pub(crate) fn manual_fields(
    theme: &Theme,
    settings: &Settings,
    texts: &mut Vec<Text>,
    icons: &mut Vec<Icon>,
    drawings: (&Arc<Drawing>, &Arc<Drawing>),
    width: f32,
    scroll: f32,
) {
    let (minus, plus) = drawings;
    let edit = settings.datetime_edit();
    let row = edit_row(edit, 1, 1, width, scroll);
    let values = [edit.day, edit.month, edit.year, edit.hour, edit.minute];
    let names = ["D", "M", "A", "h", "m"];
    for field in 0..5 {
        let (minus_rect, plus_rect) = stepper_rects(&row, field);
        let slot = measures::DATETIME_STEP_SLOT;
        let base = row.x + row.width - measures::CONTENT_PADDING - 5.0 * slot;
        let center = base + f32::from(field) * slot + slot / 2.0;
        let middle = row.y + row.height / 2.0;
        if middle >= measures::CONTENT_TOP && middle <= settings.window_height() {
            texts.push(Text {
                x: center - measures::DATETIME_FIELD_OFFSET,
                center_y: middle,
                content: names[field as usize].to_owned(),
                size: measures::SLIDER_CAPTION_SIZE,
                is_bold: false,
                color: theme.settings.card_text,
                align: TextAlign::Left,
            });
            texts.push(Text {
                x: center + measures::DATETIME_FIELD_OFFSET,
                center_y: middle,
                content: format!("{:02}", values[field as usize]),
                size: measures::CONTENT_LABEL_SIZE,
                is_bold: false,
                color: theme.settings.row_text,
                align: TextAlign::Right,
            });
        }
        icons.push(Icon {
            x: minus_rect.x,
            y: minus_rect.y,
            side: minus_rect.width,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(minus))),
            placeholder: theme.settings.card_text,
            placeholder_radius: 0.0,
        });
        icons.push(Icon {
            x: plus_rect.x,
            y: plus_rect.y,
            side: plus_rect.width,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(plus))),
            placeholder: theme.settings.card_text,
            placeholder_radius: 0.0,
        });
    }
}

/// Zona actual y su lista con la actual marcada.
pub(crate) fn zone_texts(
    theme: &Theme,
    settings: &Settings,
    texts: &mut Vec<Text>,
    width: f32,
    scroll: f32,
    window_height: f32,
) {
    let edit: &DateTimeEdit = settings.datetime_edit();
    let summary = settings.system_summary().unwrap_or_default();
    let current = summary.datetime.timezone_name.clone();
    let row = edit_row(edit, 2, 1, width, scroll);
    let middle = row.y + row.height / 2.0;
    if middle >= measures::CONTENT_TOP && middle <= window_height {
        let shown = current
            .as_deref()
            .and_then(|key| {
                edit.zones
                    .iter()
                    .find(|zone| zone.key == key)
                    .map(|zone| zone.display.clone())
            })
            .unwrap_or_else(|| current.clone().unwrap_or_else(|| "No disponible".to_owned()));
        let suffix = if edit.zone_open { " ▴" } else { " ▾" };
        texts.push(Text {
            x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
            center_y: middle,
            content: shown + suffix,
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Right,
        });
    }
    if !edit.zone_open {
        return;
    }
    for slot in 0..edit.zones.len().min(zone_capacity()) {
        let index = edit.zone_scroll.saturating_add(slot);
        let Some(zone) = edit.zones.get(index) else {
            continue;
        };
        let row = edit_row(edit, 2, 2 + slot, width, scroll);
        let middle = row.y + row.height / 2.0;
        if middle < measures::CONTENT_TOP || middle > window_height {
            continue;
        }
        let marked = current.as_deref() == Some(zone.key.as_str());
        let content = if marked {
            format!("✓ {}", zone.display)
        } else {
            zone.display.clone()
        };
        texts.push(Text {
            x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
            center_y: middle,
            content,
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.row_text,
            align: TextAlign::Left,
        });
    }
}
