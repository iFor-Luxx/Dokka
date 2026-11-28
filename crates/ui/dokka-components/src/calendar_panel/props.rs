/// Datos de un mes ya calculados.
#[derive(Debug, Clone)]
pub struct CalendarMonth {
    pub title: String,
    /// Columna del día 1; 0 es domingo, o lunes con `monday_first`.
    pub first_column: u16,
    pub days: u16,
    /// Días que trae el mes anterior, para dibujar sus últimos días
    /// en las celdas antes del día 1.
    pub prev_days: u16,
    pub today: Option<u16>,
    /// La semana empieza en lunes; si no, en domingo.
    pub monday_first: bool,
}
