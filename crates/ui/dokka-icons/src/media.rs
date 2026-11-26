use dokka_scene::{Color, Drawing, Shape};

use crate::shapes::rounded;

/// Dos triángulos hacia la izquierda.
pub fn previous_icon(color: Color) -> Drawing {
    Drawing {
        shapes: vec![
            triangle([(50.0, 24.0), (50.0, 76.0), (10.0, 50.0)], color),
            triangle([(88.0, 24.0), (88.0, 76.0), (48.0, 50.0)], color),
        ],
    }
}

/// Dos triángulos hacia la derecha.
pub fn next_icon(color: Color) -> Drawing {
    Drawing {
        shapes: vec![
            triangle([(12.0, 24.0), (12.0, 76.0), (52.0, 50.0)], color),
            triangle([(50.0, 24.0), (50.0, 76.0), (90.0, 50.0)], color),
        ],
    }
}

pub fn play_icon(color: Color) -> Drawing {
    Drawing {
        shapes: vec![triangle([(28.0, 16.0), (28.0, 84.0), (82.0, 50.0)], color)],
    }
}

pub fn pause_icon(color: Color) -> Drawing {
    Drawing {
        shapes: vec![
            rounded(24.0, 18.0, 18.0, 64.0, 5.0, color),
            rounded(58.0, 18.0, 18.0, 64.0, 5.0, color),
        ],
    }
}

fn triangle(points: [(f32, f32); 3], color: Color) -> Shape {
    Shape::Triangle { points, color }
}
