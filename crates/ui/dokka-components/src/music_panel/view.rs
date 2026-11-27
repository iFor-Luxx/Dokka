use std::sync::Arc;

use dokka_scene::{Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, music_panel as measures, typography};

use super::{MusicPanelHit, MusicPanelIcons, MusicTrack};
use crate::popup_panel::popup_panel;

/// Tamaño fijo de la tarjeta, con o sin canción para no reconstruir capas.
pub fn music_panel_size(_track: Option<&MusicTrack>) -> Size {
    Size {
        width: measures::WIDTH,
        height: measures::HEIGHT,
    }
}

/// Tarjeta de cristal con título, artista y barra de botones.
/// Con `None` muestra "Nada en reproducción" y la barra mide cero.
pub fn music_panel_panel(
    theme: &Theme,
    icons: &MusicPanelIcons,
    bounds: Rect,
    track: Option<&MusicTrack>,
) -> Overlay {
    Overlay {
        panel: popup_panel(theme, bounds),
        pill: None,
        highlight: None,
        flat_panels: vec![button_bar_panel(theme, bounds, track)],
        dividers: Vec::new(),
        dots: Vec::new(),
        pointer: None,
        texts: music_texts(theme, bounds, track),
        icons: music_icons(theme, icons, bounds, track),
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Botón bajo el punto, si hay canción.
pub fn music_panel_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    track: Option<&MusicTrack>,
) -> Option<MusicPanelHit> {
    if track.is_none() {
        return None;
    }
    let buttons = music_buttons(bounds);
    if contains(&buttons[0], x, y) {
        Some(MusicPanelHit::Previous)
    } else if contains(&buttons[1], x, y) {
        Some(MusicPanelHit::Toggle)
    } else if contains(&buttons[2], x, y) {
        Some(MusicPanelHit::Next)
    } else {
        None
    }
}

fn button_bar(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: bounds.y + bounds.height - measures::PADDING - measures::BUTTON_BAR_HEIGHT,
        width: bounds.width - measures::PADDING * 2.0,
        height: measures::BUTTON_BAR_HEIGHT,
    }
}

fn button_bar_panel(theme: &Theme, bounds: Rect, track: Option<&MusicTrack>) -> Panel {
    let bar = button_bar(bounds);
    if track.is_some() {
        Panel {
            bounds: bar,
            corner_radius: measures::BUTTON_BAR_RADIUS,
            border_width: 0.0,
            tint: theme.panel.highlight,
            border: theme.panel.highlight,
        }
    } else {
        Panel {
            bounds: Rect {
                x: bar.x,
                y: bar.y,
                width: 0.0,
                height: 0.0,
            },
            corner_radius: measures::BUTTON_BAR_RADIUS,
            border_width: 0.0,
            tint: theme.panel.highlight,
            border: theme.panel.highlight,
        }
    }
}

fn music_buttons(bounds: Rect) -> [Rect; 3] {
    let bar = button_bar(bounds);
    let third = bar.width / 3.0;
    [0.0, 1.0, 2.0].map(|index| Rect {
        x: bar.x + third * index,
        width: third,
        ..bar
    })
}

fn music_texts(theme: &Theme, bounds: Rect, track: Option<&MusicTrack>) -> Vec<Text> {
    let center_x = bounds.x + bounds.width / 2.0;
    let (title, artist) = match track {
        Some(track) => (
            shorten(&track.title, measures::TITLE_CHARACTERS),
            shorten(&track.artist, measures::ARTIST_CHARACTERS),
        ),
        None => (measures::EMPTY_TEXT.to_owned(), String::new()),
    };
    vec![
        Text {
            x: center_x,
            center_y: bounds.y + measures::TITLE_CENTER_OFFSET,
            content: title,
            size: typography::TITLE_SIZE,
            is_bold: true,
            color: theme.panel.text,
            align: TextAlign::Center,
        },
        Text {
            x: center_x,
            center_y: bounds.y + measures::ARTIST_CENTER_OFFSET,
            content: artist,
            size: typography::BODY_SIZE,
            is_bold: false,
            color: theme.panel.text,
            align: TextAlign::Center,
        },
    ]
}

fn music_icons(
    theme: &Theme,
    icons: &MusicPanelIcons,
    bounds: Rect,
    track: Option<&MusicTrack>,
) -> Vec<Icon> {
    let toggle = match track {
        Some(track) if track.is_playing => &icons.pause,
        _ => &icons.play,
    };
    let drawings: [&Arc<dokka_scene::Drawing>; 3] = [&icons.previous, toggle, &icons.next];
    let visible = track.is_some();
    music_buttons(bounds)
        .iter()
        .zip(drawings)
        .map(|(area, drawing)| {
            let side = if visible {
                measures::BUTTON_ICON_SIDE
            } else {
                0.0
            };
            Icon {
                x: area.x + (area.width - side) / 2.0,
                y: area.y + (area.height - side) / 2.0,
                side,
                image: Some(IconImage::Drawing(Arc::clone(drawing))),
                placeholder: theme.panel.background,
                placeholder_radius: 0.0,
            }
        })
        .collect()
}

fn contains(bounds: &Rect, x: f32, y: f32) -> bool {
    x >= bounds.x
        && x < bounds.x + bounds.width
        && y >= bounds.y
        && y < bounds.y + bounds.height
}

fn shorten(value: &str, limit: usize) -> String {
    if value.chars().count() <= limit {
        return value.to_owned();
    }
    let kept: String = value.chars().take(limit - 1).collect();
    format!("{}…", kept.trim_end())
}
