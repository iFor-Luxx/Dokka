use dokka_model::AppInfo;

use crate::ContractError;

/// Aplicaciones instaladas que se muestran en el Launchpad.
pub trait AppCatalog {
    fn installed_apps(&self) -> Result<Vec<AppInfo>, ContractError>;
}
