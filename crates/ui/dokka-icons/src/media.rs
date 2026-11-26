use dokka_scene::{Color, Drawing, Shape};

use crate::shapes::rounded;

/// Oscuro, para leerse sobre la píldora clara.
const INK: Color = Color::rgba(30, 32, 36, 255);

/// Dos triángulos hacia la izquierda.
pub fn previous_icon() -> Drawing {
    Drawing {
        shapes: vec![
            triangle([(50.0, 24.0), (50.0, 76.0), (10.0, 50.0)]),
            triangle([(88.0, 24.0), (88.0, 76.0), (48.0, 50.0)]),
        ],
    }
}

/// Dos triángulos hacia la derecha.
pub fn next_icon() -> Drawing {
    Drawing {
        shapes: vec![
            triangle([(12.0, 24.0), (12.0, 76.0), (52.0, 50.0)]),
            triangle([(50.0, 24.0), (50.0, 76.0), (90.0, 50.0)]),
        ],
    }
}

pub fn play_icon() -> Drawing {
    Drawing {
        shapes: vec![triangle([(28.0, 16.0), (28.0, 84.0), (82.0, 50.0)])],
    }
}

pub fn pause_icon() -> Drawing {
    Drawing {
        shapes: vec![
            rounded(24.0, 18.0, 18.0, 64.0, 5.0, INK),
            rounded(58.0, 18.0, 18.0, 64.0, 5.0, INK),
        ],
    }
}

fn triangle(points: [(f32, f32); 3]) -> Shape {
    Shape::Triangle { points, color: INK }
}
