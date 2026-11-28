use dokka_scene::{Color, Dot, Overlay, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, popup as measures, typography};

use super::CalendarMonth;
use crate::nav_pill::{
    NavPillIcons, NavPillProps, nav_pill_divider, nav_pill_highlight, nav_pill_icons,
    nav_pill_panel,
};
use crate::popup_panel::popup_panel;

const WEEKDAYS_SUNDAY: [&str; 7] = ["Do", "Lu", "Ma", "Mi", "Ju", "Vi", "Sá"];
const WEEKDAYS_MONDAY: [&str; 7] = ["Lu", "Ma", "Mi", "Ju", "Vi", "Sá", "Do"];
/// El calendario mide siempre 6 filas de semanas, como macOS.
const WEEKS: u16 = 6;

pub fn calendar_size() -> Size {
    let rows = 1.0 + f32::from(WEEKS);
    Size {
        width: measures::CALENDAR_CELL_WIDTH * 7.0 + measures::PADDING * 2.0,
        height: measures::CALENDAR_HEADER
            + measures::CALENDAR_CELL_HEIGHT * rows
            + measures::PADDING,
    }
}

/// Cápsula de navegación, arriba a la derecha de la cabecera.
pub fn calendar_nav_pill(bounds: Rect) -> Rect {
    Rect {
        x: bounds.x + bounds.width - measures::PADDING - measures::CALENDAR_NAV_PILL_WIDTH,
        y: bounds.y
            + (measures::CALENDAR_HEADER - measures::CALENDAR_NAV_PILL_HEIGHT) / 2.0
            + measures::CALENDAR_HEADER_DROP,
        width: measures::CALENDAR_NAV_PILL_WIDTH,
        height: measures::CALENDAR_NAV_PILL_HEIGHT,
    }
}

/// Mes con su píldora de navegación, igual en el dock y en la barra.
pub fn calendar_panel(
    theme: &Theme,
    drawings: &NavPillIcons,
    bounds: Rect,
    month: &CalendarMonth,
    pill: &NavPillProps,
) -> Overlay {
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
    let mut texts = vec![centered(
        month.title.clone(),
        (title_x, header_y),
        typography::TITLE_SIZE,
        true,
        theme.panel.text,
    )];
    // Columna del día 1 en la semana que se muestra: con lunes primero el
    // domingo (6) cae en la última columna.
    let leading: u16 = if month.monday_first {
        (month.first_column + 6) % 7
    } else {
        month.first_column
    };
    let weekdays = if month.monday_first {
        WEEKDAYS_MONDAY
    } else {
        WEEKDAYS_SUNDAY
    };
    texts.extend((0..7).zip(weekdays).map(|(column, name)| {
        let center = (column_x(column), row_y(0));
        centered(
            name.to_owned(),
            center,
            typography::BODY_SIZE,
            true,
            theme.panel.text,
        )
    }));

    let mut dots = Vec::new();
    // Vecinos del mes anterior: últimos días antes del día 1, atenuados.
    for cell in 0..leading {
        let day = month.prev_days - leading + 1 + cell;
        let (x, y) = (column_x(cell % 7), row_y(cell / 7 + 1));
        texts.push(centered(
            day.to_string(),
            (x, y),
            typography::BODY_SIZE,
            false,
            theme.panel.adjacent_text,
        ));
    }
    for day in 1..=month.days {
        let cell = leading + day - 1;
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
        texts.push(centered(
            day.to_string(),
            (x, y),
            typography::BODY_SIZE,
            false,
            theme.panel.text,
        ));
    }
    // Vecinos del mes siguiente: primeros días hasta completar 6 filas.
    let used = u32::from(leading) + u32::from(month.days);
    let trailing = 7 * u32::from(WEEKS) - used;
    for next in 0..trailing {
        let cell = (used + next) as u16;
        let (x, y) = (column_x(cell % 7), row_y(cell / 7 + 1));
        texts.push(centered(
            (next + 1).to_string(),
            (x, y),
            typography::BODY_SIZE,
            false,
            theme.panel.adjacent_text,
        ));
    }

    Overlay {
        panel: popup_panel(theme, bounds),
        tiles: Vec::new(),
        panel_shadow: None,
        tile_shadow: None,
        pill: Some(nav_pill_panel(theme, pill)),
        highlight: Some(nav_pill_highlight(theme, pill)),
        flat_panels: Vec::new(),
        dividers: vec![nav_pill_divider(theme, pill)],
        cursors: Vec::new(),
        dots,
        pointer: None,
        texts,
        icons: nav_pill_icons(theme, drawings, pill),
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

fn centered(content: String, center: (f32, f32), size: f32, is_bold: bool, color: Color) -> Text {
    Text {
        x: center.0,
        center_y: center.1,
        content,
        size,
        is_bold,
        color,
        align: TextAlign::Center,
    }
}
