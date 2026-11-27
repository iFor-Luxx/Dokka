use dokka_model::AudioOutput;
use dokka_scene::{Divider, Overlay, Panel, Rect, Size};
use dokka_theme::{Theme, volume as measures};

use super::layout::{
    mute_row, output_row, separator_rect, settings_row, slider_area, slider_touch, title_row,
    visible_count,
};
use super::rows::{Rows, row_icons, row_texts};
use super::{VolumePanelHit, VolumePanelIcons, VolumePanelProps};
use crate::slider::{SliderColors, SliderProps, slider_panels, slider_track_bounds};

/// Tamaño del panel para estas salidas (sin ninguna, el compacto de antes).
pub fn volume_panel_size(output_count: usize) -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::panel_height(output_count),
    }
}

/// Un solo cristal oscuro Tahoe: título con porcentaje, barra fina con
/// tirador entre dos altavoces, sección de salidas si hay, línea,
/// silenciar, línea y ajustes.
pub fn volume_panel(
    theme: &Theme,
    icons: &VolumePanelIcons,
    bounds: Rect,
    props: &VolumePanelProps,
    outputs: &[AudioOutput],
) -> Overlay {
    let title = title_row(bounds);
    let touch = slider_touch(bounds);
    let area = slider_area(touch);
    let track = slider_track_bounds(area);
    let slider = SliderProps {
        value: props.level,
        neutral: None,
        active: !props.is_muted,
    };
    let flats: [Panel; 4] = slider_panels(track, &slider, &SliderColors::glass(theme));
    let mute = mute_row(bounds, outputs);
    let settings = settings_row(bounds, outputs);
    let rows = Rows {
        bounds,
        title,
        touch,
        mute,
        settings,
    };
    Overlay {
        panel: glass(theme, bounds),
        tiles: Vec::new(),
        pill: None,
        highlight: Some(highlight_panel(
            theme,
            bounds,
            outputs,
            mute,
            settings,
            props.hovered,
        )),
        flat_panels: flats.into_iter().collect(),
        dividers: vec![
            separator(theme, bounds, outputs, 0),
            separator(theme, bounds, outputs, 1),
        ],
        dots: Vec::new(),
        pointer: None,
        texts: row_texts(theme, rows, outputs, props),
        icons: row_icons(theme, icons, rows, outputs, props),
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Línea fina entre secciones, con el color del cristal.
fn separator(theme: &Theme, bounds: Rect, outputs: &[AudioOutput], index: usize) -> Divider {
    Divider {
        bounds: separator_rect(bounds, outputs, index),
        color: theme.tray.separator,
    }
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

/// Resaltado suave de una salida, silenciar o ajustes. Existe siempre, de
/// tamaño cero si no hay nada, para no reconstruir capas.
fn highlight_panel(
    theme: &Theme,
    bounds: Rect,
    outputs: &[AudioOutput],
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
        Some(VolumePanelHit::Output(index)) => {
            if index < visible_count(outputs) {
                output_row(bounds, index)
            } else {
                return hidden;
            }
        }
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
