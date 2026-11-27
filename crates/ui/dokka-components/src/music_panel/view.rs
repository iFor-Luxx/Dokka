use std::sync::Arc;

use dokka_scene::{Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, music_panel as measures};

use super::{MusicPanelHit, MusicPanelIcons, MusicTrack};

/// Tamaño fijo de la tarjeta, con o sin canción para no reconstruir capas.
pub fn music_panel_size(_track: Option<&MusicTrack>) -> Size {
    Size {
        width: measures::WIDTH,
        height: measures::HEIGHT,
    }
}

/// Un solo cristal oscuro con carátula, título y controles grandes.
/// Sin canción muestra "Nada en reproducción" con los botones atenuados.
/// Capas siempre iguales: cristal, un plano de carátula, dos textos
/// y tres iconos.
pub fn music_panel_panel(
    theme: &Theme,
    icons: &MusicPanelIcons,
    bounds: Rect,
    track: Option<&MusicTrack>,
) -> Overlay {
    Overlay {
        panel: Panel {
            bounds,
            corner_radius: capped(bounds, measures::TILE_RADIUS),
            border_width: measures::TILE_BORDER_WIDTH,
            tint: theme.tray.panel,
            border: theme.tray.panel_border,
        },
        tiles: Vec::new(),
        pill: None,
        highlight: None,
        flat_panels: vec![artwork_panel(theme, bounds)],
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

/// Botón bajo el punto, si hay canción (atenuados no responden).
pub fn music_panel_hit(
    bounds: Rect,
    x: f32,
    y: f32,
    track: Option<&MusicTrack>,
) -> Option<MusicPanelHit> {
    track?;
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

fn artwork_rect(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + measures::PADDING,
        y: bounds.y + measures::PADDING,
        width: measures::ARTWORK_SIDE,
        height: measures::ARTWORK_SIDE,
    }
}

fn artwork_panel(theme: &Theme, bounds: Rect) -> Panel {
    let art = artwork_rect(bounds);
    Panel {
        bounds: art,
        corner_radius: capped(art, measures::ARTWORK_RADIUS),
        border_width: 0.0,
        tint: theme.tray.circle,
        border: theme.tray.circle,
    }
}

fn text_left(bounds: Rect) -> f32 {
    let art = artwork_rect(bounds);
    art.x + art.width + measures::TEXT_GAP
}

fn controls_top(bounds: Rect) -> f32 {
    bounds.y + bounds.height - measures::PADDING - measures::CONTROLS_HEIGHT
}

fn music_buttons(bounds: Rect) -> [Rect; 3] {
    let total = measures::BUTTON_WIDTH * 3.0;
    let start = bounds.x + (bounds.width - total) / 2.0;
    [0.0, 1.0, 2.0].map(|index| Rect {
        x: start + measures::BUTTON_WIDTH * index,
        y: controls_top(bounds),
        width: measures::BUTTON_WIDTH,
        height: measures::CONTROLS_HEIGHT,
    })
}

fn music_texts(theme: &Theme, bounds: Rect, track: Option<&MusicTrack>) -> Vec<Text> {
    let left = text_left(bounds);
    let art = artwork_rect(bounds);
    let (title, artist, artist_color) = match track {
        Some(track) => (
            shorten(&track.title, measures::TITLE_CHARACTERS),
            shorten(&track.artist, measures::ARTIST_CHARACTERS),
            theme.tray.secondary,
        ),
        None => (
            measures::EMPTY_TEXT.to_owned(),
            String::new(),
            theme.tray.secondary,
        ),
    };
    vec![
        Text {
            x: left,
            center_y: art.y + art.height / 2.0 - 11.0,
            content: title,
            size: measures::TITLE_SIZE,
            is_bold: true,
            color: theme.tray.text,
            align: TextAlign::Left,
        },
        Text {
            x: left,
            center_y: art.y + art.height / 2.0 + 11.0,
            content: artist,
            size: measures::SUBTITLE_SIZE,
            is_bold: false,
            color: artist_color,
            align: TextAlign::Left,
        },
    ]
}

fn music_icons(
    theme: &Theme,
    icons: &MusicPanelIcons,
    bounds: Rect,
    track: Option<&MusicTrack>,
) -> Vec<Icon> {
    let playing = track.is_some_and(|track| track.is_playing);
    let dimmed = track.is_none();
    let drawings: [&Arc<dokka_scene::Drawing>; 3] = if dimmed {
        match playing {
            true => [&icons.previous_dim, &icons.pause_dim, &icons.next_dim],
            false => [&icons.previous_dim, &icons.play_dim, &icons.next_dim],
        }
    } else if playing {
        [&icons.previous, &icons.pause, &icons.next]
    } else {
        [&icons.previous, &icons.play, &icons.next]
    };
    music_buttons(bounds)
        .iter()
        .zip(drawings)
        .map(|(area, drawing)| {
            let side = measures::BUTTON_ICON_SIDE;
            Icon {
                x: area.x + (area.width - side) / 2.0,
                y: area.y + (area.height - side) / 2.0,
                side,
                image: Some(IconImage::Drawing(Arc::clone(drawing))),
                placeholder: theme.tray.clear,
                placeholder_radius: 0.0,
            }
        })
        .collect()
}

fn contains(bounds: &Rect, x: f32, y: f32) -> bool {
    x >= bounds.x && x < bounds.x + bounds.width && y >= bounds.y && y < bounds.y + bounds.height
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

fn shorten(value: &str, limit: usize) -> String {
    if value.chars().count() <= limit {
        return value.to_owned();
    }
    let kept: String = value.chars().take(limit - 1).collect();
    format!("{}…", kept.trim_end())
}
