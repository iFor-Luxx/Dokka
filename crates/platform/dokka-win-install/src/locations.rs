//! Carpetas donde Dokka pone y quita sus archivos, siempre de Windows.
//!
//! Nunca se escriben rutas a mano: salen de `SHGetKnownFolderPath` y de la
//! carpeta de Windows. `None` sería inventar una ruta, así que es error.

use std::path::PathBuf;

use dokka_contracts::InstallFailure;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::System::SystemInformation::GetWindowsDirectoryW;
use windows::Win32::UI::Shell::{
    FOLDERID_CommonPrograms, FOLDERID_Desktop, FOLDERID_ProgramFiles, FOLDERID_RoamingAppData,
    KF_FLAG_DEFAULT, SHGetKnownFolderPath,
};
use windows::core::GUID;

use crate::install_error::{from_io, from_windows};

/// Nombre de la carpeta de Dokka en Archivos de programa.
pub(crate) const DOKKA_EXE: &str = "Dokka.exe";
/// Nombre del desinstalador junto a `Dokka.exe`.
pub(crate) const UNINSTALLER_EXE: &str = "Desinstalar.exe";
/// Nombre del acceso directo en el menú Inicio y el escritorio.
pub(crate) const LINK_NAME: &str = "Dokka.lnk";

/// Carpeta de Dokka en Archivos de programa.
pub(crate) fn install_dir() -> Result<PathBuf, InstallFailure> {
    known_folder(&FOLDERID_ProgramFiles, "Archivos de programa").map(|dir| dir.join("Dokka"))
}

/// Ruta de `Dokka.exe` instalado.
pub(crate) fn dokka_exe() -> Result<PathBuf, InstallFailure> {
    install_dir().map(|dir| dir.join(DOKKA_EXE))
}

/// Ruta de `Desinstalar.exe` instalado.
pub(crate) fn uninstaller_exe() -> Result<PathBuf, InstallFailure> {
    install_dir().map(|dir| dir.join(UNINSTALLER_EXE))
}

/// Acceso directo en el menú Inicio común.
pub(crate) fn start_menu_link() -> Result<PathBuf, InstallFailure> {
    known_folder(&FOLDERID_CommonPrograms, "el menú Inicio común").map(|dir| dir.join(LINK_NAME))
}

/// Acceso directo en el escritorio del usuario.
pub(crate) fn desktop_link() -> Result<PathBuf, InstallFailure> {
    known_folder(&FOLDERID_Desktop, "el escritorio").map(|dir| dir.join(LINK_NAME))
}

/// Carpeta de ajustes (`%APPDATA%\Dokka`), solo para borrarla al desinstalar.
pub(crate) fn settings_dir() -> Result<PathBuf, InstallFailure> {
    known_folder(&FOLDERID_RoamingAppData, "los datos de la cuenta").map(|dir| dir.join("Dokka"))
}

/// `explorer.exe` con su ruta completa, para abrir Dokka sin elevación.
pub(crate) fn explorer_exe() -> Result<PathBuf, InstallFailure> {
    let mut buffer = [0_u16; 260];
    // SAFETY: el buffer es escribible y declara su tamaño por el corte.
    let length = unsafe { GetWindowsDirectoryW(Some(&mut buffer)) };
    if length == 0 || length as usize > buffer.len() {
        return Err(InstallFailure::Other(
            "no se pudo localizar la carpeta de Windows".to_owned(),
        ));
    }
    let text = String::from_utf16_lossy(&buffer[..length as usize]);
    if text.trim().is_empty() {
        return Err(InstallFailure::Other(
            "la carpeta de Windows vino vacía".to_owned(),
        ));
    }
    Ok(PathBuf::from(text).join("explorer.exe"))
}

/// Bytes del ejecutable actual, para copiarlo como `Desinstalar.exe`.
pub(crate) fn current_exe_bytes() -> Result<Vec<u8>, InstallFailure> {
    let exe = std::env::current_exe()
        .map_err(|error| from_io("no se pudo localizar el instalador", &error))?;
    std::fs::read(&exe).map_err(|error| from_io("no se pudo leer el instalador", &error))
}

/// Una carpeta conocida de Windows como `PathBuf`.
fn known_folder(id: &GUID, what: &str) -> Result<PathBuf, InstallFailure> {
    // SAFETY: Windows reserva la cadena y se libera con `CoTaskMemFree`.
    let path = unsafe { SHGetKnownFolderPath(id as *const _, KF_FLAG_DEFAULT, None) };
    let path = path.map_err(from_windows)?;
    // SAFETY: `path` es la cadena que reservó Windows.
    let text = unsafe { path.to_string() }.map_err(|error| {
        InstallFailure::Other(format!("la ruta de Windows no se pudo leer: {error}"))
    })?;
    // SAFETY: `path` sigue válida hasta liberarla aquí.
    unsafe { CoTaskMemFree(Some(path.0.cast())) };
    if text.trim().is_empty() {
        return Err(InstallFailure::Other(format!(
            "Windows devolvió una ruta vacía para {what}"
        )));
    }
    Ok(PathBuf::from(text))
}
