use dokka_components::calendar_panel::{
    calendar_month, calendar_panel, calendar_pill_props, calendar_size, calendar_weeks,
};
use dokka_components::popup_panel::place_above;
use dokka_components::preview_card::{PreviewHover, preview_card, preview_size};
use dokka_scene::{Overlay, Rect};
use dokka_theme::popup as theme;

use crate::{PreviewContent, Previews};

impl Previews {
    /// Rectángulo del panel abierto, en reposo.
    pub(crate) fn bounds(&self) -> Option<Rect> {
        let request = self.open.as_ref()?;
        let size = match &request.content {
            PreviewContent::Calendar => {
                let month = calendar_month(self.today, self.month_offset);
                // El recuerdo solo crece: el panel nunca se encoge mientras
                // está abierto. Al abrir es 0 y manda el mes actual.
                calendar_size(calendar_weeks(&month).max(self.calendar_weeks))
            }
            PreviewContent::Windows(windows) => preview_size(
                visible_windows(windows.len(), request.limit),
                self.media.is_some(),
            ),
        };
        Some(place_above(
            size,
            request.center_x,
            request.bottom,
            request.limit,
        ))
    }

    /// Rectángulo tal como se dibuja, desplazado hacia abajo según la animación.
    /// La escala la aplica la plataforma sobre este rectángulo.
    fn drawn(&self) -> Option<Rect> {
        let rest = self.bounds()?;
        Some(Rect {
            y: rest.y + self.drop(),
            ..rest
        })
    }

    /// Cuánto baja el panel respecto al reposo. En el rebote es negativo.
    fn drop(&self) -> f32 {
        self.reveal
            .drop(theme::POPUP_ENTER_TRAVEL, theme::POPUP_EXIT_TRAVEL)
    }

    /// Escala actual del panel, con origen en el centro inferior sobre el icono.
    fn scale(&self) -> f32 {
        self.reveal
            .scale(theme::POPUP_ENTER_SCALE, theme::POPUP_EXIT_SCALE)
    }

    /// Punto que queda fijo al escalar: centro inferior sobre el icono.
    fn origin(&self) -> Option<(f32, f32)> {
        let request = self.open.as_ref()?;
        Some((request.center_x, request.bottom))
    }

    /// `true` solo si el punto está dentro del panel abierto, sin el hueco
    /// que lo separa del dock (ese hueco sigue siendo del dock).
    /// Durante la salida no acepta clics.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        if self.reveal.is_leaving() {
            return false;
        }
        self.bounds().is_some_and(|bounds| {
            x >= bounds.x
                && x <= bounds.x + bounds.width
                && y >= bounds.y
                && y <= bounds.y + bounds.height
        })
    }

    /// Zona que debe verse y recibir el mouse: lo dibujado y el hueco hasta el dock.
    /// Cubre también el rebote, que se pasa un poco por encima del reposo.
    pub fn area(&self) -> Option<Rect> {
        let rest = self.bounds()?;
        let drawn = self.drawn()?;
        let origin = self.origin()?;
        let scaled = scaled_rect(drawn, self.scale(), origin);
        let floor = self.open.as_ref()?.floor;
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

    pub fn overlay(&self) -> Option<Overlay> {
        let drawn = self.drawn()?;
        let request = self.open.as_ref()?;
        let mut overlay = match &request.content {
            PreviewContent::Calendar => {
                let month = calendar_month(self.today, self.month_offset);
                let pill = calendar_pill_props(drawn, self.nav_hovered);
                calendar_panel(&self.theme, &self.nav_pill, drawn, &month, &pill, false)
            }
            PreviewContent::Windows(windows) => {
                let shown = visible_windows(windows.len(), request.limit);
                let media = self.media.as_ref().map(|now| (now, &self.media_icons));
                let hovered = self.hovered_index(&windows[..shown]);
                preview_card(
                    &self.theme,
                    &self.window_buttons,
                    drawn,
                    &windows[..shown],
                    media,
                    hovered,
                )
            }
        };
        // Los botones van en otra ventana: aparecen al terminar la entrada
        // para no moverla en cada fotograma.
        if self.reveal.is_animating() {
            overlay.controls = None;
        }
        overlay.opacity = self.reveal.fade(theme::POPUP_OPACITY_SPAN);
        overlay.scale = self.scale();
        overlay.origin = self.origin()?;
        Some(overlay)
    }

    /// Mini bajo el cursor con su estado de fijada, si sigue visible.
    fn hovered_index(
        &self,
        shown: &[dokka_components::preview_card::PreviewWindow],
    ) -> Option<PreviewHover> {
        let hovered = self.hovered?;
        let index = shown.iter().position(|item| item.window == hovered)?;
        Some(PreviewHover {
            index,
            is_pinned: self.pinned.contains(&hovered),
        })
    }
}

/// Rectángulo tras escalarlo sobre `origin`.
fn scaled_rect(rect: Rect, scale: f32, origin: (f32, f32)) -> Rect {
    Rect {
        x: origin.0 + (rect.x - origin.0) * scale,
        y: origin.1 + (rect.y - origin.1) * scale,
        width: rect.width * scale,
        height: rect.height * scale,
    }
}

/// Cuántas ventanas caben, hasta el máximo del tema.
fn visible_windows(count: usize, limit: f32) -> usize {
    let fits = ((limit - theme::PADDING * 2.0) / theme::PREVIEW_SLOT_WIDTH) as usize;
    count.min(theme::PREVIEW_MAXIMUM_WINDOWS).min(fits.max(1))
}
