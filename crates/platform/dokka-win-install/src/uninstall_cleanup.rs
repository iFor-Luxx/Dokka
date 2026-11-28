//! Borrado de lo que Dokka puso al desinstalar.
//!
//! Solo archivos de Dokka en su carpeta, sus accesos directos (ver
//! `shortcuts`), su valor de arranque y su entrada del registro. El propio
//! `Desinstalar.exe` y su carpeta se marcan para borrarse al reiniciar,
//! porque Windows no deja borrar el programa en marcha. Nada recursivo
//! sobre Archivos de programa: la carpeta solo se quita si queda vacía.

use std::path::Path;

use dokka_contracts::InstallFailure;
use windows::Win32::Storage::FileSystem::{MOVEFILE_DELAY_UNTIL_REBOOT, MoveFileExW};
use windows::core::{HSTRING, PCWSTR};

use crate::install_error::{from_io, from_windows};
use crate::locations::{DOKKA_EXE, UNINSTALLER_EXE};

/// Borra los ejecutables de Dokka y su carpeta (vacía o al reiniciar).
pub(crate) fn remove_install_files(dir: &Path) -> Result<(), InstallFailure> {
    remove_known_file(&dir.join(DOKKA_EXE))?;
    remove_uninstaller(&dir.join(UNINSTALLER_EXE))?;
    remove_leftover(&dir.join(format!("{DOKKA_EXE}.bak")));
    remove_leftover(&dir.join(format!("{DOKKA_EXE}.new")));
    remove_leftover(&dir.join(format!("{UNINSTALLER_EXE}.bak")));
    remove_leftover(&dir.join(format!("{UNINSTALLER_EXE}.new")));
    match std::fs::remove_dir(dir) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => {
            // Quedó algo dentro (el desinstalador en uso): sale al reiniciar.
            schedule_delete_on_reboot(dir)
        }
    }
}

/// Borra la carpeta de ajustes (`%APPDATA%\Dokka`), solo esa.
pub(crate) fn remove_settings_dir() -> Result<(), InstallFailure> {
    let dir = crate::locations::settings_dir()?;
    match std::fs::remove_dir_all(&dir) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(from_io(
            "no se pudo borrar la configuración de Dokka",
            &error,
        )),
    }
}

/// Marca un archivo o carpeta para borrarse al reiniciar Windows.
pub(crate) fn schedule_delete_on_reboot(path: &Path) -> Result<(), InstallFailure> {
    let wide = HSTRING::from(path.as_os_str());
    // SAFETY: la cadena vive en la llamada y el destino nulo pide borrar.
    let scheduled = unsafe { MoveFileExW(&wide, PCWSTR::null(), MOVEFILE_DELAY_UNTIL_REBOOT) };
    scheduled.map_err(from_windows)
}

/// Borra un archivo de Dokka. Si no existe, ya está.
fn remove_known_file(path: &Path) -> Result<(), InstallFailure> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(from_io("no se pudo borrar un archivo de Dokka", &error)),
    }
}

/// Borra el desinstalador; en marcha no se puede y queda para el reinicio.
fn remove_uninstaller(path: &Path) -> Result<(), InstallFailure> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => schedule_delete_on_reboot(path),
    }
}

/// Quita un resto temporal sin que su falta sea error.
fn remove_leftover(path: &Path) {
    let _ = std::fs::remove_file(path);
}
