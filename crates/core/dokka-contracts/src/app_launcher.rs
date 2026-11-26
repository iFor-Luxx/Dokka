use dokka_model::AppInfo;

use crate::ContractError;

/// Abre aplicaciones.
pub trait AppLauncher {
    fn launch(&self, app: &AppInfo) -> Result<(), ContractError>;
}
