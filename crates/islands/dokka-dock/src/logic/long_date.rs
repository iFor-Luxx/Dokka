const WEEKDAYS: [&str; 7] = [
    "domingo",
    "lunes",
    "martes",
    "miércoles",
    "jueves",
    "viernes",
    "sábado",
];
const MONTHS: [&str; 12] = [
    "enero",
    "febrero",
    "marzo",
    "abril",
    "mayo",
    "junio",
    "julio",
    "agosto",
    "septiembre",
    "octubre",
    "noviembre",
    "diciembre",
];

/// Fecha escrita, como "jueves, 26 de noviembre de 2026". `weekday` 0 es domingo.
pub(crate) fn long_date(year: u16, month: u16, day: u16, weekday: u16) -> String {
    let weekday = WEEKDAYS[usize::from(weekday % 7)];
    let month = MONTHS[usize::from(month.clamp(1, 12) - 1)];
    format!("{weekday}, {day} de {month} de {year}")
}
