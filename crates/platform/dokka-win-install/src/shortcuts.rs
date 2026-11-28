//! Accesos directos del menú Inicio y del escritorio con `IShellLinkW`.
//!
//! Solo crea o borra los `.lnk` de Dokka; nunca toca otros. Al crear se
//! guarda lo que hubiera antes para deshacerlo si la instalación falla.

use std::path::Path;

use dokka_contracts::InstallFailure;
use windows::Win32::Foundation::S_OK;
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
    IPersistFile,
};
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
use windows::core::{HSTRING, Interface};

use crate::install_error::{from_io, from_windows};

/// Un acceso directo creado, con lo que había antes para deshacerlo.
pub(crate) struct ShortcutChange {
    link: std::path::PathBuf,
    previous: Option<Vec<u8>>,
}

/// Sesión COM del hilo: solo libera si la inició ella.
struct ComSession {
    owns: bool,
}

impl ComSession {
    fn start() -> Self {
        // SAFETY: inicializa COM en este hilo; si ya estaba, no cambia nada.
        let code = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        Self { owns: code == S_OK }
    }
}

impl Drop for ComSession {
    fn drop(&mut self) {
        if self.owns {
            // SAFETY: este hilo inició COM y sigue vivo.
            unsafe { CoUninitialize() };
        }
    }
}

/// Crea el acceso directo a `target` en `link` con su icono y descripción.
pub(crate) fn create_shortcut(
    link: &Path,
    target: &Path,
) -> Result<ShortcutChange, InstallFailure> {
    let previous = std::fs::read(link).ok();
    if let Some(parent) = link.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| from_io("no se pudo crear la carpeta del acceso directo", &error))?;
    }
    let _com = ComSession::start();
    // SAFETY: crea el enlace en este hilo con COM ya iniciado.
    let shell: IShellLinkW =
        unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_ALL) }.map_err(from_windows)?;
    let target_text = HSTRING::from(target.as_os_str());
    // SAFETY: `target_text` vive durante la llamada y solo se lee.
    unsafe { shell.SetPath(&target_text) }.map_err(from_windows)?;
    if let Some(folder) = target.parent() {
        let folder_text = HSTRING::from(folder.as_os_str());
        // SAFETY: `folder_text` vive durante la llamada y solo se lee.
        unsafe { shell.SetWorkingDirectory(&folder_text) }.map_err(from_windows)?;
    }
    // SAFETY: `target_text` vive durante la llamada y solo se lee.
    unsafe { shell.SetIconLocation(&target_text, 0) }.map_err(from_windows)?;
    let description = HSTRING::from("Dokka");
    // SAFETY: `description` vive durante la llamada y solo se lee.
    unsafe { shell.SetDescription(&description) }.map_err(from_windows)?;
    let persist: IPersistFile = shell.cast().map_err(from_windows)?;
    let link_text = HSTRING::from(link.as_os_str());
    // SAFETY: `link_text` vive durante la llamada y solo se escribe el `.lnk`.
    unsafe { persist.Save(&link_text, true) }.map_err(from_windows)?;
    Ok(ShortcutChange {
        link: link.to_owned(),
        previous,
    })
}

/// Borra un acceso directo de Dokka. Si no existe, ya está.
pub(crate) fn remove_shortcut(link: &Path) -> Result<(), InstallFailure> {
    match std::fs::remove_file(link) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(from_io("no se pudo borrar el acceso directo", &error)),
    }
}

/// Deshace una creación: devuelve lo que había o quita el archivo.
pub(crate) fn undo_shortcut(change: &ShortcutChange) {
    match &change.previous {
        Some(bytes) => {
            let _ = std::fs::write(&change.link, bytes);
        }
        None => {
            let _ = std::fs::remove_file(&change.link);
        }
    }
}
