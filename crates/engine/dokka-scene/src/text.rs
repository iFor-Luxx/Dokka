use crate::Color;

/// Texto de una línea centrado en un punto.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub center_x: f32,
    pub center_y: f32,
    pub content: String,
    pub size: f32,
    pub is_bold: bool,
    pub color: Color,
}
