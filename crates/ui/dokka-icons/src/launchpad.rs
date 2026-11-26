use dokka_scene::{Color, Drawing};

use crate::shapes::{rounded, tile};

const BACKGROUND: Color = Color::rgba(226, 229, 234, 255);
const SQUARES: [Color; 9] = [
    Color::rgba(76, 201, 90, 255),
    Color::rgba(250, 200, 40, 255),
    Color::rgba(250, 140, 40, 255),
    Color::rgba(235, 70, 70, 255),
    Color::rgba(140, 145, 155, 255),
    Color::rgba(240, 80, 130, 255),
    Color::rgba(160, 90, 220, 255),
    Color::rgba(50, 150, 240, 255),
    Color::rgba(40, 200, 200, 255),
];

/// Cuadrícula de colores.
pub fn launchpad_icon() -> Drawing {
    let mut shapes = vec![tile(BACKGROUND)];
    shapes.extend(SQUARES.iter().enumerate().map(|(index, &color)| {
        let column = (index % 3) as f32;
        let row = (index / 3) as f32;
        rounded(
            20.0 + column * 22.0,
            20.0 + row * 22.0,
            16.0,
            16.0,
            5.0,
            color,
        )
    }));
    Drawing { shapes }
}
