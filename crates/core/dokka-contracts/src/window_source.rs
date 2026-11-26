use dokka_model::OpenWindow;

use crate::ContractError;

/// Informa qué ventanas están abiertas.
pub trait WindowSource {
    fn open_windows(&self) -> Result<Vec<OpenWindow>, ContractError>;
}
