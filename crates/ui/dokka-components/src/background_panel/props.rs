//! App en segundo plano: nombre ya limpio e icono real, si lo hay.

use std::sync::Arc;

use dokka_model::TrayItemId;
use dokka_scene::Bitmap;

/// Una fila del panel: nombre para mostrar e icono real de la bandeja.
#[derive(Debug, Clone)]
pub struct BackgroundApp {
    pub id: TrayItemId,
    pub name: String,
    pub icon: Option<Arc<Bitmap>>,
}

impl BackgroundApp {
    pub fn new(id: TrayItemId, name: impl Into<String>, icon: Option<Arc<Bitmap>>) -> Self {
        Self {
            id,
            name: name.into(),
            icon,
        }
    }
}

/// Fila bajo el punto, con su app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundHit {
    Row(TrayItemId),
}
