use dokka_scene::{Divider, Overlay, Panel, Rect, Size};
use dokka_theme::{Theme, volume as measures};

use super::knob::slider_knob;
use super::layout::{mute_row, separator_rect, settings_row, slider_bar, slider_touch, title_row};
use super::rows::{row_icons, row_texts};
use super::track::track_panels;
use super::{VolumePanelHit, VolumePanelIcons, VolumePanelProps};

/// Tamaño fijo del panel: un solo cristal con título, deslizador y dos filas.
pub fn volume_panel_size() -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::panel_height(),
    }
}

/// Un solo cristal oscuro Tahoe: título con porcentaje, barra fina con
/// tirador entre dos altavoces, línea, silenciar, línea y ajustes.
/// `tiles` vacío y cantidad constante: dos planos, dos líneas, un punto,
/// cuatro textos y cuatro iconos.
pub fn volume_panel(
    theme: &Theme,
    icons: &VolumePanelIcons,
    bounds: Rect,
    props: &VolumePanelProps,
) -> Overlay {
    let title = title_row(bounds);
    let touch = slider_touch(bounds);
    let bar = slider_bar(touch);
    let mute = mute_row(bounds);
    let settings = settings_row(bounds);
    let flats = track_panels(theme, bar, props);
    Overlay {
        panel: glass(theme, bounds),
        tiles: Vec::new(),
        pill: None,
        highlight: Some(highlight_panel(theme, mute, settings, props.hovered)),
        flat_panels: vec![flats[0], flats[1]],
        dividers: vec![separator(theme, bounds, 0), separator(theme, bounds, 1)],
        dots: vec![slider_knob(theme, bar, props)],
        pointer: None,
        texts: row_texts(theme, title, mute, settings, props).into(),
        icons: row_icons(theme, icons, touch, mute, settings, props).into(),
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Línea fina entre secciones, con el color del cristal.
fn separator(theme: &Theme, bounds: Rect, index: usize) -> Divider {
    Divider {
        bounds: separator_rect(bounds, index),
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
