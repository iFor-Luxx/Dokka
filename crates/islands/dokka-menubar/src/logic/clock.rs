//! Texto de la derecha: la hora con el formato de Dokka. La fecha se ve en
//! el calendario.

/// Hora de la barra. La fecha llega pero no se muestra.
pub fn clock_text(
    _weekday: u16,
    _day: u16,
    _month: u16,
    hour: u16,
    minute: u16,
    hour_24: bool,
) -> String {
    if hour_24 {
        format!("{:02}:{:02}", hour % 24, minute % 60)
    } else {
        let hour = hour % 24;
        let (hour, period) = match hour {
            0 => (12, "a. m."),
            1..=11 => (hour, "a. m."),
            12 => (12, "p. m."),
            _ => (hour - 12, "p. m."),
        };
        format!("{hour}:{:02} {period}", minute % 60)
    }
}
