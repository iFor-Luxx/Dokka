use std::f32::consts::TAU;

use dokka_scene::{Color, Drawing, Shape};

use crate::shapes::{WHITE, tile};

const BACKGROUND: Color = Color::rgba(40, 40, 44, 255);
const INK: Color = Color::rgba(25, 25, 28, 255);
const SECOND_HAND: Color = Color::rgba(250, 140, 30, 255);
const CENTER: f32 = 50.0;

/// Reloj analógico con la hora indicada.
pub fn clock_icon(hour: u16, minute: u16, second: u16) -> Drawing {
    let mut shapes = vec![tile(BACKGROUND), circle(CENTER, CENTER, 42.0, WHITE)];
    shapes.extend((0..12).map(|mark| {
        let turn = f32::from(mark as u8) / 12.0;
        line(point(turn, 33.0), point(turn, 38.0), 2.5, INK)
    }));

    let minutes = f32::from(minute) / 60.0;
    let hours = (f32::from(hour % 12) + minutes) / 12.0;
    let seconds = f32::from(second) / 60.0;
    let middle = (CENTER, CENTER);
    shapes.push(line(middle, point(hours, 20.0), 5.0, INK));
    shapes.push(line(middle, point(minutes, 31.0), 3.5, INK));
    shapes.push(line(
        point(seconds, -8.0),
        point(seconds, 35.0),
        1.5,
        SECOND_HAND,
    ));
    shapes.push(circle(CENTER, CENTER, 3.0, SECOND_HAND));
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

fn circle(x: f32, y: f32, radius: f32, color: Color) -> Shape {
    Shape::Circle {
        x,
        y,
        radius,
        color,
    }
}
