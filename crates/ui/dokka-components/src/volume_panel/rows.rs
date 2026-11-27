//! Contenido de las filas: textos e iconos del panel de volumen.
//!
//! Fila del título con el porcentaje a la derecha en texto secundario,
//! altavoces a los extremos de la barra y filas compactas de silenciar y
//! ajustes. Cantidad constante: cuatro textos y cuatro iconos.

use std::sync::Arc;

use dokka_scene::{Drawing, Icon, IconImage, Rect, Text, TextAlign};
use dokka_theme::{Theme, volume as measures};

use super::layout::{level01, slider_left_icon, slider_right_icon};
use super::{VolumePanelIcons, VolumePanelProps};

/// Título, porcentaje, silenciar y ajustes, en ese orden.
pub(crate) fn row_texts(
    theme: &Theme,
    title: Rect,
    mute: Rect,
    settings: Rect,
    props: &VolumePanelProps,
) -> [Text; 4] {
    [
        title_text(theme, title),
        percent_text(theme, title, props),
        mute_text(theme, mute),
        settings_text(theme, settings),
    ]
}

/// Laterales del deslizador, altavoz de silenciar y chevrón, en ese orden.
pub(crate) fn row_icons(
    theme: &Theme,
    icons: &VolumePanelIcons,
    touch: Rect,
    mute: Rect,
    settings: Rect,
    props: &VolumePanelProps,
) -> [Icon; 4] {
    [
        slider_end_icon(theme, icons.slider_left(props.is_muted), touch, true),
        slider_end_icon(theme, icons.slider_right(props.is_muted), touch, false),
        speaker_icon(theme, icons, mute, props),
        settings_icon(theme, icons, settings),
    ]
}

/// "Sonido" a la izquierda de la fila del título.
fn title_text(theme: &Theme, title: Rect) -> Text {
    Text {
        x: title.x + measures::PADDING,
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
        x: title.x + title.width - measures::PADDING - measures::PERCENT_BOX_WIDTH / 2.0,
        center_y: title.y + title.height / 2.0,
        content: format!("{percent}%"),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: theme.tray.secondary,
        align: TextAlign::Center,
    }
}

fn mute_text(theme: &Theme, row: Rect) -> Text {
    Text {
        x: row.x + measures::ICON_SIDE + measures::ICON_GAP,
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
        x: settings.x,
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

fn speaker_icon(
    theme: &Theme,
    icons: &VolumePanelIcons,
    row: Rect,
    props: &VolumePanelProps,
) -> Icon {
    Icon {
        x: row.x,
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
        x: settings.x + settings.width - measures::SETTINGS_CHEVRON_SIDE,
        y: settings.y + (settings.height - measures::SETTINGS_CHEVRON_SIDE) / 2.0,
        side: measures::SETTINGS_CHEVRON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
        placeholder: theme.tray.text,
        placeholder_radius: 0.0,
    }
}
