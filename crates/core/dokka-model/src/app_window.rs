use crate::WindowId;

/// Ventana abierta que pertenece a una aplicación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppWindow {
    pub id: WindowId,
    pub title: String,
    pub is_minimized: bool,
}
