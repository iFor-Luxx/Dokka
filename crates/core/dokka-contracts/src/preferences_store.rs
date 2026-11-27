use dokka_model::Preferences;

use crate::ContractError;

/// Guarda lo ajustado en Preferencias.
pub trait PreferencesStore {
    /// Sin archivo todavía devuelve los valores de fábrica.
    fn load(&self) -> Result<Preferences, ContractError>;
    fn save(&self, preferences: &Preferences) -> Result<(), ContractError>;
}
