//! Arranque con Windows: valor `Dokka` en `HKCU\...\Run`.
//!
//! Solo ese valor; nunca toca otros. Al cambiarlo se guarda lo que hubiera
//! para devolverlo si la instalación falla.

use std::path::Path;

use dokka_contracts::InstallFailure;
use windows::Win32::System::Registry::HKEY_CURRENT_USER;

use crate::registry::{OpenKey, delete_value, read_string_value, write_string_value};

/// Clave del arranque por usuario.
const RUN_SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
/// Nombre del valor de Dokka.
const RUN_VALUE: &str = "Dokka";

/// Lo que había en el arranque antes de cambiarlo.
pub(crate) struct RunBackup {
    pub(crate) previous: Option<String>,
}

/// Lee el valor actual del arranque de Dokka, si existe.
pub(crate) fn read_run_value() -> Option<String> {
    read_string_value(HKEY_CURRENT_USER, RUN_SUBKEY, RUN_VALUE)
}

/// Apunta `Dokka.exe` entre comillas en el arranque. Guarda lo anterior.
pub(crate) fn set_run_value(exe: &Path) -> Result<RunBackup, InstallFailure> {
    let previous = read_run_value();
    let quoted = quoted_path(exe)?;
    let key = OpenKey::create(HKEY_CURRENT_USER, RUN_SUBKEY)?;
    write_string_value(key.handle(), RUN_VALUE, &quoted)?;
    Ok(RunBackup { previous })
}

/// Quita a Dokka del arranque. Si no estaba, ya está.
pub(crate) fn remove_run_value() -> Result<(), InstallFailure> {
    let key = OpenKey::create(HKEY_CURRENT_USER, RUN_SUBKEY)?;
    delete_value(key.handle(), RUN_VALUE)
}

/// Devuelve el arranque a como estaba: lo anterior o nada.
pub(crate) fn restore_run_value(backup: &RunBackup) {
    if let Ok(key) = OpenKey::create(HKEY_CURRENT_USER, RUN_SUBKEY) {
        match &backup.previous {
            Some(previous) => {
                let _ = write_string_value(key.handle(), RUN_VALUE, previous);
            }
            None => {
                let _ = delete_value(key.handle(), RUN_VALUE);
            }
        }
    }
}

/// Ruta entre comillas para el valor del registro.
fn quoted_path(exe: &Path) -> Result<String, InstallFailure> {
    let mut quoted = std::ffi::OsString::from("\"");
    quoted.push(exe.as_os_str());
    quoted.push("\"");
    quoted.into_string().map_err(|_| {
        InstallFailure::Other("la ruta de Dokka no se puede escribir en el registro".to_owned())
    })
}
