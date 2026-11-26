//! Si la barra se oculta sola al abrir Dokka.

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use dokka_contracts::ContractError;

/// Cómo se comporta la barra de tareas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarMode {
    Manual,
    Automatic,
}

/// Guarda la elección en un archivo de texto.
#[derive(Debug)]
pub struct TaskbarModeFile {
    path: PathBuf,
}

impl TaskbarModeFile {
    /// Usa la carpeta de datos del usuario.
    pub fn in_user_folder() -> Result<Self, ContractError> {
        let folder = std::env::var_os("APPDATA").ok_or(ContractError::NotFound)?;
        let path = PathBuf::from(folder).join("Dokka").join("taskbar_mode.txt");
        Ok(Self { path })
    }

    /// Lee la elección; si falta o no se entiende, es manual.
    pub fn load(&self) -> Result<TaskbarMode, ContractError> {
        let text = match fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(TaskbarMode::Manual),
            Err(error) => return Err(failed(&error)),
        };
        Ok(match text.trim() {
            "auto" => TaskbarMode::Automatic,
            _ => TaskbarMode::Manual,
        })
    }

    /// Guarda la elección.
    pub fn save(&self, mode: TaskbarMode) -> Result<(), ContractError> {
        if let Some(folder) = self.path.parent() {
            fs::create_dir_all(folder).map_err(|error| failed(&error))?;
        }
        let text = match mode {
            TaskbarMode::Manual => "manual\n",
            TaskbarMode::Automatic => "auto\n",
        };
        let draft = self.path.with_extension("tmp");
        fs::write(&draft, text).map_err(|error| failed(&error))?;
        fs::rename(&draft, &self.path).map_err(|error| failed(&error))
    }
}

fn failed(error: &std::io::Error) -> ContractError {
    ContractError::Failed(error.to_string())
}
