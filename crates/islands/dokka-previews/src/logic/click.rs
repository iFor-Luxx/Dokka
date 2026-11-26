use dokka_components::calendar_panel::calendar_arrows;
use dokka_components::preview_card::{media_buttons, preview_slot};
use dokka_scene::Rect;

use crate::{MediaCommand, PreviewClick, PreviewContent, Previews};

impl Previews {
    /// `None` si el clic cayó fuera del panel.
    pub fn click(&mut self, x: f32, y: f32) -> Option<PreviewClick> {
        let bounds = self.bounds()?;
        if !contains(bounds, x, y) {
            return None;
        }
        let content = self.open.as_ref()?.content.clone();
        match content {
            PreviewContent::Calendar => {
                let (previous, next) = calendar_arrows(bounds);
                if contains(previous, x, y) {
                    self.month_offset -= 1;
                } else if contains(next, x, y) {
                    self.month_offset += 1;
                }
                Some(PreviewClick::Handled)
            }
            PreviewContent::Windows(windows) => {
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
