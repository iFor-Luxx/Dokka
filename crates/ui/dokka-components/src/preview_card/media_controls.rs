use std::sync::{Arc, LazyLock};

use dokka_icons::{next_icon, pause_icon, play_icon, previous_icon};
use dokka_scene::{Controls, Drawing, Icon, IconImage, Panel, Rect};
use dokka_theme::popup as theme;

use super::NowPlaying;

// Inmutables y creados una sola vez: el pintor solo repinta cuando recibe otro dibujo.
static PREVIOUS: LazyLock<Arc<Drawing>> = LazyLock::new(|| Arc::new(previous_icon()));
static PLAY: LazyLock<Arc<Drawing>> = LazyLock::new(|| Arc::new(play_icon()));
static PAUSE: LazyLock<Arc<Drawing>> = LazyLock::new(|| Arc::new(pause_icon()));
static NEXT: LazyLock<Arc<Drawing>> = LazyLock::new(|| Arc::new(next_icon()));

/// Zonas de anterior, reproducir/pausar y siguiente, sobre la imagen.
pub fn media_buttons(bounds: Rect) -> [Rect; 3] {
    let pill = pill(bounds);
    [0.0, 1.0, 2.0].map(|index| Rect {
        x: pill.x + theme::MEDIA_BUTTON_WIDTH * index,
        width: theme::MEDIA_BUTTON_WIDTH,
        ..pill
    })
}

pub(super) fn media_controls(bounds: Rect, playing: &NowPlaying) -> Controls {
    let toggle = if playing.is_playing { &PAUSE } else { &PLAY };
    let icons = media_buttons(bounds)
        .iter()
        .zip([&PREVIOUS, toggle, &NEXT])
        .map(|(area, drawing)| Icon {
            x: area.x + (area.width - theme::MEDIA_ICON_SIDE) / 2.0,
            y: area.y + (area.height - theme::MEDIA_ICON_SIDE) / 2.0,
            side: theme::MEDIA_ICON_SIDE,
            image: Some(IconImage::Drawing(Arc::clone(drawing))),
            placeholder: theme::MEDIA_TINT,
            placeholder_radius: 0.0,
        })
        .collect();
    Controls {
        panel: Panel {
            bounds: pill(bounds),
            corner_radius: theme::MEDIA_HEIGHT / 2.0,
            tint: theme::MEDIA_TINT,
            border: theme::MEDIA_BORDER,
        },
        icons,
    }
}

fn pill(card: Rect) -> Rect {
    let width = theme::MEDIA_BUTTON_WIDTH * 3.0;
    let image_bottom = card.y + theme::PREVIEW_PADDING + theme::PREVIEW_IMAGE_HEIGHT;
    Rect {
        x: card.x + (card.width - width) / 2.0,
        y: image_bottom - theme::MEDIA_HEIGHT - theme::MEDIA_INSET,
        width,
        height: theme::MEDIA_HEIGHT,
    }
}
