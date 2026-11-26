use crate::{Color, Rect};

/// Panel de cristal con esquinas redondeadas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Panel {
    pub bounds: Rect,
    pub corner_radius: f32,
    pub border_width: f32,
    pub tint: Color,
    pub border: Color,
}
