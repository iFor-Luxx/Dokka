use dokka_scene::{Color, Drawing, Shape};

// El trazo tiene extremos planos: cada punta se redondea con un círculo
// del diámetro del trazo; el del vértice redondea la unión.
const TOP: (f32, f32) = (34.0, 26.0);
const TIP: (f32, f32) = (66.0, 50.0);
const BOTTOM: (f32, f32) = (34.0, 74.0);
const WIDTH: f32 = 14.0;

/// Chevrón estilo macOS que marca las filas con submenú.
pub fn chevron_icon(color: Color) -> Drawing {
    Drawing {
        shapes: vec![
            Shape::Line {
                from: TOP,
                to: TIP,
                width: WIDTH,
                color,
            },
            Shape::Line {
                from: TIP,
                to: BOTTOM,
                width: WIDTH,
                color,
            },
            Shape::Circle {
                x: TOP.0,
                y: TOP.1,
                radius: WIDTH / 2.0,
                color,
            },
            Shape::Circle {
                x: TIP.0,
                y: TIP.1,
                radius: WIDTH / 2.0,
                color,
            },
            Shape::Circle {
                x: BOTTOM.0,
                y: BOTTOM.1,
                radius: WIDTH / 2.0,
                color,
            },
        ],
    }
}
