use dokka_events::Event;

use crate::DockState;

impl DockState {
    /// Única entrada para modificar el estado.
    pub fn apply(&mut self, event: Event) {
        match event {
            Event::AppPinned(info) => self.pin(info),
            Event::AppUnpinned(app_id) => self.unpin(&app_id),
            Event::AppMoved { app_id, to_index } => self.move_app(&app_id, to_index),
            Event::WindowOpened(open) => self.track(open),
            Event::WindowClosed(window_id) => self.forget(window_id),
        }
    }
}
