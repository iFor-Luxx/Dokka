use std::sync::Arc;

use dokka_scene::{Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, volume as measures};

use crate::slider::{SliderColors, SliderProps, slider_hit, slider_panels, slider_track_bounds};

use super::{VolumePanelHit, VolumePanelIcons, VolumePanelProps};

/// Tamaño fijo del panel: tarjeta de sonido, hueco y tarjeta de ajustes.
pub fn volume_panel_size() -> Size {
    Size {
        width: measures::PANEL_WIDTH,
        height: measures::panel_height(),
    }
}

/// Dos piezas de cristal Tahoe sobre fondo invisible: tarjeta "Sonido" con
/// deslizador fino (altavoz que silencia a la izquierda, ondas a la derecha)
/// y tarjeta con la fila de ajustes. Sin tirador ni separador: el hueco
/// entre piezas separa. Sale con escala 1; la isla le pone la animación.
pub fn volume_panel(
    theme: &Theme,
    icons: &VolumePanelIcons,
    bounds: Rect,
    props: &VolumePanelProps,
) -> Overlay {
    let sound = sound_tile(bounds);
    let settings = settings_tile(bounds);
    let row = slider_row(sound);
    let mute_area = mute_rect(row);
    let waves_area = waves_rect(row);
    let area = bar_area(row);
    let track = slider_track_bounds(area);
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
        panel: clear_panel(theme, bounds),
        tiles: vec![tile_panel(theme, sound), tile_panel(theme, settings)],
        pill: None,
        highlight: Some(highlight_panel(theme, sound, settings, mute_area, props.hovered)),
        flat_panels: vec![flats[0], flats[1]],
        dividers: Vec::new(),
        dots: Vec::new(),
        pointer: None,
        texts: vec![title_text(theme, sound), settings_text(theme, settings)],
        icons: vec![
            speaker_icon(theme, icons, mute_area, props),
            waves_icon(theme, icons, waves_area),
            settings_icon(theme, icons, settings),
        ],
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Zona tocada: altavoz que silencia, fila de ajustes o valor sobre la barra.
pub fn volume_panel_hit(bounds: Rect, x: f32, y: f32) -> Option<VolumePanelHit> {
    let sound = sound_tile(bounds);
    let row = slider_row(sound);
    if contains(mute_rect(row), x, y) {
        return Some(VolumePanelHit::Mute);
    }
    if contains(settings_tile(bounds), x, y) {
        return Some(VolumePanelHit::Settings);
    }
    slider_hit(bar_area(row), x, y).map(VolumePanelHit::Slider)
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    (rect.x..rect.x + rect.width).contains(&x) && (rect.y..rect.y + rect.height).contains(&y)
}

/// Fondo invisible que agrupa las piezas: mismos bounds de siempre.
fn clear_panel(theme: &Theme, bounds: Rect) -> Panel {
    Panel {
        bounds,
        corner_radius: 0.0,
        border_width: 0.0,
        tint: theme.tray.clear,
        border: theme.tray.clear,
    }
}

/// Pieza de cristal ahumado con borde fino y luminoso.
fn tile_panel(theme: &Theme, tile: Rect) -> Panel {
    Panel {
        bounds: tile,
        corner_radius: capped(tile, measures::TILE_RADIUS),
        border_width: measures::TILE_BORDER_WIDTH,
        tint: theme.tray.tile,
        border: theme.tray.tile_border,
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

/// Tarjeta superior con el título y el deslizador.
fn sound_tile(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y,
        width: bounds.width,
        height: measures::sound_tile_height(),
    }
}

/// Tarjeta inferior con la fila de ajustes.
fn settings_tile(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x,
        y: bounds.y + measures::sound_tile_height() + measures::GAP,
        width: bounds.width,
        height: measures::SETTINGS_TILE_HEIGHT,
    }
}

/// Fila con el altavoz, la barra y las ondas, dentro de la tarjeta.
fn slider_row(sound: Rect) -> Rect {
    Rect {
        x: sound.x + measures::PADDING,
        y: sound.y + measures::PADDING + measures::TITLE_HEIGHT + measures::TITLE_GAP,
        width: sound.width - measures::PADDING * 2.0,
        height: measures::SLIDER_ROW_HEIGHT,
    }
}

/// Hueco pulsable del altavoz de la izquierda (silencia).
fn mute_rect(row: Rect) -> Rect {
    Rect {
        x: row.x,
        y: row.y + (row.height - measures::MUTE_HIT_SIDE) / 2.0,
        width: measures::MUTE_HIT_SIDE,
        height: measures::MUTE_HIT_SIDE,
    }
}

/// Hueco del altavoz con ondas de la derecha (adorno).
fn waves_rect(row: Rect) -> Rect {
    Rect {
        x: row.x + row.width - measures::MUTE_HIT_SIDE,
        y: row.y + (row.height - measures::MUTE_HIT_SIDE) / 2.0,
        width: measures::MUTE_HIT_SIDE,
        height: measures::MUTE_HIT_SIDE,
    }
}

/// Zona de toque de la barra, entre el altavoz y las ondas.
fn bar_area(row: Rect) -> Rect {
    let mute = mute_rect(row);
    let waves = waves_rect(row);
    Rect {
        x: mute.x + mute.width + measures::ICON_GAP,
        y: row.y,
        width: waves.x - measures::ICON_GAP - (mute.x + mute.width),
        height: row.height,
    }
}

/// Resaltado del altavoz (círculo) o de la tarjeta de ajustes.
/// Existe siempre, de tamaño cero si no hay nada, para no reconstruir capas.
fn highlight_panel(
    theme: &Theme,
    _sound: Rect,
    settings: Rect,
    mute_area: Rect,
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
    match hovered {
        Some(VolumePanelHit::Mute) => Panel {
            bounds: mute_area,
            corner_radius: capped(mute_area, mute_area.width / 2.0),
            border_width: 0.0,
            tint: theme.tray.hover,
            border: theme.tray.hover,
        },
        Some(VolumePanelHit::Settings) => Panel {
            bounds: settings,
            corner_radius: capped(settings, measures::TILE_RADIUS),
            border_width: 0.0,
            tint: theme.tray.hover,
            border: theme.tray.hover,
        },
        _ => {
            hidden
        }
    }
}

fn title_text(theme: &Theme, sound: Rect) -> Text {
    Text {
        x: sound.x + measures::PADDING,
        center_y: sound.y + measures::PADDING + measures::TITLE_HEIGHT / 2.0,
        content: measures::TITLE.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

fn settings_text(theme: &Theme, settings: Rect) -> Text {
    Text {
        x: settings.x + measures::PADDING,
        center_y: settings.y + settings.height / 2.0,
        content: measures::SETTINGS_LABEL.to_owned(),
        size: measures::TITLE_SIZE,
        is_bold: false,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

fn centered_icon(
    theme: &Theme,
    drawing: &Arc<dokka_scene::Drawing>,
    area: Rect,
    side: f32,
) -> Icon {
    Icon {
        x: area.x + (area.width - side) / 2.0,
        y: area.y + (area.height - side) / 2.0,
        side,
        image: Some(IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.tray.text,
        placeholder_radius: 0.0,
    }
}

fn speaker_icon(
    theme: &Theme,
    icons: &VolumePanelIcons,
    area: Rect,
    props: &VolumePanelProps,
) -> Icon {
    centered_icon(
        theme,
        icons.speaker(props.level, props.is_muted),
        area,
        measures::ICON_SIDE,
    )
}

fn waves_icon(theme: &Theme, icons: &VolumePanelIcons, area: Rect) -> Icon {
    centered_icon(theme, icons.waves(), area, measures::ICON_SIDE)
}

fn settings_icon(theme: &Theme, icons: &VolumePanelIcons, settings: Rect) -> Icon {
    Icon {
        x: settings.x + settings.width - measures::PADDING - measures::SETTINGS_CHEVRON_SIDE,
        y: settings.y + (settings.height - measures::SETTINGS_CHEVRON_SIDE) / 2.0,
        side: measures::SETTINGS_CHEVRON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(icons.chevron()))),
        placeholder: theme.tray.text,
        placeholder_radius: 0.0,
    }
}
