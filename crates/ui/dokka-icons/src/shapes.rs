use dokka_scene::{Color, DRAWING_SIDE, Shape};

// Las coordenadas son el dibujo mismo; los colores llegan del tema.
const MARGIN: f32 = 2.0;
const TILE_RADIUS: f32 = 22.0;

pub(crate) fn rounded(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) -> Shape {
    Shape::RoundedRect {
        x,
        y,
        width,
        height,
        radius,
        color,
    }
}

/// Fondo cuadrado redondeado común a los iconos.
pub(crate) fn tile(color: Color) -> Shape {
    let side = DRAWING_SIDE - MARGIN * 2.0;
    rounded(MARGIN, MARGIN, side, side, TILE_RADIUS, color)
}

pub(crate) fn circle(x: f32, y: f32, radius: f32, color: Color) -> Shape {
    Shape::Circle {
        x,
        y,
        radius,
        color,
    }
}
