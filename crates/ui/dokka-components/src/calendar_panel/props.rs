/// Datos de un mes ya calculados.
#[derive(Debug, Clone)]
pub struct CalendarMonth {
    pub title: String,
    /// Columna del día 1; 0 es domingo.
    pub first_column: u16,
    pub days: u16,
    pub today: Option<u16>,
}
