use dokka_model::WindowId;

use crate::ContractError;

/// Actúa sobre ventanas ya abiertas.
pub trait WindowControl {
    fn activate(&self, window: WindowId) -> Result<(), ContractError>;
    fn minimize(&self, window: WindowId) -> Result<(), ContractError>;
    /// Pide a la ventana que se cierre, como su propia X (`WM_CLOSE`).
    /// La app puede preguntar si guardar; no termina el proceso.
    fn close(&self, window: WindowId) -> Result<(), ContractError>;
    /// Deja la ventana por encima de las demás (`topmost`) o la devuelve a su orden.
    fn set_topmost(&self, window: WindowId, topmost: bool) -> Result<(), ContractError>;
    /// Lee de la propia ventana si está por encima de las demás.
    fn is_topmost(&self, window: WindowId) -> Result<bool, ContractError>;
    /// De estas ventanas, la que está más al frente. `None` si ninguna existe.
    fn front_window(&self, windows: &[WindowId]) -> Option<WindowId>;
}
