use dokka_model::{OpenWindow, WindowId};

use crate::DockState;

impl DockState {
    pub(crate) fn track(&mut self, open: OpenWindow) {
        let app = self.find_or_add(open.app);
        match app.windows.iter_mut().find(|w| w.id == open.window.id) {
            Some(known) => *known = open.window,
            None => app.windows.push(open.window),
        }
    }

    pub(crate) fn forget(&mut self, window_id: WindowId) {
        for app in &mut self.apps {
            app.windows.retain(|window| window.id != window_id);
        }
        self.remove_idle_apps();
    }
}
