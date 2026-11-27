use dokka_components::calendar_panel::{calendar_month, calendar_panel, calendar_pill_props};
use dokka_components::preview_card::{PreviewHover, preview_card};
use dokka_scene::Overlay;
use dokka_theme::popup as theme;

use super::place::visible_windows;
use crate::{PreviewContent, Previews};

impl Previews {
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

    pub fn overlay(&self) -> Option<Overlay> {
        let drawn = self.drawn()?;
        let request = self.open.as_ref()?;
        let mut overlay = match &request.content {
            PreviewContent::Calendar => {
                let month = calendar_month(self.today, self.month_offset);
                let pill = calendar_pill_props(drawn, self.nav_hovered);
                calendar_panel(&self.theme, &self.nav_pill, drawn, &month, &pill)
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
