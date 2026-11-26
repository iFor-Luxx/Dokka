/// Etiqueta del reloj: hora en 12 horas, como "3:05 p. m.". Sin segundos.
pub(crate) fn clock_label(hour: u16, minute: u16) -> String {
    let hour = hour % 24;
    let (hour, period) = match hour {
        0 => (12, "a. m."),
        1..=11 => (hour, "a. m."),
        12 => (12, "p. m."),
        _ => (hour - 12, "p. m."),
    };
    format!("{hour}:{:02} {period}", minute % 60)
}
