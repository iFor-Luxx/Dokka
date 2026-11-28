use crate::Color;

/// Cursor tras un texto de una línea: la escena dice detrás de qué texto va y
/// la plataforma mide ESE texto con la misma fuente, tamaño y peso con que lo
/// pinta (incluyendo los espacios finales) y coloca ahí el cursor.
///
/// Con la casilla vacía el cursor va al inicio del texto. `max_x` es el borde
/// derecho que ni el texto ni el cursor pueden pasar. `visible` junta el foco
/// y el parpadeo: `false` lo oculta sin destruir la capa.
#[derive(Debug, Clone, PartialEq)]
pub struct TextCursor {
    /// Borde izquierdo donde empieza el texto.
    pub text_x: f32,
    /// Centro vertical del texto; el cursor se centra en él.
    pub center_y: f32,
    /// Lo escrito; vacío deja el cursor al inicio.
    pub content: String,
    /// Tamaño y peso iguales a los del `Text` pintado.
    pub size: f32,
    pub is_bold: bool,
    /// Borde derecho interior de la casilla.
    pub max_x: f32,
    /// Aire entre la última letra y el cursor (0 con la casilla vacía).
    pub gap: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub visible: bool,
}
