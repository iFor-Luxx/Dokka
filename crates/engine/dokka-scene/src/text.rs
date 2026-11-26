use crate::Color;

/// Cómo se coloca el texto respecto a `x`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Center,
    Left,
}

/// Texto de una línea. Con `Center` va centrado en `x`;
/// con `Left` empieza en `x`.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub x: f32,
    pub center_y: f32,
    pub content: String,
    pub size: f32,
    pub is_bold: bool,
    pub color: Color,
    pub align: TextAlign,
}
