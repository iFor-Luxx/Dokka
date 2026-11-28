//! Escritura de `Dokka.exe` y `Desinstalar.exe` conservando lo anterior.
//!
//! Cada archivo se escribe en un temporal y se renombra al final; si ya
//! había una versión, se aparta como `.bak` antes del renombre. El diario de
//! `installer` confirma (borra el respaldo) o deshace (quita lo nuevo y
//! devuelve el respaldo) según salga la instalación.

use std::path::{Path, PathBuf};

use dokka_contracts::InstallFailure;

use crate::install_error::from_io;

/// Un archivo ya colocado, pendiente de confirmar o deshacer.
pub(crate) struct StagedFile {
    target: PathBuf,
    staging: PathBuf,
    backup: PathBuf,
    had_previous: bool,
}

/// Coloca `bytes` en `target` con temporal y respaldo de la versión anterior.
pub(crate) fn stage_bytes(target: &Path, bytes: &[u8]) -> Result<StagedFile, InstallFailure> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| from_io("no se pudo crear la carpeta de Dokka", &error))?;
    }
    let staging = with_suffix(target, ".new");
    let backup = with_suffix(target, ".bak");
    if let Err(error) = std::fs::write(&staging, bytes) {
        let _ = std::fs::remove_file(&staging);
        return Err(from_io("no se pudo escribir el archivo temporal", &error));
    }
    let had_previous = target.exists();
    if had_previous && let Err(error) = std::fs::rename(target, &backup) {
        let _ = std::fs::remove_file(&staging);
        return Err(from_io("no se pudo respaldar la versión anterior", &error));
    }
    if let Err(error) = std::fs::rename(&staging, target) {
        if had_previous {
            let _ = std::fs::rename(&backup, target);
        }
        return Err(from_io("no se pudo colocar el archivo nuevo", &error));
    }
    Ok(StagedFile {
        target: target.to_owned(),
        staging,
        backup,
        had_previous,
    })
}

/// La instalación salió: quita el temporal y el respaldo que sobren.
pub(crate) fn commit_staged(staged: &StagedFile) {
    let _ = std::fs::remove_file(&staged.staging);
    let _ = std::fs::remove_file(&staged.backup);
}

/// La instalación falló: quita lo nuevo y devuelve la versión anterior.
pub(crate) fn rollback_staged(staged: &StagedFile) {
    let _ = std::fs::remove_file(&staged.target);
    if staged.had_previous {
        let _ = std::fs::rename(&staged.backup, &staged.target);
    }
    let _ = std::fs::remove_file(&staged.staging);
}

/// Ruta hermana con un sufijo añadido (`Dokka.exe` + `.new`).
fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut owned = path.as_os_str().to_owned();
    owned.push(suffix);
    PathBuf::from(owned)
}
