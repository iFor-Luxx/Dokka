use dokka_model::AppInfo;

use crate::ContractError;

/// Acciones rápidas sobre una aplicación, aparte de abrirla.
pub trait AppActions {
    /// Abre la app pidiendo permisos de administrador; Windows pregunta antes.
    /// Que el usuario rechace la pregunta no es un error.
    fn run_as_administrator(&self, app: &AppInfo) -> Result<(), ContractError>;
    /// Muestra el archivo de la app seleccionado en su carpeta.
    fn show_in_folder(&self, app: &AppInfo) -> Result<(), ContractError>;
}
