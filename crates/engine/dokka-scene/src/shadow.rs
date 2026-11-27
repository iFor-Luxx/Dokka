use crate::Color;

/// Sombra negra suave bajo una pieza de cristal.
///
/// El color trae su propia opacidad (negro con alfa ~25-35%), el desenfoque
/// ronda 24-32 y el desplazamiento baja unos 8. La plataforma la dibuja con
/// `DropShadow` bajo el cristal, con la misma forma redondeada, sin tapar el
/// interior translúcido. `None` donde se pida es no tener sombra.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub color: Color,
    pub blur: f32,
    pub offset: (f32, f32),
}
