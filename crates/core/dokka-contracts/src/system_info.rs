use dokka_model::{StorageBreakdown, SystemSummary};

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
/// El desglose de almacenamiento va aparte: `storage` devuelve el último
/// cálculo (`None` si aún no hay), `request_storage` lo calcula en su hilo
/// solo al abrir el apartado y `cancel_storage` lo detiene al salir.
pub trait SystemInfo {
    fn summary(&self) -> Option<SystemSummary>;
    fn request_refresh(&self);
    /// Último desglose de almacenamiento calculado, si hay.
    fn storage(&self) -> Option<StorageBreakdown>;
    /// Calcula el desglose en el hilo propio, sin bloquear.
    fn request_storage(&self);
    /// Detiene el cálculo en curso, si hay; conserva lo ya calculado.
    fn cancel_storage(&self);
    /// Abre la página del sistema indicada.
    fn open_page(&self, page: SystemPage) -> Result<(), ContractError>;
}
