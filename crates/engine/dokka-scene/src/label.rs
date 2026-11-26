use crate::Color;

/// Etiqueta flotante: píldora de cristal con una punta inferior.
/// La plataforma mide el texto y calcula el tamaño: el radio de la
/// píldora es la mitad de su alto y la punta usa el mismo tinte y borde,
/// sin costura entre ambas piezas.
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub text: String,
    pub center_x: f32,
    /// Extremo inferior de la punta.
    pub bottom: f32,
    pub font_size: f32,
    pub text_color: Color,
    pub tint: Color,
    pub border: Color,
    pub border_width: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub pointer_height: f32,
    /// Radio del vértice de la punta.
    pub pointer_radius: f32,
}
