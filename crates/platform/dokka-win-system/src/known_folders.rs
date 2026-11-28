//! Carpetas del usuario con las rutas conocidas de Windows.
//!
//! Nunca se escriben a mano: salen de `SHGetKnownFolderPath`. `None` si no
//! se lee alguna; nunca se registra ninguna ruta.

use std::path::PathBuf;

use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::System::SystemInformation::GetWindowsDirectoryW;
use windows::Win32::UI::Shell::{
    FOLDERID_Documents, FOLDERID_Downloads, FOLDERID_Music, FOLDERID_Pictures, FOLDERID_Videos,
    KF_FLAG_DEFAULT, SHGetKnownFolderPath,
};

/// Documentos, Imágenes, Música, Videos y Descargas del usuario, en orden.
pub(crate) fn user_folders() -> [Option<PathBuf>; 5] {
    [
        known_folder(&FOLDERID_Documents),
        known_folder(&FOLDERID_Pictures),
        known_folder(&FOLDERID_Music),
        known_folder(&FOLDERID_Videos),
        known_folder(&FOLDERID_Downloads),
    ]
}

/// Carpeta de Windows, como `C:\Windows`.
pub(crate) fn windows_folder() -> Option<PathBuf> {
    let mut buffer = [0_u16; 260];
    // SAFETY: el buffer es escribible y declara su tamaño por el corte.
    let length = unsafe { GetWindowsDirectoryW(Some(&mut buffer)) };
    if length == 0 || length as usize > buffer.len() {
        return None;
    }
    let text = String::from_utf16_lossy(&buffer[..length as usize]);
    (!text.trim().is_empty()).then(|| PathBuf::from(text))
}

fn known_folder(id: &windows::core::GUID) -> Option<PathBuf> {
    // SAFETY: Windows reserva la cadena y se libera con `CoTaskMemFree`.
    let path = unsafe { SHGetKnownFolderPath(id as *const _, KF_FLAG_DEFAULT, None).ok()? };
    // SAFETY: `path` es la cadena que reservó Windows.
    let text = unsafe { path.to_string() }.ok()?;
    // SAFETY: `path` sigue válida hasta liberarla aquí.
    unsafe { CoTaskMemFree(Some(path.0.cast())) };
    (!text.trim().is_empty()).then(|| PathBuf::from(text))
}
