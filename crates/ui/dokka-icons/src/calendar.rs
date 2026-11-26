use dokka_scene::{Color, Drawing, Shape};

use crate::shapes::{WHITE, tile};

const MONTH_COLOR: Color = Color::rgba(230, 60, 55, 255);
const DAY_COLOR: Color = Color::rgba(25, 25, 28, 255);
const MONTHS: [&str; 12] = [
    "ENE", "FEB", "MAR", "ABR", "MAY", "JUN", "JUL", "AGO", "SEP", "OCT", "NOV", "DIC",
];

/// Hoja de calendario con la fecha indicada (`month` de 1 a 12).
pub fn calendar_icon(month: u16, day: u16) -> Drawing {
    let index = usize::from(month.clamp(1, 12) - 1);
    Drawing {
        shapes: vec![
            tile(WHITE),
            text(MONTHS[index].to_owned(), 25.0, 21.0, true, MONTH_COLOR),
            text(day.to_string(), 63.0, 50.0, false, DAY_COLOR),
        ],
    }
}

fn text(text: String, y: f32, size: f32, is_bold: bool, color: Color) -> Shape {
    Shape::Text {
        text,
        x: 50.0,
        y,
        size,
        is_bold,
        color,
    }
}
