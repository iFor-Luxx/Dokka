use dokka_scene::{Overlay, Rect, Size, Text, Thumbnail};
use dokka_theme::{Theme, popup as measures, typography};

use super::media_controls::media_controls;
use super::{MediaIcons, NowPlaying, PreviewWindow};
use crate::popup_panel::{popup_panel, text};

/// Con música no hay títulos: la imagen y los botones bastan.
pub fn preview_size(windows: usize, has_media: bool) -> Size {
    let below = if has_media {
        measures::PREVIEW_PADDING
    } else {
        measures::PREVIEW_TITLE_HEIGHT
    };
    Size {
        width: measures::PREVIEW_SLOT_WIDTH * windows as f32 + measures::PREVIEW_PADDING * 2.0,
        height: measures::PREVIEW_PADDING + measures::PREVIEW_IMAGE_HEIGHT + below,
    }
}

/// Zona pulsable de la ventana número `index`.
pub fn preview_slot(bounds: Rect, index: usize) -> Rect {
    Rect {
        x: bounds.x + measures::PREVIEW_PADDING + measures::PREVIEW_SLOT_WIDTH * index as f32,
        y: bounds.y + measures::PREVIEW_PADDING,
        width: measures::PREVIEW_SLOT_WIDTH,
        height: bounds.height - measures::PREVIEW_PADDING,
    }
}

pub fn preview_card(
    theme: &Theme,
    bounds: Rect,
    windows: &[PreviewWindow],
    media: Option<(&NowPlaying, &MediaIcons)>,
) -> Overlay {
    let slots = || (0..windows.len()).map(|index| preview_slot(bounds, index));
    let inset = measures::PREVIEW_IMAGE_INSET;
    let thumbnails = slots()
        .zip(windows)
        .map(|(slot, item)| Thumbnail {
            window: item.window,
            bounds: Rect {
                x: slot.x + inset,
                y: slot.y,
                width: slot.width - inset * 2.0,
                height: measures::PREVIEW_IMAGE_HEIGHT,
            },
            reserved_corner: 0.0,
        })
        .collect();
    let texts = if media.is_some() {
        Vec::new()
    } else {
        slots()
            .zip(windows)
            .map(|(slot, item)| title(theme, slot, &item.title))
            .collect()
    };

    Overlay {
        panel: popup_panel(theme, bounds),
        highlight: None,
        dividers: Vec::new(),
        dots: Vec::new(),
        icons: Vec::new(),
        texts,
        thumbnails,
        controls: media.map(|(playing, icons)| media_controls(theme, icons, bounds, playing)),
    }
}

fn title(theme: &Theme, slot: Rect, title: &str) -> Text {
    let center = (
        slot.x + slot.width / 2.0,
        slot.y + measures::PREVIEW_IMAGE_HEIGHT + measures::PREVIEW_TITLE_HEIGHT / 2.0,
    );
    text(theme, shorten(title), center, typography::BODY_SIZE, false)
}

fn shorten(title: &str) -> String {
    let limit = measures::PREVIEW_TITLE_CHARACTERS;
    if title.chars().count() <= limit {
        return title.to_owned();
    }
    let kept: String = title.chars().take(limit - 1).collect();
    format!("{}…", kept.trim_end())
}
