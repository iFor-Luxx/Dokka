use dokka_scene::{Color, Dot, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, popup as measures, typography};

use super::CalendarMonth;
use crate::nav_pill::{
    NavPillIcons, NavPillProps, nav_pill_divider, nav_pill_highlight, nav_pill_icons,
    nav_pill_panel,
};
use crate::popup_panel::popup_panel;

const WEEKDAYS: [&str; 7] = ["Do", "Lu", "Ma", "Mi", "Ju", "Vi", "Sá"];
/// Semanas mínimas: los meses de 4 o 5 semanas miden igual y las flechas
/// no saltan al navegar; solo los de 6 semanas crecen una fila.
const MINIMUM_WEEKS: u16 = 5;

pub fn calendar_weeks(month: &CalendarMonth) -> u16 {
    let used = u32::from(month.first_column) + u32::from(month.days);
    (used.div_ceil(7) as u16).max(MINIMUM_WEEKS)
}

pub fn calendar_size(weeks: u16) -> Size {
    let rows = 1.0 + f32::from(weeks);
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

/// Mes con su píldora. En la barra (`dark`) usa el cristal oscuro con
/// texto blanco y el hoy en círculo blanco con número oscuro; en el
/// dock sigue claro. La píldora clara se lee sobre ambos.
pub fn calendar_panel(
    theme: &Theme,
    drawings: &NavPillIcons,
    bounds: Rect,
    month: &CalendarMonth,
    pill: &NavPillProps,
    dark: bool,
) -> Overlay {
    let column_x = |column: u16| {
        bounds.x + measures::PADDING + (f32::from(column) + 0.5) * measures::CALENDAR_CELL_WIDTH
    };
    let row_y = |row: u16| {
        bounds.y
            + measures::CALENDAR_HEADER
            + (f32::from(row) + 0.5) * measures::CALENDAR_CELL_HEIGHT
    };

    let main = if dark { theme.tray.text } else { theme.panel.text };
    let faint = if dark {
        theme.tray.secondary
    } else {
        theme.panel.text
    };
    let today_bg = if dark {
        theme.tray.fill
    } else {
        theme.panel.highlight
    };
    let today_ink = if dark {
        theme.tray.percent
    } else {
        theme.panel.text
    };
    let header_y = bounds.y + measures::CALENDAR_HEADER / 2.0 + measures::CALENDAR_HEADER_DROP;
    let title_x = bounds.x + measures::PADDING + measures::CALENDAR_CELL_WIDTH * 2.5;
    let mut texts = vec![centered(
        month.title.clone(),
        (title_x, header_y),
        typography::TITLE_SIZE,
        true,
        main,
    )];
    texts.extend((0..7).zip(WEEKDAYS).map(|(column, name)| {
        let center = (column_x(column), row_y(0));
        centered(name.to_owned(), center, typography::BODY_SIZE, true, faint)
    }));

    let mut dots = Vec::new();
    for day in 1..=month.days {
        let cell = month.first_column + day - 1;
        let (x, y) = (column_x(cell % 7), row_y(cell / 7 + 1));
        let is_today = month.today == Some(day);
        if is_today {
            let diameter = measures::CALENDAR_CELL_HEIGHT - measures::CALENDAR_TODAY_INSET;
            dots.push(Dot {
                x: x - diameter / 2.0,
                y: y - diameter / 2.0,
                diameter,
                color: today_bg,
            });
        }
        texts.push(centered(
            day.to_string(),
            (x, y),
            typography::BODY_SIZE,
            false,
            if is_today { today_ink } else { main },
        ));
    }

    Overlay {
        panel: glass(theme, bounds, dark),
        tiles: Vec::new(),
        pill: Some(nav_pill_panel(theme, pill)),
        highlight: Some(nav_pill_highlight(theme, pill)),
        flat_panels: Vec::new(),
        dividers: vec![nav_pill_divider(theme, pill)],
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

fn glass(theme: &Theme, bounds: Rect, dark: bool) -> Panel {
    if dark {
        Panel {
            bounds,
            corner_radius: capped(bounds, dokka_theme::tray_glass::PANEL_RADIUS),
            border_width: dokka_theme::tray_glass::PANEL_BORDER_WIDTH,
            tint: theme.tray.panel,
            border: theme.tray.panel_border,
        }
    } else {
        popup_panel(theme, bounds)
    }
}

fn capped(bounds: Rect, maximum: f32) -> f32 {
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return 0.0;
    }
    maximum
        .min(bounds.width / 2.0)
        .min(bounds.height / 2.0)
        .max(0.0)
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
