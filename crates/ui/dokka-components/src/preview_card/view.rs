use dokka_scene::{Overlay, Rect, Size, Thumbnail};
use dokka_theme::popup as theme;

use super::media_controls::media_texts;
use super::{NowPlaying, PreviewWindow};
use crate::popup_panel::{popup_panel, text};

/// Caracteres que caben en el título de una ventana.
const TITLE_CHARACTERS: usize = 28;

pub fn preview_size(windows: usize, has_media: bool) -> Size {
    let media = if has_media { theme::MEDIA_HEIGHT } else { 0.0 };
    Size {
        width: theme::PREVIEW_SLOT_WIDTH * windows as f32 + theme::PADDING * 2.0,
        height: theme::PREVIEW_IMAGE_HEIGHT
            + theme::PREVIEW_TITLE_HEIGHT
            + theme::PADDING
            + media,
    }
}

/// Zona pulsable de la ventana número `index`.
pub fn preview_slot(bounds: Rect, index: usize) -> Rect {
    Rect {
        x: bounds.x + theme::PADDING + theme::PREVIEW_SLOT_WIDTH * index as f32,
        y: bounds.y + theme::PADDING,
        width: theme::PREVIEW_SLOT_WIDTH,
        height: theme::PREVIEW_IMAGE_HEIGHT + theme::PREVIEW_TITLE_HEIGHT,
    }
}

pub fn preview_card(
    bounds: Rect,
    windows: &[PreviewWindow],
    media: Option<&NowPlaying>,
) -> Overlay {
    let slots = || (0..windows.len()).map(|index| preview_slot(bounds, index));
    let thumbnails = slots()
        .zip(windows)
        .map(|(slot, item)| Thumbnail {
            window: item.window,
            bounds: Rect {
                x: slot.x + 6.0,
                y: slot.y,
                width: slot.width - 12.0,
                height: theme::PREVIEW_IMAGE_HEIGHT,
            },
            reserved_corner: 0.0,
        })
        .collect();
    let mut texts: Vec<_> = slots()
        .zip(windows)
        .map(|(slot, item)| {
            let center_y = slot.y + theme::PREVIEW_IMAGE_HEIGHT + theme::PREVIEW_TITLE_HEIGHT / 2.0;
            let x = slot.x + slot.width / 2.0;
            text(shorten(&item.title), x, center_y, theme::BODY_SIZE, false)
        })
        .collect();
    if let Some(playing) = media {
        texts.extend(media_texts(bounds, playing));
    }

    Overlay {
        panel: popup_panel(bounds),
        dots: Vec::new(),
        texts,
        thumbnails,
    }
}

pub(super) fn shorten(title: &str) -> String {
    if title.chars().count() <= TITLE_CHARACTERS {
        return title.to_owned();
    }
    let kept: String = title.chars().take(TITLE_CHARACTERS - 1).collect();
    format!("{}…", kept.trim_end())
}
