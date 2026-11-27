use std::sync::Arc;

use dokka_scene::{Divider, Dot, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, volume as measures};

use crate::slider::{SliderColors, SliderProps, slider_hit, slider_panels, slider_track_bounds};

use super::{VolumePanelHit, VolumePanelIcons, VolumePanelProps};

/// Tamaño fijo del panel: un solo cristal con título, cápsula y dos filas.
pub fn volume_panel_size() -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::panel_height(),
    }
}

/// Un solo cristal oscuro Tahoe: título, cápsula ancha con el número,
/// línea, silenciar, línea y ajustes. `tiles` vacío y cantidad constante:
/// dos planos, dos líneas, un punto, cuatro textos y dos iconos.
pub fn volume_panel(
    theme: &Theme,
    icons: &VolumePanelIcons,
    bounds: Rect,
    props: &VolumePanelProps,
) -> Overlay {
    let title = title_row(bounds);
    let touch = slider_touch(bounds);
    let track = slider_track_bounds(touch);
    let mute = mute_row(bounds);
    let settings = settings_row(bounds);
    let colors = SliderColors {
        track: theme.tray.track,
        fill: theme.tray.fill,
        muted: theme.tray.secondary,
    };
    let slider = SliderProps {
        value: props.level,
        active: !props.is_muted,
    };
    let flats = slider_panels(track, &slider, &colors);
    Overlay {
        panel: glass(theme, bounds),
        tiles: Vec::new(),
        pill: None,
        highlight: Some(highlight_panel(theme, mute, settings, props.hovered)),
        flat_panels: vec![flats[0], flats[1]],
        dividers: vec![separator(theme, bounds, 0), separator(theme, bounds, 1)],
        dots: vec![knob(theme, track, props)],
        pointer: None,
        texts: vec![
            title_text(theme, title),
            percent_text(theme, track, props),
            mute_text(theme, mute),
            settings_text(theme, settings),
        ],
        icons: vec![
            speaker_icon(theme, icons, mute, props),
            settings_icon(theme, icons, settings),
        ],
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Zona tocada: silenciar, ajustes o valor sobre la cápsula.
pub fn volume_panel_hit(bounds: Rect, x: f32, y: f32) -> Option<VolumePanelHit> {
    if contains(mute_row(bounds), x, y) {
        return Some(VolumePanelHit::Mute);
    }
    if contains(settings_row(bounds), x, y) {
        return Some(VolumePanelHit::Settings);
    }
    slider_hit(slider_touch(bounds), x, y).map(VolumePanelHit::Slider)
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    (rect.x..rect.x + rect.width).contains(&x) && (rect.y..rect.y + rect.height).contains(&y)
}

/// Cristal único oscuro con borde fino y filo claro.
fn glass(theme: &Theme, bounds: Rect) -> Panel {
    Panel {
        bounds,
        corner_radius: capped(bounds, measures::PANEL_RADIUS),
        border_width: measures::PANEL_BORDER_WIDTH,
        tint: theme.tray.panel,
        border: theme.tray.panel_border,
    }
}

/// Radio recortado para no superar la mitad del lado menor.
fn capped(tile: Rect, maximum: f32) -> f32 {
    if tile.width <= 0.0 || tile.height <= 0.0 {
        return 0.0;
    }
    maximum
        .min(tile.width / 2.0)
        .min(tile.height / 2.0)
        .max(0.0)
}

fn title_row(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y + measures::PADDING,
        width: bounds.width,
        height: measures::TITLE_HEIGHT,
    }
}

/// Zona de toque del deslizador, más alta que la cápsula.
fn slider_touch(bounds: Rect) -> Rect {
    let title = title_row(bounds);
    Rect {
        x: bounds.x + measures::PADDING,
        y: title.y + title.height + measures::TITLE_GAP,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::SLIDER_ROW_HEIGHT,
    }
}

fn mute_row(bounds: Rect) -> Rect {
    let touch = slider_touch(bounds);
    let top =
        touch.y + touch.height + measures::SECTION_GAP + dokka_theme::tray_glass::SEPARATOR_HEIGHT;
    Rect {
        x: bounds.x + measures::PADDING,
        y: top,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

fn settings_row(bounds: Rect) -> Rect {
    let mute = mute_row(bounds);
    Rect {
        x: bounds.x + measures::PADDING,
        y: mute.y + mute.height + dokka_theme::tray_glass::SEPARATOR_HEIGHT,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::ROW_HEIGHT,
    }
}

fn separator(theme: &Theme, bounds: Rect, index: usize) -> Divider {
    let mute = mute_row(bounds);
    let y = if index == 0 {
        mute.y - measures::SECTION_GAP
    } else {
        mute.y + mute.height
    };
    Divider {
        bounds: Rect {
            x: bounds.x + measures::PADDING,
            y,
            width: bounds.width - measures::PADDING * 2.0,
            height: dokka_theme::tray_glass::SEPARATOR_HEIGHT,
        },
        color: theme.tray.separator,
    }
}

/// Resaltado suave de silenciar o ajustes. Existe siempre, de tamaño
/// cero si no hay nada, para no reconstruir capas.
fn highlight_panel(
    theme: &Theme,
    mute: Rect,
    settings: Rect,
    hovered: Option<VolumePanelHit>,
) -> Panel {
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: 0.0,
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    };
    let row = match hovered {
        Some(VolumePanelHit::Mute) => mute,
        Some(VolumePanelHit::Settings) => settings,
        _ => return hidden,
    };
    Panel {
        bounds: row,
        corner_radius: capped(row, measures::HIGHLIGHT_RADIUS),
        border_width: 0.0,
        tint: theme.tray.hover,
        border: theme.tray.hover,
    }
}

/// Recorta el nivel a 0-1 sin `clamp`: el silencio (`NaN`) queda en 0.
fn level01(value: f32) -> f32 {
    if value.is_nan() || value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

/// Tirador redondo blanco al final del relleno.
fn knob(theme: &Theme, track: Rect, props: &VolumePanelProps) -> Dot {
    let fill = track.width * level01(props.level);
    let diameter = measures::THUMB_DIAMETER.min(track.height);
    let center = (track.x + fill).clamp(track.x, track.x + track.width);
    Dot {
        x: center - diameter / 2.0,
        y: track.y + (track.height - diameter) / 2.0,
        diameter: diameter.max(0.0),
        color: theme.tray.fill,
    }
}

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

/// Número dentro de la cápsula, a la izquierda: oscuro sobre el relleno
/// blanco y blanco cuando el relleno aún no lo cubre.
fn percent_text(theme: &Theme, track: Rect, props: &VolumePanelProps) -> Text {
    let percent = (level01(props.level) * 100.0).round() as i32;
    let fill = track.width * level01(props.level);
    Text {
        x: track.x + measures::PERCENT_INSET,
        center_y: track.y + track.height / 2.0,
        content: format!("{percent}%"),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: if fill >= measures::PERCENT_FILL_MIN {
            theme.tray.percent
        } else {
            theme.tray.text
        },
        align: TextAlign::Left,
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
