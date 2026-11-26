use std::f32::consts::TAU;

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::IconColors;

use crate::shapes::{circle, tile};

const CENTER: f32 = 50.0;

/// Reloj analógico con la hora indicada.
pub fn clock_icon(colors: &IconColors, hour: u16, minute: u16, second: u16) -> Drawing {
    let mut shapes = vec![
        tile(colors.clock_background),
        circle(CENTER, CENTER, 42.0, colors.clock_face),
    ];
    shapes.extend((0..12).map(|mark| {
        let turn = f32::from(mark as u8) / 12.0;
        line(point(turn, 33.0), point(turn, 38.0), 2.5, colors.clock_ink)
    }));

    let minutes = f32::from(minute) / 60.0;
    let hours = (f32::from(hour % 12) + minutes) / 12.0;
    let seconds = f32::from(second) / 60.0;
    let middle = (CENTER, CENTER);
    shapes.push(line(middle, point(hours, 20.0), 5.0, colors.clock_ink));
    shapes.push(line(middle, point(minutes, 31.0), 3.5, colors.clock_ink));
    let hand = colors.clock_second_hand;
    shapes.push(line(point(seconds, -8.0), point(seconds, 35.0), 1.5, hand));
    shapes.push(circle(CENTER, CENTER, 3.0, hand));
    Drawing { shapes }
}

/// Punto a `length` del centro; `turn` es la fracción de vuelta desde las doce.
fn point(turn: f32, length: f32) -> (f32, f32) {
    let angle = turn * TAU;
    (CENTER + angle.sin() * length, CENTER - angle.cos() * length)
}

fn line(from: (f32, f32), to: (f32, f32), width: f32, color: Color) -> Shape {
    Shape::Line {
        from,
        to,
        width,
        color,
    }
}
