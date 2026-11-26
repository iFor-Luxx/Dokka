use std::path::PathBuf;

use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::UI::Shell::{
    FOLDERID_Desktop, FOLDERID_PublicDesktop, KF_FLAG_DEFAULT, SHGetKnownFolderPath,
};

/// Escritorios del usuario y público.
pub(crate) fn desktop_folders() -> Vec<PathBuf> {
    let mut folders = Vec::new();
    for id in [FOLDERID_Desktop, FOLDERID_PublicDesktop] {
        if let Some(path) = known_folder(&id) {
            folders.push(path);
        }
    }
    folders
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
