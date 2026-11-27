//! Contenido de las filas: textos e iconos del panel de volumen.
//!
//! Fila del título con el porcentaje a la derecha en texto secundario,
//! altavoces a los extremos de la barra, sección de salidas con marca,
//! filas compactas de silenciar y ajustes. Los textos e iconos crecen con
//! las salidas; el resto es constante.

use std::sync::Arc;

use dokka_model::AudioOutput;
use dokka_scene::{Drawing, Icon, IconImage, Rect, Text, TextAlign};
use dokka_theme::{Theme, tray_glass as glass, volume as measures};

use super::layout::{
    level01, output_row, output_title_row, slider_left_icon, slider_right_icon, visible_count,
};
use super::{VolumePanelIcons, VolumePanelProps};

/// Cajas fijas del panel, compartidas por los textos y los iconos.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Rows {
    pub bounds: Rect,
    pub title: Rect,
    pub touch: Rect,
    pub mute: Rect,
    pub settings: Rect,
}

/// Título, porcentaje, título de salidas, salidas, silenciar y ajustes.
pub(crate) fn row_texts(
    theme: &Theme,
    rows: Rows,
    outputs: &[AudioOutput],
    props: &VolumePanelProps,
) -> Vec<Text> {
    let Rows {
        bounds,
        title,
        mute,
        settings,
        ..
    } = rows;
    let visible = visible_count(outputs);
    let mut texts = Vec::with_capacity(4 + visible * 2);
    texts.push(title_text(theme, title));
    texts.push(percent_text(theme, title, props));
    if visible > 0 {
        texts.push(output_title_text(theme, output_title_row(bounds)));
    }
    for index in 0..visible {
        let Some(output) = outputs.get(index) else {
            continue;
        };
        texts.push(output_text(theme, output_row(bounds, index), output));
    }
    texts.push(mute_text(theme, mute));
    texts.push(settings_text(theme, settings));
    texts
}

/// Laterales del deslizador, marcas de salida, altavoz y chevrón.
pub(crate) fn row_icons(
    theme: &Theme,
    icons: &VolumePanelIcons,
    rows: Rows,
    outputs: &[AudioOutput],
    props: &VolumePanelProps,
) -> Vec<Icon> {
    let Rows {
        bounds,
        touch,
        mute,
        settings,
        ..
    } = rows;
    let visible = visible_count(outputs);
    let mut panel_icons = Vec::with_capacity(4 + visible);
    panel_icons.push(slider_end_icon(
        theme,
        icons.slider_left(props.is_muted),
        touch,
        true,
    ));
    panel_icons.push(slider_end_icon(
        theme,
        icons.slider_right(props.is_muted),
        touch,
        false,
    ));
    for index in 0..visible {
        let Some(output) = outputs.get(index) else {
            continue;
        };
        panel_icons.push(output_icon(theme, icons, output_row(bounds, index), output));
    }
    panel_icons.push(speaker_icon(theme, icons, mute, props));
    panel_icons.push(settings_icon(theme, icons, settings));
    panel_icons
}

