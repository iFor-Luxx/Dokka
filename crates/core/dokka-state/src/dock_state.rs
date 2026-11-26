use dokka_model::{AppId, AppInfo, DockApp};

/// Apps del dock en el orden en que se muestran.
#[derive(Debug, Default)]
pub struct DockState {
    pub(crate) apps: Vec<DockApp>,
}

impl DockState {
    pub fn apps(&self) -> &[DockApp] {
        &self.apps
    }

    pub(crate) fn position_of(&self, app_id: &AppId) -> Option<usize> {
        self.apps.iter().position(|app| &app.info.id == app_id)
    }

    /// Devuelve la app del dock, añadiéndola al final si no estaba.
    pub(crate) fn find_or_add(&mut self, info: AppInfo) -> &mut DockApp {
        let index = match self.position_of(&info.id) {
            Some(index) => index,
            None => {
                self.apps.push(DockApp::new(info));
                self.apps.len() - 1
            }
        };
        &mut self.apps[index]
    }

    pub(crate) fn remove_idle_apps(&mut self) {
        self.apps.retain(DockApp::should_stay);
    }
}
