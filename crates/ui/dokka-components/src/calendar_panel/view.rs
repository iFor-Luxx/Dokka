use dokka_scene::{Dot, Overlay, Rect, Size};
use dokka_theme::popup as theme;

use super::CalendarMonth;
use crate::popup_panel::{popup_panel, text};

const WEEKDAYS: [&str; 7] = ["Do", "Lu", "Ma", "Mi", "Ju", "Vi", "Sá"];
/// Fila de nombres de día más seis semanas.
const ROWS: f32 = 7.0;
const ARROW_SIDE: f32 = 28.0;

pub fn calendar_size() -> Size {
    Size {
        width: theme::CALENDAR_CELL_WIDTH * 7.0 + theme::PADDING * 2.0,
        height: theme::CALENDAR_HEADER + theme::CALENDAR_CELL_HEIGHT * ROWS + theme::PADDING,
    }
}

/// Zonas pulsables para el mes anterior y el siguiente.
pub fn calendar_arrows(bounds: Rect) -> (Rect, Rect) {
    let arrow = |from_right: f32| Rect {
        x: bounds.x + bounds.width - theme::PADDING - from_right,
        y: bounds.y + (theme::CALENDAR_HEADER - ARROW_SIDE) / 2.0 + 2.0,
        width: ARROW_SIDE,
        height: ARROW_SIDE,
    };
    (arrow(ARROW_SIDE * 2.0), arrow(ARROW_SIDE))
}

pub fn calendar_panel(bounds: Rect, month: &CalendarMonth) -> Overlay {
    let column_x = |column: u16| {
        bounds.x + theme::PADDING + (f32::from(column) + 0.5) * theme::CALENDAR_CELL_WIDTH
    };
    let row_y = |row: u16| {
        bounds.y + theme::CALENDAR_HEADER + (f32::from(row) + 0.5) * theme::CALENDAR_CELL_HEIGHT
    };

    let header_y = bounds.y + theme::CALENDAR_HEADER / 2.0 + 2.0;
    let title_x = bounds.x + theme::PADDING + theme::CALENDAR_CELL_WIDTH * 2.5;
    let mut texts = vec![text(
        month.title.clone(),
        title_x,
        header_y,
        theme::TITLE_SIZE,
        true,
    )];
    let (previous, next) = calendar_arrows(bounds);
    for (symbol, area) in [("‹", previous), ("›", next)] {
        let x = area.x + area.width / 2.0;
        texts.push(text(symbol.to_owned(), x, header_y - 2.0, 22.0, false));
    }
    texts.extend((0..7).zip(WEEKDAYS).map(|(column, name)| {
        text(
            name.to_owned(),
            column_x(column),
            row_y(0),
            theme::BODY_SIZE,
            true,
        )
    }));

    let mut dots = Vec::new();
    for day in 1..=month.days {
        let cell = month.first_column + day - 1;
        let (x, y) = (column_x(cell % 7), row_y(cell / 7 + 1));
        if month.today == Some(day) {
            let diameter = theme::CALENDAR_CELL_HEIGHT - 3.0;
            dots.push(Dot {
                x: x - diameter / 2.0,
                y: y - diameter / 2.0,
                diameter,
                color: theme::HIGHLIGHT,
            });
        }
        texts.push(text(day.to_string(), x, y, theme::BODY_SIZE, false));
    }

    Overlay {
        panel: popup_panel(bounds),
        dots,
        texts,
        thumbnails: Vec::new(),
    }
}
