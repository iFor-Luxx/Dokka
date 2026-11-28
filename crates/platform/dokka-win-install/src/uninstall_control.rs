//! Desinstalación desde Preferencias cumpliendo `UninstallControl`.
//!
//! Solo abre `Desinstalar.exe --uninstall`; no cierra Dokka (lo hace el
//! desinstalador, que ya pide su confirmación). Si Dokka no está instalado
//! (versión de desarrollo), el botón se atenúa y no responde.

use std::path::{Path, PathBuf};

use dokka_contracts::{ContractError, UninstallControl};
use windows::Win32::Foundation::S_OK;
use windows::Win32::System::Com::{
    COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoInitializeEx, CoUninitialize,
};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, PCWSTR, w};

/// Dokka ya instalado en Archivos de programa.
pub struct InstalledDokka;

/// Sesión COM del hilo que abre el desinstalador.
struct ComSession {
    owns: bool,
}

impl ComSession {
    fn start() -> Self {
        // SAFETY: inicializa COM en este hilo; si ya estaba, no cambia nada.
        let code =
            unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) };
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

/// Valor que devuelve `ShellExecuteW` a partir del cual todo fue bien.
const FIRST_SUCCESS_VALUE: usize = 33;

impl UninstallControl for InstalledDokka {
    fn can_uninstall(&self) -> bool {
        let Ok(dir) = crate::locations::install_dir() else {
            return false;
        };
        let Ok(uninstaller) = crate::locations::uninstaller_exe() else {
            return false;
        };
        if !uninstaller.is_file() {
            return false;
        }
        let Ok(current) = std::env::current_exe() else {
            return false;
        };
        let Some(parent) = current.parent().map(PathBuf::from) else {
            return false;
        };
        same_folder(&parent, &dir)
    }

    fn open_uninstaller(&self) -> Result<(), ContractError> {
        let uninstaller = crate::locations::uninstaller_exe().map_err(|failure| {
            ContractError::Failed(format!("Dokka no está instalado: {failure}"))
        })?;
        if !uninstaller.is_file() {
            return Err(ContractError::Failed("Dokka no está instalado".to_owned()));
        }
        std::thread::spawn(move || open_in_background(&uninstaller));
        Ok(())
    }
}

/// `true` si ambas carpetas son la misma sin distinguir mayúsculas.
fn same_folder(first: &Path, second: &Path) -> bool {
    first.as_os_str().to_string_lossy().to_lowercase()
        == second.as_os_str().to_string_lossy().to_lowercase()
}

/// Abre el desinstalador en su hilo; si falla o se cancela, lo registra.
fn open_in_background(uninstaller: &Path) {
    let _com = ComSession::start();
    let file = HSTRING::from(uninstaller.as_os_str());
    // SAFETY: las cadenas viven durante toda la llamada.
    let launched = unsafe {
        ShellExecuteW(
            None,
            w!("open"),
            &file,
            w!("--uninstall"),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };
    if (launched.0 as usize) < FIRST_SUCCESS_VALUE {
        eprintln!("Dokka no pudo abrir el desinstalador");
    }
}
