use std::sync::Arc;

use dokka_icons::{media_next_icon, media_pause_icon, media_play_icon, media_previous_icon};
use dokka_scene::{Controls, Drawing, Icon, IconImage, Panel, Rect};
use dokka_theme::{Theme, popup as measures};

use super::NowPlaying;

/// Iconos ya construidos con los colores de un tema. Se reutilizan entre fotogramas:
/// el pintor solo repinta cuando recibe otro dibujo.
#[derive(Debug)]
pub struct MediaIcons {
    previous: Arc<Drawing>,
    play: Arc<Drawing>,
    pause: Arc<Drawing>,
    next: Arc<Drawing>,
}

impl MediaIcons {
    pub fn new(theme: &Theme) -> Self {
        let color = theme.media.icon;
        Self {
            previous: Arc::new(media_previous_icon(color)),
            play: Arc::new(media_play_icon(color)),
            pause: Arc::new(media_pause_icon(color)),
            next: Arc::new(media_next_icon(color)),
        }
    }
}

/// Zonas de anterior, reproducir/pausar y siguiente, sobre la imagen.
pub fn media_buttons(bounds: Rect) -> [Rect; 3] {
    let pill = pill(bounds);
    [0.0, 1.0, 2.0].map(|index| Rect {
        x: pill.x + measures::MEDIA_BUTTON_WIDTH * index,
        width: measures::MEDIA_BUTTON_WIDTH,
        ..pill
    })
}

pub(super) fn media_controls(
    theme: &Theme,
    icons: &MediaIcons,
    bounds: Rect,
    playing: &NowPlaying,
) -> Controls {
    let toggle = if playing.is_playing {
        &icons.pause
    } else {
        &icons.play
    };
    let buttons = media_buttons(bounds)
        .iter()
        .zip([&icons.previous, toggle, &icons.next])
        .map(|(area, drawing)| Icon {
            x: area.x + (area.width - measures::MEDIA_ICON_SIDE) / 2.0,
            y: area.y + (area.height - measures::MEDIA_ICON_SIDE) / 2.0,
            side: measures::MEDIA_ICON_SIDE,
            image: Some(IconImage::Drawing(Arc::clone(drawing))),
            placeholder: theme.media.background,
            placeholder_radius: 0.0,
        })
        .collect();
    Controls {
        panel: Panel {
            bounds: pill(bounds),
            corner_radius: measures::MEDIA_HEIGHT / 2.0,
            border_width: measures::BORDER_WIDTH,
            tint: theme.media.background,
            border: theme.media.border,
        },
        icons: buttons,
    }
}

fn pill(card: Rect) -> Rect {
    let width = measures::MEDIA_BUTTON_WIDTH * 3.0;
    let image_bottom = card.y + measures::PREVIEW_PADDING + measures::PREVIEW_IMAGE_HEIGHT;
    Rect {
        x: card.x + (card.width - width) / 2.0,
        y: image_bottom - measures::MEDIA_HEIGHT - measures::MEDIA_INSET,
        width,
        height: measures::MEDIA_HEIGHT,
    }
}
