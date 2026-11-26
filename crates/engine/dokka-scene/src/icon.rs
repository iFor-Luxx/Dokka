use std::sync::Arc;

use crate::{Bitmap, Color};

/// Icono cuadrado dentro de un panel.
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    pub x: f32,
    pub y: f32,
    pub side: f32,
    pub image: Option<Arc<Bitmap>>,
    /// Color del cuadro que se muestra cuando no hay imagen.
    pub placeholder: Color,
    pub placeholder_radius: f32,
}
