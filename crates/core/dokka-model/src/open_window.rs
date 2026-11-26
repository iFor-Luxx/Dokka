use crate::{AppInfo, AppWindow};

/// Ventana detectada junto con la aplicación a la que pertenece.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenWindow {
    pub app: AppInfo,
    pub window: AppWindow,
}
