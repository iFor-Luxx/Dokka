use std::path::PathBuf;

use crate::AppId;

/// Datos necesarios para mostrar y abrir una aplicación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInfo {
    pub id: AppId,
    pub name: String,
    pub executable: PathBuf,
}