/// "Sonido" a la izquierda de la fila del título.
fn title_text(theme: &Theme, title: Rect) -> Text {
    Text {
        x: title.x + glass::ROW_MARGIN + glass::ROW_INSET,
        center_y: title.y + title.height / 2.0,
        content: measures::TITLE.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

/// Porcentaje centrado en su reserva a la derecha del título, en texto
/// secundario. Al no moverse con los dígitos, no salta al arrastrar.
fn percent_text(theme: &Theme, title: Rect, props: &VolumePanelProps) -> Text {
    let percent = (level01(props.level) * 100.0).round() as i32;
    Text {
        x: title.x + title.width
            - (glass::ROW_MARGIN + glass::ROW_INSET)
            - measures::PERCENT_BOX_WIDTH / 2.0,
        center_y: title.y + title.height / 2.0,
        content: format!("{percent}%"),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: theme.tray.secondary,
        align: TextAlign::Center,
    }
}

/// "Salida" pequeño en texto secundario sobre las filas de salida.
fn output_title_text(theme: &Theme, title: Rect) -> Text {
    Text {
        x: title.x + glass::ROW_MARGIN + glass::ROW_INSET,
        center_y: title.y + title.height / 2.0,
        content: measures::OUTPUT_TITLE.to_owned(),
        size: measures::SUBTITLE_SIZE,
        is_bold: false,
        color: theme.tray.secondary,
        align: TextAlign::Left,
    }
}

fn output_text(theme: &Theme, row: Rect, output: &AudioOutput) -> Text {
    Text {
        x: row.x + glass::ROW_INSET + measures::CHECK_SIDE + measures::ICON_GAP,
        center_y: row.y + row.height / 2.0,
        content: short(&output.name),
        size: measures::TITLE_SIZE,
        is_bold: output.is_default,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

fn mute_text(theme: &Theme, row: Rect) -> Text {
    Text {
        x: row.x + glass::ROW_INSET + measures::ICON_SIDE + measures::ICON_GAP,
        center_y: row.y + row.height / 2.0,
        content: measures::MUTE_LABEL.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

fn settings_text(theme: &Theme, settings: Rect) -> Text {
    Text {
        x: settings.x + glass::ROW_INSET,
        center_y: settings.y + settings.height / 2.0,
        content: measures::SETTINGS_LABEL.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

/// Altavoz de un extremo de la barra, a la izquierda o a la derecha.
fn slider_end_icon(theme: &Theme, image: &Arc<Drawing>, touch: Rect, left: bool) -> Icon {
    let frame = if left {
        slider_left_icon(touch)
    } else {
        slider_right_icon(touch)
    };
    Icon {
        x: frame.x,
        y: frame.y,
        side: frame.width,
        image: Some(IconImage::Drawing(Arc::clone(image))),
        placeholder: theme.tray.text,
        placeholder_radius: 0.0,
    }
}

/// Marca de verificación a la izquierda solo en la salida activa.
/// Existe siempre (lado cero si no está activa) para no reconstruir capas.
fn output_icon(theme: &Theme, icons: &VolumePanelIcons, row: Rect, output: &AudioOutput) -> Icon {
    if output.is_default {
        Icon {
            x: row.x + glass::ROW_INSET,
            y: row.y + (row.height - measures::CHECK_SIDE) / 2.0,
            side: measures::CHECK_SIDE,
            image: Some(IconImage::Drawing(Arc::clone(icons.output_check()))),
            placeholder: theme.tray.panel,
            placeholder_radius: 0.0,
        }
    } else {
        Icon {
            x: row.x + glass::ROW_INSET,
            y: row.y + row.height / 2.0,
            side: 0.0,
            image: None,
            placeholder: theme.tray.panel,
            placeholder_radius: 0.0,
        }
    }
}

fn speaker_icon(
    theme: &Theme,
    icons: &VolumePanelIcons,
    row: Rect,
    props: &VolumePanelProps,
) -> Icon {
    Icon {
        x: row.x + glass::ROW_INSET,
        y: row.y + (row.height - measures::ICON_SIDE) / 2.0,
        side: measures::ICON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(
            icons.speaker(props.level, props.is_muted),
        ))),
        placeholder: theme.tray.text,
        placeholder_radius: 0.0,
    }
}

fn settings_icon(theme: &Theme, icons: &VolumePanelIcons, settings: Rect) -> Icon {
    Icon {
        x: settings.x + settings.width - glass::ROW_INSET - measures::SETTINGS_CHEVRON_SIDE,
        y: settings.y + (settings.height - measures::SETTINGS_CHEVRON_SIDE) / 2.0,
        side: measures::SETTINGS_CHEVRON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
        placeholder: theme.tray.text,
        placeholder_radius: 0.0,
    }
}

fn short(name: &str) -> String {
    if name.chars().count() <= measures::MAX_NAME_CHARACTERS {
        return name.to_owned();
    }
    let kept: String = name
        .chars()
        .take(measures::MAX_NAME_CHARACTERS - 1)
        .collect();
    format!("{}…", kept.trim_end())
}
