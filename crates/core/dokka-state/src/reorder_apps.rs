use dokka_model::AppId;

use crate::DockState;

impl DockState {
    /// Un destino fuera de rango deja la app al final.
    pub(crate) fn move_app(&mut self, app_id: &AppId, to_index: usize) {
        if let Some(from_index) = self.position_of(app_id) {
            let app = self.apps.remove(from_index);
            let target = to_index.min(self.apps.len());
            self.apps.insert(target, app);
        }
    }
}
