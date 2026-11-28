use dokka_model::SystemSummary;

use crate::ContractError;

/// Página del sistema que se abre desde "Acerca de este PC" o desde
/// "Fecha y hora" e "Idioma y región".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPage {
    /// `ms-settings:display`.
    Display,
    /// `ms-settings:storagesense`.
    Storage,
    /// `msinfo32`.
    Report,
    /// `ms-settings:dateandtime`.
    DateTime,
    /// `ms-settings:regionformatting`.
    Region,
}

/// Resumen del equipo sin bloquear, como `BrightnessControl`.
///
/// `summary` devuelve el último valor conocido (`None` si aún no se
/// leyó nada). `request_refresh` pide releer en el hilo propio.
pub trait SystemInfo {
    fn summary(&self) -> Option<SystemSummary>;
    fn request_refresh(&self);
    /// Abre la página del sistema indicada.
    fn open_page(&self, page: SystemPage) -> Result<(), ContractError>;
}
