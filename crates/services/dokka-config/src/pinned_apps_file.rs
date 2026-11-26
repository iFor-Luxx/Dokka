use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use dokka_contracts::{ContractError, PinnedAppsStore};
use dokka_model::{AppId, AppInfo};

/// Lista de apps fijadas en un archivo de texto.
#[derive(Debug)]
pub struct PinnedAppsFile {
    path: PathBuf,
}

impl PinnedAppsFile {
    /// Usa la carpeta de datos del usuario.
    pub fn in_user_folder() -> Result<Self, ContractError> {
        let folder = std::env::var_os("APPDATA").ok_or(ContractError::NotFound)?;
        let path = PathBuf::from(folder).join("Dokka").join("pinned_apps.txt");
        Ok(Self { path })
    }
}

impl PinnedAppsStore for PinnedAppsFile {
    fn load(&self) -> Result<Vec<AppInfo>, ContractError> {
        let text = match fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(failed(&error)),
        };
        // Las líneas mal escritas se ignoran para no perder el resto.
        Ok(text.lines().filter_map(parse_line).collect())
    }

    fn save(&self, apps: &[AppInfo]) -> Result<(), ContractError> {
        if let Some(folder) = self.path.parent() {
            fs::create_dir_all(folder).map_err(|error| failed(&error))?;
        }
        let text: String = apps
            .iter()
            .map(|app| format!("{}\t{}\n", app.name, app.executable.display()))
            .collect();
        // Se escribe aparte y se renombra para no dejar un archivo a medias.
        let draft = self.path.with_extension("tmp");
        fs::write(&draft, text).map_err(|error| failed(&error))?;
        fs::rename(&draft, &self.path).map_err(|error| failed(&error))
    }
}

fn parse_line(line: &str) -> Option<AppInfo> {
    let (name, path) = line.split_once('\t')?;
    let executable = PathBuf::from(path);
    Some(AppInfo {
        id: AppId::from_executable(&executable),
        name: name.to_owned(),
        executable,
    })
}

fn failed(error: &std::io::Error) -> ContractError {
    ContractError::Failed(error.to_string())
}
