use dokka_model::{TrayItem, TrayItemId};

use crate::ContractError;

/// Iconos de las apps en segundo plano.
pub trait SystemTray {
    /// Iconos visibles ahora, en el orden en que aparecieron.
    fn items(&self) -> Vec<TrayItem>;
    /// Clic izquierdo: la app hace lo que haría en la bandeja de Windows.
    fn activate(&self, id: TrayItemId) -> Result<(), ContractError>;
    /// Clic derecho: la app abre su propio menú junto al cursor.
    fn open_menu(&self, id: TrayItemId) -> Result<(), ContractError>;
}
