use dokka_contracts::{AppActions, ContractError};
use dokka_model::AppInfo;

use crate::run_as_administrator::run_as_administrator;
use crate::show_in_folder::show_in_folder;

/// Acciones rápidas sobre una aplicación, aparte de abrirla.
#[derive(Debug, Default)]
pub struct SystemAppActions;

impl AppActions for SystemAppActions {
    fn run_as_administrator(&self, app: &AppInfo) -> Result<(), ContractError> {
        run_as_administrator(&app.executable, &app.name)
    }

    fn show_in_folder(&self, app: &AppInfo) -> Result<(), ContractError> {
        show_in_folder(&app.executable, &app.name)
    }
}
