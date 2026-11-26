use crate::{Color, Size};

/// Panel de cristal con esquinas redondeadas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Panel {
    pub size: Size,
    pub corner_radius: f32,
    pub tint: Color,
    pub border: Color,
}
