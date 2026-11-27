//! Dónde va el panel según el borde: arriba o a un lado, sin salirse.

use dokka_components::calendar_panel::calendar_size;
use dokka_components::popup_panel::{place_above, place_after, place_before};
use dokka_components::preview_card::preview_size;
use dokka_model::DockPosition;
use dokka_scene::Rect;
use dokka_theme::popup as theme;

use crate::{PreviewContent, Previews};

impl Previews {
    /// Rectángulo del panel abierto, en reposo.
    pub(crate) fn bounds(&self) -> Option<Rect> {
        let request = self.open.as_ref()?;
        let size = match &request.content {
            PreviewContent::Calendar => calendar_size(),
            PreviewContent::Windows(windows) => preview_size(
                visible_windows(windows.len(), request.limit),
                self.media.is_some(),
            ),
        };
        match request.position {
            DockPosition::Left => Some(place_after(
                size,
                request.edge,
                request.center_y,
                request.limit_along,
            )),
            DockPosition::Right => Some(place_before(
                size,
                request.edge,
                request.center_y,
                request.limit_along,
            )),
            _ => Some(place_above(
                size,
                request.center_x,
                request.bottom,
                request.limit,
            )),
        }
    }

    /// Rectángulo tal como se dibuja, desplazado según la animación.
    /// La escala la aplica la plataforma sobre este rectángulo.
    pub(crate) fn drawn(&self) -> Option<Rect> {
        let rest = self.bounds()?;
        let request = self.open.as_ref()?;
        match request.position {
            DockPosition::Left => Some(Rect {
                x: rest.x - self.drop(),
                ..rest
            }),
            DockPosition::Right => Some(Rect {
                x: rest.x + self.drop(),
                ..rest
            }),
            _ => Some(Rect {
                y: rest.y + self.drop(),
                ..rest
            }),
        }
    }

    /// Cuánto se desplaza el panel respecto al reposo. En el rebote es negativo.
    pub(crate) fn drop(&self) -> f32 {
        self.reveal
            .drop(theme::POPUP_ENTER_TRAVEL, theme::POPUP_EXIT_TRAVEL)
    }

    /// Escala actual del panel, con origen sobre el icono.
    pub(crate) fn scale(&self) -> f32 {
        self.reveal
            .scale(theme::POPUP_ENTER_SCALE, theme::POPUP_EXIT_SCALE)
    }

    /// Punto que queda fijo al escalar: sobre el icono.
    pub(crate) fn origin(&self) -> Option<(f32, f32)> {
        let request = self.open.as_ref()?;
        match request.position {
            DockPosition::Left | DockPosition::Right => Some((request.edge, request.center_y)),
            _ => Some((request.center_x, request.bottom)),
        }
    }

    /// Zona que debe verse y recibir el mouse: lo dibujado y el hueco hasta el dock.
    /// Cubre también el rebote, que se pasa un poco por encima del reposo.
    pub fn area(&self) -> Option<Rect> {
        let rest = self.bounds()?;
        let drawn = self.drawn()?;
        let origin = self.origin()?;
        let scaled = scaled_rect(drawn, self.scale(), origin);
        let request = self.open.as_ref()?;
        match request.position {
            DockPosition::Left => {
                let left = request.bar_side.min(scaled.x);
                let top = rest.y.min(scaled.y);
                let right = (rest.x + rest.width).max(scaled.x + scaled.width);
                let bottom = (rest.y + rest.height).max(scaled.y + scaled.height);
                Some(Rect {
                    x: left,
                    y: top,
                    width: (right - left).max(0.0),
                    height: (bottom - top).max(0.0),
                })
            }
            DockPosition::Right => {
                let right = request.bar_side.max(scaled.x + scaled.width);
                let left = rest.x.min(scaled.x);
                let top = rest.y.min(scaled.y);
                let bottom = (rest.y + rest.height).max(scaled.y + scaled.height);
                Some(Rect {
                    x: left,
                    y: top,
                    width: (right - left).max(0.0),
                    height: (bottom - top).max(0.0),
                })
            }
            _ => {
                let floor = request.floor;
                let left = rest.x.min(scaled.x);
                let top = rest.y.min(scaled.y);
                let right = (rest.x + rest.width).max(scaled.x + scaled.width);
                let bottom = floor.max(scaled.y + scaled.height);
                Some(Rect {
                    x: left,
                    y: top,
                    width: (right - left).max(0.0),
                    height: (bottom - top).max(0.0),
                })
            }
        }
    }
}

/// Rectángulo tras escalarlo sobre `origin`.
pub(crate) fn scaled_rect(rect: Rect, scale: f32, origin: (f32, f32)) -> Rect {
    Rect {
        x: origin.0 + (rect.x - origin.0) * scale,
        y: origin.1 + (rect.y - origin.1) * scale,
        width: rect.width * scale,
        height: rect.height * scale,
    }
}

/// Cuántas ventanas caben, hasta el máximo del tema.
pub(crate) fn visible_windows(count: usize, limit: f32) -> usize {
    let fits = ((limit - theme::PADDING * 2.0) / theme::PREVIEW_SLOT_WIDTH) as usize;
    count.min(theme::PREVIEW_MAXIMUM_WINDOWS).min(fits.max(1))
}
