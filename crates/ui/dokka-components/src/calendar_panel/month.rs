use super::CalendarMonth;

const MONTHS: [&str; 12] = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
];

/// Mes situado a `offset` meses del de `today` (año, mes, día).
pub fn calendar_month(today: (u16, u16, u16), offset: i32) -> CalendarMonth {
    let months = i32::from(today.0) * 12 + i32::from(today.1.max(1)) - 1 + offset;
    let year = months.div_euclid(12);
    let month = months.rem_euclid(12) as usize + 1;
    CalendarMonth {
        title: format!("{} {year}", MONTHS[month - 1]),
        first_column: weekday_of_first(year, month),
        days: days_in(year, month),
        today: (offset == 0).then_some(today.2),
    }
}

fn days_in(year: i32, month: usize) -> u16 {
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    match month {
        2 if is_leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Día de la semana del día 1 (método de Sakamoto); 0 es domingo.
fn weekday_of_first(year: i32, month: usize) -> u16 {
    const SHIFTS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 3 { year - 1 } else { year };
    (year + year / 4 - year / 100 + year / 400 + SHIFTS[month - 1] + 1).rem_euclid(7) as u16
}
