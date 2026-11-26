use dokka_scene::{Controls, Overlay, Panel, Rect, Size, Text, Thumbnail};
use dokka_theme::{Theme, popup as measures, typography};

use super::media_controls::media_controls;
use super::window_buttons::{button_icons, window_button_areas};
use super::{MediaIcons, NowPlaying, PreviewHover, PreviewWindow, WindowButtonIcons};
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
    buttons: &WindowButtonIcons,
    bounds: Rect,
    windows: &[PreviewWindow],
    media: Option<(&NowPlaying, &MediaIcons)>,
    hovered: Option<PreviewHover>,
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
    let hovered_valid = hovered.filter(|hovered| hovered.index < windows.len());

    Overlay {
        panel: popup_panel(theme, bounds),
        highlight: None,
        dividers: Vec::new(),
        dots: Vec::new(),
        pointer: None,
        texts,
        icons: Vec::new(),
        thumbnails,
        controls: preview_controls(theme, buttons, bounds, media, hovered_valid),
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Botones de la segunda ventana: la píldora de música y, si el cursor está
/// sobre una mini, cerrar y fijar arriba a la izquierda de su imagen.
fn preview_controls(
    theme: &Theme,
    buttons: &WindowButtonIcons,
    bounds: Rect,
    media: Option<(&NowPlaying, &MediaIcons)>,
    hovered: Option<PreviewHover>,
) -> Option<Controls> {
    let mut icons = Vec::new();
    let mut panel = None;
    if let Some((playing, media_icons)) = media {
        let controls = media_controls(theme, media_icons, bounds, playing);
        panel = Some(controls.panel);
        icons.extend(controls.icons);
    }
    if let Some(hovered) = hovered {
        let areas = window_button_areas(bounds, hovered.index);
        if panel.is_none() {
            panel = Some(hidden_panel(theme, &areas));
        }
        icons.extend(button_icons(theme, buttons, &areas, &hovered));
    }
    panel.map(|panel| Controls { panel, icons })
}

/// Cristal redondo tras el primer botón cuando no hay música: queda tapado
/// por el propio botón opaco y solo define la ventana superpuesta.
fn hidden_panel(theme: &Theme, areas: &[Rect; 2]) -> Panel {
    Panel {
        bounds: areas[0],
        corner_radius: measures::PREVIEW_BUTTON_SIDE / 2.0,
        border_width: measures::BORDER_WIDTH,
        tint: theme.panel.background,
        border: theme.panel.border,
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
