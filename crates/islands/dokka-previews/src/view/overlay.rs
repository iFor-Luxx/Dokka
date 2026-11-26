use dokka_components::calendar_panel::{calendar_panel, calendar_size};
use dokka_components::popup_panel::place_above;
use dokka_components::preview_card::{preview_card, preview_size};
use dokka_scene::{Overlay, Rect};
use dokka_theme::popup as theme;

use crate::logic::calendar_month;
use crate::{PreviewContent, Previews};

impl Previews {
    /// Rectángulo del panel abierto.
    pub(crate) fn bounds(&self) -> Option<Rect> {
        let request = self.open.as_ref()?;
        let size = match &request.content {
            PreviewContent::Calendar => calendar_size(),
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

    /// `true` solo si el punto está dentro del panel abierto, sin el hueco
    /// que lo separa del dock (ese hueco sigue siendo del dock).
    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.bounds().is_some_and(|bounds| {
            x >= bounds.x
                && x <= bounds.x + bounds.width
                && y >= bounds.y
                && y <= bounds.y + bounds.height
        })
    }

    /// Zona que debe verse y recibir el mouse: el panel y el hueco hasta el dock.
    pub fn area(&self) -> Option<Rect> {
        let bounds = self.bounds()?;
        let floor = self.open.as_ref()?.floor;
        Some(Rect {
            height: floor - bounds.y,
            ..bounds
        })
    }

    pub fn overlay(&self) -> Option<Overlay> {
        let bounds = self.bounds()?;
        let request = self.open.as_ref()?;
        Some(match &request.content {
            PreviewContent::Calendar => {
                let month = calendar_month(self.today, self.month_offset);
                calendar_panel(&self.theme, bounds, &month)
            }
            PreviewContent::Windows(windows) => {
                let shown = visible_windows(windows.len(), request.limit);
                let media = self.media.as_ref().map(|now| (now, &self.media_icons));
                preview_card(&self.theme, bounds, &windows[..shown], media)
            }
        })
    }
}

/// Cuántas ventanas caben, hasta el máximo del tema.
fn visible_windows(count: usize, limit: f32) -> usize {
    let fits = ((limit - theme::PADDING * 2.0) / theme::PREVIEW_SLOT_WIDTH) as usize;
    count.min(theme::PREVIEW_MAXIMUM_WINDOWS).min(fits.max(1))
}
