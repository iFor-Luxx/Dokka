use dokka_model::WindowId;

use crate::ContractError;

/// Actúa sobre ventanas ya abiertas.
pub trait WindowControl {
    fn activate(&self, window: WindowId) -> Result<(), ContractError>;
    fn minimize(&self, window: WindowId) -> Result<(), ContractError>;
}
