use dokka_scene::{Rect, Text};
use dokka_theme::popup as theme;

use super::NowPlaying;
use super::view::shorten;
use crate::popup_panel::text;

/// Zonas de anterior, reproducir/pausar y siguiente, al pie de la tarjeta.
pub fn media_buttons(bounds: Rect) -> [Rect; 3] {
    let left = bounds.x + (bounds.width - theme::MEDIA_BUTTON_WIDTH * 3.0) / 2.0;
    let top = bounds.y + bounds.height - theme::MEDIA_BUTTON_HEIGHT - 4.0;
    [0.0, 1.0, 2.0].map(|index| Rect {
        x: left + theme::MEDIA_BUTTON_WIDTH * index,
        y: top,
        width: theme::MEDIA_BUTTON_WIDTH,
        height: theme::MEDIA_BUTTON_HEIGHT,
    })
}

pub(super) fn media_texts(bounds: Rect, playing: &NowPlaying) -> Vec<Text> {
    let title_y = bounds.y + bounds.height - theme::MEDIA_HEIGHT + 10.0;
    let mut texts = vec![text(
        shorten(&playing.title),
        bounds.x + bounds.width / 2.0,
        title_y,
        theme::BODY_SIZE,
        true,
    )];
    let toggle = if playing.is_playing { "⏸" } else { "▶" };
    texts.extend(
        media_buttons(bounds)
            .iter()
            .zip(["⏮", toggle, "⏭"])
            .map(|(area, symbol)| {
                text(
                    symbol.to_owned(),
                    area.x + area.width / 2.0,
                    area.y + area.height / 2.0,
                    theme::MEDIA_SYMBOL_SIZE,
                    false,
                )
            }),
    );
    texts
}
