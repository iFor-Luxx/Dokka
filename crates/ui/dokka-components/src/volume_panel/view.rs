use std::sync::Arc;

use dokka_scene::{Divider, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, typography, volume as measures};

use crate::popup_panel::popup_panel;
use crate::slider::{SliderProps, slider_hit, slider_knob, slider_panels, slider_track_bounds};

use super::{VolumePanelHit, VolumePanelIcons, VolumePanelProps};

/// Tamaño fijo del panel.
pub fn volume_panel_size() -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::panel_height(),
    }
}

/// Panel con título, deslizador, separador y fila de ajustes.
/// Sale con escala 1; la isla le pone la animación.
pub fn volume_panel(
    theme: &Theme,
    icons: &VolumePanelIcons,
    bounds: Rect,
    props: &VolumePanelProps,
) -> Overlay {
    let row = slider_row(bounds);
    let speaker = speaker_rect(row);
    let mute_area = mute_rect(row);
    let area = bar_area(row);
    let track = slider_track_bounds(area);
    let slider = SliderProps {
        value: props.level,
        active: !props.is_muted,
    };
    let flats = slider_panels(theme, track, &slider);
    Overlay {
        panel: popup_panel(theme, bounds),
        pill: None,
        highlight: Some(highlight_panel(theme, bounds, props.hovered)),
        flat_panels: vec![flats[0], flats[1]],
        dividers: vec![separator(theme, bounds)],
        dots: vec![slider_knob(theme, track, props.level)],
        pointer: None,
        texts: vec![title_text(theme, bounds), settings_text(theme, bounds)],
        icons: vec![
            speaker_icon(theme, icons, speaker, props),
            mute_icon(theme, icons, mute_area),
            settings_icon(theme, icons, bounds),
        ],
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Zona tocada: botón, fila de ajustes o valor sobre la barra.
pub fn volume_panel_hit(bounds: Rect, x: f32, y: f32) -> Option<VolumePanelHit> {
    let row = slider_row(bounds);
    if contains(mute_rect(row), x, y) {
        return Some(VolumePanelHit::Mute);
    }
    if contains(settings_rect(bounds), x, y) {
        return Some(VolumePanelHit::Settings);
    }
    slider_hit(bar_area(row), x, y).map(VolumePanelHit::Slider)
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    (rect.x..rect.x + rect.width).contains(&x) && (rect.y..rect.y + rect.height).contains(&y)
}

/// Fila con el altavoz, la barra y el botón de silencio.
fn slider_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: bounds.y + measures::PADDING + measures::TITLE_HEIGHT + measures::TITLE_GAP,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::SLIDER_HEIGHT,
    }
}

fn speaker_rect(row: Rect) -> Rect {
    Rect {
        x: row.x,
        y: row.y + (row.height - measures::SPEAKER_SIDE) / 2.0,
        width: measures::SPEAKER_SIDE,
        height: measures::SPEAKER_SIDE,
    }
}

fn mute_rect(row: Rect) -> Rect {
    Rect {
        x: row.x + row.width - measures::MUTE_SIDE,
        y: row.y + (row.height - measures::MUTE_SIDE) / 2.0,
        width: measures::MUTE_SIDE,
        height: measures::MUTE_SIDE,
    }
}

/// Zona de toque de la barra, entre el altavoz y el botón.
fn bar_area(row: Rect) -> Rect {
    let speaker = speaker_rect(row);
    let mute = mute_rect(row);
    Rect {
        x: speaker.x + speaker.width + measures::ICON_GAP,
        y: row.y,
        width: mute.x - measures::ICON_GAP - (speaker.x + speaker.width + measures::ICON_GAP),
        height: row.height,
    }
}

fn separator_top(bounds: Rect) -> f32 {
    slider_row(bounds).y + measures::SLIDER_HEIGHT + measures::SLIDER_GAP
}

fn settings_rect(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: separator_top(bounds) + measures::SEPARATOR_HEIGHT,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::SETTINGS_HEIGHT,
    }
}

/// Resaltado suave del botón o de la fila bajo el cursor. Existe siempre,
/// de tamaño cero cuando no hay nada que resaltar, para no reconstruir capas.
fn highlight_panel(theme: &Theme, bounds: Rect, hovered: Option<VolumePanelHit>) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    };
    let area = match hovered {
        Some(VolumePanelHit::Mute) => mute_rect(slider_row(bounds)),
        Some(VolumePanelHit::Settings) => settings_rect(bounds),
        _ => return hidden,
    };
    Panel {
        bounds: area,
        corner_radius: measures::HIGHLIGHT_RADIUS,
        border_width: 0.0,
        tint: theme.settings.hover,
        border: theme.settings.hover,
    }
}

fn separator(theme: &Theme, bounds: Rect) -> Divider {
    Divider {
        bounds: Rect {
            x: bounds.x + measures::PADDING,
            y: separator_top(bounds)
                + (measures::SEPARATOR_HEIGHT - measures::DIVIDER_HEIGHT) / 2.0,
            width: bounds.width - measures::PADDING * 2.0,
            height: measures::DIVIDER_HEIGHT,
        },
        color: theme.settings.divider,
    }
}

fn title_text(theme: &Theme, bounds: Rect) -> Text {
    Text {
        x: bounds.x + measures::PADDING,
        center_y: bounds.y + measures::PADDING + measures::TITLE_HEIGHT / 2.0,
        content: measures::TITLE.to_owned(),
        size: typography::TITLE_SIZE,
        is_bold: true,
        color: theme.panel.text,
        align: TextAlign::Left,
    }
}

fn settings_text(theme: &Theme, bounds: Rect) -> Text {
    let area = settings_rect(bounds);
    Text {
        x: area.x,
        center_y: area.y + area.height / 2.0,
        content: measures::SETTINGS_LABEL.to_owned(),
        size: typography::BODY_SIZE,
        is_bold: false,
        color: theme.panel.text,
        align: TextAlign::Left,
    }
}

fn speaker_icon(
    theme: &Theme,
    icons: &VolumePanelIcons,
    area: Rect,
    props: &VolumePanelProps,
) -> Icon {
    Icon {
        x: area.x,
        y: area.y,
        side: measures::SPEAKER_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(
            icons.speaker(props.level, props.is_muted),
        ))),
        placeholder: theme.panel.text,
        placeholder_radius: 0.0,
    }
}

fn mute_icon(theme: &Theme, icons: &VolumePanelIcons, area: Rect) -> Icon {
    let inset = (measures::MUTE_SIDE - measures::MUTE_ICON_SIDE) / 2.0;
    Icon {
        x: area.x + inset,
        y: area.y + inset,
        side: measures::MUTE_ICON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(icons.mute()))),
        placeholder: theme.panel.text,
        placeholder_radius: 0.0,
    }
}

fn settings_icon(theme: &Theme, icons: &VolumePanelIcons, bounds: Rect) -> Icon {
    let area = settings_rect(bounds);
    Icon {
        x: area.x + area.width - measures::SETTINGS_CHEVRON_SIDE,
        y: area.y + (area.height - measures::SETTINGS_CHEVRON_SIDE) / 2.0,
        side: measures::SETTINGS_CHEVRON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
        placeholder: theme.panel.text,
        placeholder_radius: 0.0,
    }
}
