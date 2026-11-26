use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::IconColors;

use crate::shapes::tile;

const MONTHS: [&str; 12] = [
    "ENE", "FEB", "MAR", "ABR", "MAY", "JUN", "JUL", "AGO", "SEP", "OCT", "NOV", "DIC",
];

/// Hoja de calendario con la fecha indicada (`month` de 1 a 12).
pub fn calendar_icon(colors: &IconColors, month: u16, day: u16) -> Drawing {
    let index = usize::from(month.clamp(1, 12) - 1);
    let month_name = MONTHS[index].to_owned();
    Drawing {
        shapes: vec![
            tile(colors.calendar_background),
            text(month_name, 25.0, 21.0, true, colors.calendar_month),
            text(day.to_string(), 63.0, 50.0, false, colors.calendar_day),
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
