use dokka_scene::{Color, Drawing, Shape};

// El trazo tiene extremos planos: cada punta se redondea con un círculo
// del diámetro del trazo; el del codo redondea la unión.
const HEEL: (f32, f32) = (27.0, 53.0);
const ELBOW: (f32, f32) = (43.0, 69.0);
const TIP: (f32, f32) = (74.0, 29.0);
const WIDTH: f32 = 14.0;

/// Marca de la opción elegida, con el mismo trazo redondeado que el chevrón.
pub fn checkmark_icon(color: Color) -> Drawing {
    Drawing {
        shapes: vec![
            Shape::Line {
                from: HEEL,
                to: ELBOW,
                width: WIDTH,
                color,
            },
            Shape::Line {
                from: ELBOW,
                to: TIP,
                width: WIDTH,
                color,
            },
            Shape::Circle {
                x: HEEL.0,
                y: HEEL.1,
                radius: WIDTH / 2.0,
                color,
            },
            Shape::Circle {
                x: ELBOW.0,
                y: ELBOW.1,
                radius: WIDTH / 2.0,
                color,
            },
            Shape::Circle {
                x: TIP.0,
                y: TIP.1,
                radius: WIDTH / 2.0,
                color,
            },
        ],
    }
}
