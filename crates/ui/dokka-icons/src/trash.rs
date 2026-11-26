use dokka_scene::{Color, Drawing, Shape};

use crate::shapes::rounded;

const BODY: Color = Color::rgba(222, 226, 232, 235);
const RIM: Color = Color::rgba(160, 166, 176, 255);
const RIB: Color = Color::rgba(150, 156, 168, 200);
const PAPER: Color = Color::rgba(250, 250, 250, 255);
const WASTE: Color = Color::rgba(90, 160, 235, 255);

/// Papelera, vacía o con contenido asomando.
pub fn trash_icon(has_items: bool) -> Drawing {
    let mut shapes = Vec::new();
    if has_items {
        shapes.push(paper(34.0, 22.0, 13.0, PAPER));
        shapes.push(paper(52.0, 18.0, 15.0, WASTE));
        shapes.push(paper(66.0, 24.0, 11.0, PAPER));
    }
    shapes.push(rounded(22.0, 26.0, 56.0, 68.0, 10.0, BODY));
    shapes.push(rounded(18.0, 23.0, 64.0, 9.0, 4.0, RIM));
    shapes.extend([36.0, 50.0, 64.0].map(|x| rounded(x - 2.0, 42.0, 4.0, 40.0, 2.0, RIB)));
    Drawing { shapes }
}

fn paper(x: f32, y: f32, radius: f32, color: Color) -> Shape {
    Shape::Circle {
        x,
        y,
        radius,
        color,
    }
}
