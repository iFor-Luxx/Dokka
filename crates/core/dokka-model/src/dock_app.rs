use crate::{AppInfo, AppWindow};

/// Aplicación presente en el dock, por estar fijada, abierta o ambas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DockApp {
    pub info: AppInfo,
    pub is_pinned: bool,
    pub windows: Vec<AppWindow>,
}

impl DockApp {
    pub fn new(info: AppInfo) -> Self {
        Self {
            info,
            is_pinned: false,
            windows: Vec::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        !self.windows.is_empty()
    }

    /// Una app sin fijar y sin ventanas no tiene motivo para seguir en el dock.
    pub fn should_stay(&self) -> bool {
        self.is_pinned || self.is_running()
    }
}
