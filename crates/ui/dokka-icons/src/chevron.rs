use dokka_scene::{Color, Drawing, Shape};

// El trazo tiene extremos planos: cada punta se redondea con un círculo
// del diámetro del trazo; el del vértice redondea la unión.
const TOP: (f32, f32) = (34.0, 26.0);
const TIP: (f32, f32) = (66.0, 50.0);
const BOTTOM: (f32, f32) = (34.0, 74.0);
const WIDTH: f32 = 14.0;

/// Chevrón derecho estilo macOS que marca las filas con submenú.
pub fn chevron_icon(color: Color) -> Drawing {
    chevron_drawing(TOP, TIP, BOTTOM, color)
}

/// Chevrón izquierdo con el mismo trazo: flecha del mes anterior.
pub fn chevron_left_icon(color: Color) -> Drawing {
    const LEFT_TOP: (f32, f32) = (66.0, 26.0);
    const LEFT_TIP: (f32, f32) = (34.0, 50.0);
    const LEFT_BOTTOM: (f32, f32) = (66.0, 74.0);
    chevron_drawing(LEFT_TOP, LEFT_TIP, LEFT_BOTTOM, color)
}

fn chevron_drawing(top: (f32, f32), tip: (f32, f32), bottom: (f32, f32), color: Color) -> Drawing {
    Drawing {
        shapes: vec![
            Shape::Line {
                from: top,
                to: tip,
                width: WIDTH,
                color,
            },
            Shape::Line {
                from: tip,
                to: bottom,
                width: WIDTH,
                color,
            },
            Shape::Circle {
                x: top.0,
                y: top.1,
                radius: WIDTH / 2.0,
                color,
            },
            Shape::Circle {
                x: tip.0,
                y: tip.1,
                radius: WIDTH / 2.0,
                color,
            },
            Shape::Circle {
                x: bottom.0,
                y: bottom.1,
                radius: WIDTH / 2.0,
                color,
            },
        ],
    }
}
