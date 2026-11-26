use crate::Color;

/// Punto circular; `x` e `y` son su esquina superior izquierda.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot {
    pub x: f32,
    pub y: f32,
    pub diameter: f32,
    pub color: Color,
}
