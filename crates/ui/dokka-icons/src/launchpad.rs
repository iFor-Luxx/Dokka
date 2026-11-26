use dokka_scene::Drawing;
use dokka_theme::IconColors;

use crate::shapes::{rounded, tile};

/// Cuadrícula de colores.
pub fn launchpad_icon(colors: &IconColors) -> Drawing {
    let mut shapes = vec![tile(colors.launchpad_background)];
    let squares = colors.launchpad_squares.iter().enumerate();
    shapes.extend(squares.map(|(index, &color)| {
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
