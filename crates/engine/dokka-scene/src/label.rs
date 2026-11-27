use crate::Color;

/// Etiqueta flotante: píldora de cristal con punta hacia el icono.
/// La plataforma mide el texto y calcula el tamaño: el radio de la
/// píldora es la mitad de su alto y la punta usa el mismo tinte y borde,
/// sin costura entre ambas piezas.
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub text: String,
    pub center_x: f32,
    /// Centro vertical para etiquetas laterales; sin uso abajo.
    pub center_y: f32,
    /// Extremo de la punta hacia el icono (abajo o al lado).
    pub bottom: f32,
    /// Lado donde está el icono respecto a la píldora.
    pub side: LabelSide,
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

/// Dónde está el icono respecto a la etiqueta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelSide {
    #[default]
    Below,
    /// Icono a la izquierda: la etiqueta va a la derecha (dock a la izquierda).
    After,
    /// Icono a la derecha: la etiqueta va a la izquierda (dock a la derecha).
    Before,
}
