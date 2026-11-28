//! Traducción de errores de Windows y de disco a `InstallFailure`.

use dokka_contracts::InstallFailure;
use windows::Win32::Foundation::{
    ERROR_ACCESS_DENIED, ERROR_DISK_FULL, ERROR_HANDLE_DISK_FULL, WIN32_ERROR,
};
use windows::core::{Error, HRESULT};

/// Clasifica un error de `windows` sin perder su texto.
pub(crate) fn from_windows(error: Error) -> InstallFailure {
    let code = error.code();
    if code == HRESULT::from_win32(ERROR_ACCESS_DENIED.0) {
        InstallFailure::AccessDenied
    } else if code == HRESULT::from_win32(ERROR_DISK_FULL.0)
        || code == HRESULT::from_win32(ERROR_HANDLE_DISK_FULL.0)
    {
        InstallFailure::NoSpace
    } else {
        InstallFailure::Other(format!("la operación de Windows falló: {error}"))
    }
}

/// Clasifica un código devuelto por el registro.
pub(crate) fn from_win32(error: WIN32_ERROR) -> InstallFailure {
    if error == ERROR_ACCESS_DENIED {
        InstallFailure::AccessDenied
    } else if error == ERROR_DISK_FULL || error == ERROR_HANDLE_DISK_FULL {
        InstallFailure::NoSpace
    } else {
        InstallFailure::Other(format!("Windows devolvió el código {}", error.0))
    }
}

/// Clasifica un error de archivos con su contexto en español.
pub(crate) fn from_io(context: &str, error: &std::io::Error) -> InstallFailure {
    use std::io::ErrorKind;
    if error.kind() == ErrorKind::PermissionDenied {
        return InstallFailure::AccessDenied;
    }
    if error.kind() == ErrorKind::StorageFull {
        return InstallFailure::NoSpace;
    }
    match error.raw_os_error() {
        Some(5) => InstallFailure::AccessDenied,
        Some(112) | Some(39) => InstallFailure::NoSpace,
        _ => InstallFailure::Other(format!("{context}: {error}")),
    }
}
