use dokka_scene::{Dot, Overlay, Rect, Size};
use dokka_theme::{Theme, popup as measures, typography};

use super::CalendarMonth;
use crate::popup_panel::{popup_panel, text};

const WEEKDAYS: [&str; 7] = ["Do", "Lu", "Ma", "Mi", "Ju", "Vi", "Sá"];
/// Fila de nombres de día más seis semanas.
const ROWS: f32 = 7.0;

pub fn calendar_size() -> Size {
    Size {
        width: measures::CALENDAR_CELL_WIDTH * 7.0 + measures::PADDING * 2.0,
        height: measures::CALENDAR_HEADER
            + measures::CALENDAR_CELL_HEIGHT * ROWS
            + measures::PADDING,
    }
}

/// Zonas pulsables para el mes anterior y el siguiente.
pub fn calendar_arrows(bounds: Rect) -> (Rect, Rect) {
    let side = measures::CALENDAR_ARROW_SIDE;
    let arrow = |from_right: f32| Rect {
        x: bounds.x + bounds.width - measures::PADDING - from_right,
        y: bounds.y + (measures::CALENDAR_HEADER - side) / 2.0 + measures::CALENDAR_HEADER_DROP,
        width: side,
        height: side,
    };
    (arrow(side * 2.0), arrow(side))
}

pub fn calendar_panel(theme: &Theme, bounds: Rect, month: &CalendarMonth) -> Overlay {
    let column_x = |column: u16| {
        bounds.x + measures::PADDING + (f32::from(column) + 0.5) * measures::CALENDAR_CELL_WIDTH
    };
    let row_y = |row: u16| {
        bounds.y
            + measures::CALENDAR_HEADER
            + (f32::from(row) + 0.5) * measures::CALENDAR_CELL_HEIGHT
    };

    let header_y = bounds.y + measures::CALENDAR_HEADER / 2.0 + measures::CALENDAR_HEADER_DROP;
    let title_x = bounds.x + measures::PADDING + measures::CALENDAR_CELL_WIDTH * 2.5;
    let mut texts = vec![text(
        theme,
        month.title.clone(),
        (title_x, header_y),
        typography::TITLE_SIZE,
        true,
    )];
    let (previous, next) = calendar_arrows(bounds);
    for (symbol, area) in [("‹", previous), ("›", next)] {
        let center = (
            area.x + area.width / 2.0,
            header_y - measures::CALENDAR_HEADER_DROP,
        );
        let size = typography::CALENDAR_ARROW_SIZE;
        texts.push(text(theme, symbol.to_owned(), center, size, false));
    }
    texts.extend((0..7).zip(WEEKDAYS).map(|(column, name)| {
        let center = (column_x(column), row_y(0));
        text(theme, name.to_owned(), center, typography::BODY_SIZE, true)
    }));

    let mut dots = Vec::new();
    for day in 1..=month.days {
        let cell = month.first_column + day - 1;
        let (x, y) = (column_x(cell % 7), row_y(cell / 7 + 1));
        if month.today == Some(day) {
            let diameter = measures::CALENDAR_CELL_HEIGHT - measures::CALENDAR_TODAY_INSET;
            dots.push(Dot {
                x: x - diameter / 2.0,
                y: y - diameter / 2.0,
                diameter,
                color: theme.panel.highlight,
            });
        }
        let size = typography::BODY_SIZE;
        texts.push(text(theme, day.to_string(), (x, y), size, false));
    }

    Overlay {
        panel: popup_panel(theme, bounds),
        highlight: None,
        dividers: Vec::new(),
        dots,
        pointer: None,
        texts,
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
    }
}
