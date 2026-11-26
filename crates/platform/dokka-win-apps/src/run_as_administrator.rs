use std::path::Path;

use dokka_contracts::ContractError;
use windows::Win32::Foundation::{ERROR_CANCELLED, GetLastError};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, PCWSTR, w};

/// Valor que devuelve `ShellExecuteW` a partir del cual todo fue bien.
const FIRST_SUCCESS_VALUE: usize = 33;

/// Abre el ejecutable pidiendo permisos de administrador.
///
/// Windows muestra su propia pregunta; si el usuario la rechaza
/// (`ERROR_CANCELLED`) se devuelve `Ok`, como pide el contrato.
pub(crate) fn run_as_administrator(executable: &Path, app_name: &str) -> Result<(), ContractError> {
    let file = HSTRING::from(executable.as_os_str());
    // SAFETY: las cadenas viven durante toda la llamada.
    let launched = unsafe {
        ShellExecuteW(
            None,
            w!("runas"),
            &file,
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };
    if launched.0 as usize >= FIRST_SUCCESS_VALUE {
        return Ok(());
    }
    // SAFETY: lee el error justo después de `ShellExecuteW`, sin llamadas en medio.
    if unsafe { GetLastError() }.0 == ERROR_CANCELLED.0 {
        // El usuario dijo que no en la pregunta de Windows: no es un error.
        return Ok(());
    }
    Err(ContractError::Failed(format!(
        "no se pudo abrir {app_name} como administrador"
    )))
}
