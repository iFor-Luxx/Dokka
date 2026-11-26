use crate::Color;

/// Lado del lienzo cuadrado en el que se describen los dibujos.
pub const DRAWING_SIDE: f32 = 100.0;

/// Forma plana en unidades del lienzo.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    RoundedRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
        color: Color,
    },
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: Color,
    },
    Line {
        from: (f32, f32),
        to: (f32, f32),
        width: f32,
        color: Color,
    },
    Triangle {
        points: [(f32, f32); 3],
        color: Color,
    },
    /// Texto centrado en (`x`, `y`).
    Text {
        text: String,
        x: f32,
        y: f32,
        size: f32,
        is_bold: bool,
        color: Color,
    },
}

/// Imagen vectorial: las formas se pintan en orden.
#[derive(Debug, Clone, PartialEq)]
pub struct Drawing {
    pub shapes: Vec<Shape>,
}
