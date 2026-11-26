use std::ffi::OsString;
use std::path::{Path, PathBuf};

use dokka_contracts::ContractError;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, PCWSTR, w};

use crate::resolve_lnk::resolve_lnk;

/// Valor que devuelve `ShellExecuteW` a partir del cual todo fue bien.
const FIRST_SUCCESS_VALUE: usize = 33;

/// Muestra el archivo seleccionado en el Explorador.
///
/// Si es un acceso directo `.lnk`, se muestra su destino cuando se puede
/// resolver; si no, el propio `.lnk`.
pub(crate) fn show_in_folder(executable: &Path, app_name: &str) -> Result<(), ContractError> {
    let shown = link_target(executable);
    // Los nombres de Windows no admiten comillas, así que entrecomillar basta.
    let parameters = OsString::from(format!("/select,\"{}\"", shown.display()));
    let parameters = HSTRING::from(parameters.as_os_str());
    // SAFETY: las cadenas viven durante toda la llamada.
    let opened = unsafe {
        ShellExecuteW(
            None,
            w!("open"),
            w!("explorer.exe"),
            &parameters,
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };
    if opened.0 as usize >= FIRST_SUCCESS_VALUE {
        Ok(())
    } else {
        Err(ContractError::Failed(format!(
            "no se pudo mostrar {app_name} en su carpeta"
        )))
    }
}

/// Destino del acceso directo cuando se puede resolver; si no, el propio archivo.
fn link_target(executable: &Path) -> PathBuf {
    let is_link = executable
        .extension()
        .is_some_and(|extension| extension.to_string_lossy().to_lowercase() == "lnk");
    if !is_link {
        return executable.to_path_buf();
    }
    resolve_lnk(executable).unwrap_or_else(|| executable.to_path_buf())
}
