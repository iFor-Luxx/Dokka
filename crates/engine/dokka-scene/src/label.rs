use crate::Color;

/// Texto de una línea sobre un fondo redondeado con una punta inferior.
/// La plataforma mide el texto y calcula el tamaño.
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub text: String,
    pub center_x: f32,
    /// Extremo inferior de la punta.
    pub bottom: f32,
    pub font_size: f32,
    pub text_color: Color,
    pub background: Color,
    pub padding_x: f32,
    pub padding_y: f32,
    pub corner_radius: f32,
    pub pointer_height: f32,
}
