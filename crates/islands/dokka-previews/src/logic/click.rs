use dokka_components::calendar_panel::{calendar_nav_pill, calendar_weeks};
use dokka_components::nav_pill::{NavHalf, NavPillProps, nav_pill_at};
use dokka_components::preview_card::{media_buttons, preview_slot, window_button_areas};
use dokka_scene::Rect;

use super::calendar_month;
use crate::{MediaCommand, PreviewClick, PreviewContent, Previews};

impl Previews {
    /// `None` si el clic cayó fuera del panel.
    pub fn click(&mut self, x: f32, y: f32) -> Option<PreviewClick> {
        if self.reveal.is_leaving() {
            return None;
        }
        let bounds = self.bounds()?;
        if !contains(bounds, x, y) {
            return None;
        }
        let content = self.open.as_ref()?.content.clone();
        match content {
            PreviewContent::Calendar => {
                let props = NavPillProps {
                    bounds: calendar_nav_pill(bounds),
                    can_previous: true,
                    can_next: true,
                    hovered: None,
                };
                // El mes que se deja también cuenta: si era de 6 semanas, no se encoge.
                let left = calendar_weeks(&calendar_month(self.today, self.month_offset));
                let stepped = match nav_pill_at(&props, x, y) {
                    Some(NavHalf::Previous) => {
                        self.month_offset -= 1;
                        true
                    }
                    Some(NavHalf::Next) => {
                        self.month_offset += 1;
                        true
                    }
                    None => false,
                };
                if stepped {
                    let weeks = calendar_weeks(&calendar_month(self.today, self.month_offset));
                    self.calendar_weeks = self.calendar_weeks.max(left).max(weeks);
                }
                Some(PreviewClick::Handled)
            }
            PreviewContent::Windows(windows) => {
                if let Some(click) = self.window_button_click(bounds, x, y) {
                    if matches!(click, PreviewClick::Close(_)) {
                        self.close();
                    }
                    return Some(click);
                }
                if let Some(command) = self.media_command(bounds, x, y) {
                    return Some(PreviewClick::Media(command));
                }
                let chosen = windows
                    .iter()
                    .enumerate()
                    .find(|(index, _)| contains(preview_slot(bounds, *index), x, y));
                Some(match chosen {
                    Some((_, item)) => {
                        self.close();
                        PreviewClick::Activate(item.window)
                    }
                    None => PreviewClick::Handled,
                })
            }
        }
    }

    fn window_button_click(&self, bounds: Rect, x: f32, y: f32) -> Option<PreviewClick> {
        let hovered = self.hovered?;
        let PreviewContent::Windows(windows) = self.open.as_ref()?.content.clone() else {
            return None;
        };
        let index = windows.iter().position(|item| item.window == hovered)?;
        let [close, pin] = window_button_areas(bounds, index);
        if contains(close, x, y) {
            Some(PreviewClick::Close(hovered))
        } else if contains(pin, x, y) {
            Some(PreviewClick::TogglePin(hovered))
        } else {
            None
        }
    }

    fn media_command(&self, bounds: Rect, x: f32, y: f32) -> Option<MediaCommand> {
        self.media.as_ref()?;
        let commands = [
            MediaCommand::Previous,
            MediaCommand::Toggle,
            MediaCommand::Next,
        ];
        media_buttons(bounds)
            .into_iter()
            .zip(commands)
            .find(|&(area, _)| contains(area, x, y))
            .map(|(_, command)| command)
    }
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x <= area.x + area.width && y >= area.y && y <= area.y + area.height
}
