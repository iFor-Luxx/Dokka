use dokka_model::{AppId, AppInfo};

use crate::DockState;

impl DockState {
    pub(crate) fn pin(&mut self, info: AppInfo) {
        self.find_or_add(info).is_pinned = true;
    }

    pub(crate) fn unpin(&mut self, app_id: &AppId) {
        if let Some(index) = self.position_of(app_id) {
            self.apps[index].is_pinned = false;
            self.remove_idle_apps();
        }
    }
}
