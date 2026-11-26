use dokka_scene::{Overlay, Panel, Rect, Size, Text, Thumbnail};
use dokka_theme::popup as theme;

use super::media_controls::media_controls;
use super::{NowPlaying, PreviewWindow};

/// Caracteres que caben en el título de una ventana.
const TITLE_CHARACTERS: usize = 36;

/// Con música no hay títulos: la imagen y los botones bastan.
pub fn preview_size(windows: usize, has_media: bool) -> Size {
    let below = if has_media {
        theme::PREVIEW_PADDING
    } else {
        theme::PREVIEW_TITLE_HEIGHT
    };
    Size {
        width: theme::PREVIEW_SLOT_WIDTH * windows as f32 + theme::PREVIEW_PADDING * 2.0,
        height: theme::PREVIEW_PADDING + theme::PREVIEW_IMAGE_HEIGHT + below,
    }
}

/// Zona pulsable de la ventana número `index`.
pub fn preview_slot(bounds: Rect, index: usize) -> Rect {
    Rect {
        x: bounds.x + theme::PREVIEW_PADDING + theme::PREVIEW_SLOT_WIDTH * index as f32,
        y: bounds.y + theme::PREVIEW_PADDING,
        width: theme::PREVIEW_SLOT_WIDTH,
        height: bounds.height - theme::PREVIEW_PADDING,
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
                x: slot.x + 2.0,
                y: slot.y,
                width: slot.width - 4.0,
                height: theme::PREVIEW_IMAGE_HEIGHT,
            },
            reserved_corner: 0.0,
        })
        .collect();
    let texts = if media.is_some() {
        Vec::new()
    } else {
        slots()
            .zip(windows)
            .map(|(slot, item)| title(slot, &item.title))
            .collect()
    };

    Overlay {
        panel: Panel {
            bounds,
            corner_radius: theme::CORNER_RADIUS,
            tint: theme::PREVIEW_TINT,
            border: theme::PREVIEW_BORDER,
        },
        dots: Vec::new(),
        texts,
        thumbnails,
        controls: media.map(|playing| media_controls(bounds, playing)),
    }
}

fn title(slot: Rect, title: &str) -> Text {
    Text {
        center_x: slot.x + slot.width / 2.0,
        center_y: slot.y + theme::PREVIEW_IMAGE_HEIGHT + theme::PREVIEW_TITLE_HEIGHT / 2.0,
        content: shorten(title),
        size: theme::BODY_SIZE,
        is_bold: false,
        color: theme::PREVIEW_INK,
    }
}

fn shorten(title: &str) -> String {
    if title.chars().count() <= TITLE_CHARACTERS {
        return title.to_owned();
    }
    let kept: String = title.chars().take(TITLE_CHARACTERS - 1).collect();
    format!("{}…", kept.trim_end())
}
