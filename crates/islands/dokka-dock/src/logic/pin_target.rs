use dokka_model::DockApp;

use super::decide_click::entry_at;
use super::{Entry, place_dock};
use crate::Dock;

impl Dock {
    /// App que se puede fijar o quitar en ese punto. El explorador es permanente.
    pub fn pin_target<'a>(&self, x: f32, y: f32, apps: &'a [DockApp]) -> Option<&'a DockApp> {
        match entry_at(&place_dock(self, apps), x, y)? {
            Entry::App(app) => Some(app),
            _ => None,
        }
    }
}
