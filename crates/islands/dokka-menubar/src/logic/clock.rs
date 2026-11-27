//! Texto de la derecha: "jue 26 nov  18:39", en minúsculas.

/// Nombres cortos en español, sin tildes para la fuente del sistema.
const WEEKDAYS: [&str; 7] = ["dom", "lun", "mar", "mie", "jue", "vie", "sab"];
const MONTHS: [&str; 12] = [
    "ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic",
];

/// Fecha y hora de la barra. `weekday` sigue a `LocalTime`: 0 es domingo.
pub fn clock_text(
    weekday: u16,
    day: u16,
    month: u16,
    hour: u16,
    minute: u16,
) -> String {
    let weekday = WEEKDAYS.get(weekday as usize % 7).unwrap_or(&"?");
    let month = MONTHS
        .get(month.saturating_sub(1) as usize)
        .unwrap_or(&"?");
    format!("{weekday} {day} {month}  {:02}:{:02}", hour % 24, minute % 60)
}
