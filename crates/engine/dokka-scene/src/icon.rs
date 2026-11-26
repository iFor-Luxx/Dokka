use std::sync::Arc;

use crate::{Bitmap, Color, Drawing};

/// Contenido de un icono.
#[derive(Debug, Clone, PartialEq)]
pub enum IconImage {
    Pixels(Arc<Bitmap>),
    Drawing(Arc<Drawing>),
}

impl IconImage {
    /// `true` si es el mismo objeto en memoria, sin comparar su contenido.
    pub fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Pixels(a), Self::Pixels(b)) => Arc::ptr_eq(a, b),
            (Self::Drawing(a), Self::Drawing(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

/// Icono cuadrado dentro de un panel.
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    pub x: f32,
    pub y: f32,
    pub side: f32,
    pub image: Option<IconImage>,
    /// Color del cuadro que se muestra cuando no hay imagen.
    pub placeholder: Color,
    pub placeholder_radius: f32,
}
