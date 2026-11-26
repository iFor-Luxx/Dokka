use std::sync::Arc;

use dokka_icons::{preview_close_icon, preview_pin_icon, preview_pinned_icon};
use dokka_scene::{Drawing, Icon, IconImage, Rect};
use dokka_theme::{Theme, popup as measures};

use super::PreviewHover;

/// Botones de cerrar y fijar, ya construidos con los colores de un tema.
/// Se reutilizan entre fotogramas: el pintor solo repinta si cambia el dibujo.
#[derive(Debug)]
pub struct WindowButtonIcons {
    close: Arc<Drawing>,
    pin: Arc<Drawing>,
    pinned: Arc<Drawing>,
}

impl WindowButtonIcons {
    pub fn new(theme: &Theme) -> Self {
        let glyph = theme.panel.text;
        let border = theme.panel.border;
        Self {
            close: Arc::new(preview_close_icon(glyph, theme.panel.highlight, border)),
            pin: Arc::new(preview_pin_icon(glyph, theme.panel.highlight, border)),
            pinned: Arc::new(preview_pinned_icon(glyph, theme.panel.background, border)),
        }
    }
}

/// Zonas de cerrar y fijar, arriba a la izquierda de la imagen `index`.
pub fn window_button_areas(bounds: Rect, index: usize) -> [Rect; 2] {
    let slot = super::preview_slot(bounds, index);
    let image_x = slot.x + measures::PREVIEW_IMAGE_INSET;
    let image_y = slot.y;
    let y = image_y + measures::PREVIEW_BUTTON_INSET;
    let close = Rect {
        x: image_x + measures::PREVIEW_BUTTON_INSET,
        y,
        width: measures::PREVIEW_BUTTON_SIDE,
        height: measures::PREVIEW_BUTTON_SIDE,
    };
    let pin = Rect {
        x: close.x + measures::PREVIEW_BUTTON_SIDE + measures::PREVIEW_BUTTON_GAP,
        y,
        width: measures::PREVIEW_BUTTON_SIDE,
        height: measures::PREVIEW_BUTTON_SIDE,
    };
    [close, pin]
}

pub(super) fn button_icons(
    theme: &Theme,
    buttons: &WindowButtonIcons,
    areas: &[Rect; 2],
    hovered: &PreviewHover,
) -> Vec<Icon> {
    let pin = if hovered.is_pinned {
        &buttons.pinned
    } else {
        &buttons.pin
    };
    areas
        .iter()
        .zip([&buttons.close, pin])
        .map(|(area, drawing)| Icon {
            x: area.x,
            y: area.y,
            side: measures::PREVIEW_BUTTON_SIDE,
            image: Some(IconImage::Drawing(Arc::clone(drawing))),
            placeholder: theme.panel.background,
            placeholder_radius: 0.0,
        })
        .collect()
}
