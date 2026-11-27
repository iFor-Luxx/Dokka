//! Texto de la derecha: solo la hora, "20:38". La fecha se ve en el calendario.

/// Hora de la barra. La fecha llega pero no se muestra.
pub fn clock_text(_weekday: u16, _day: u16, _month: u16, hour: u16, minute: u16) -> String {
    format!("{:02}:{:02}", hour % 24, minute % 60)
}
