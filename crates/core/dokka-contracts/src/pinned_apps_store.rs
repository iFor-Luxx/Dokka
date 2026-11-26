use dokka_model::AppInfo;

use crate::ContractError;

/// Conserva las apps fijadas entre sesiones, en orden.
pub trait PinnedAppsStore {
    fn load(&self) -> Result<Vec<AppInfo>, ContractError>;
    fn save(&self, apps: &[AppInfo]) -> Result<(), ContractError>;
}
