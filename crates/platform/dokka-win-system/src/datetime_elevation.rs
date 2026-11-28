//! Elevación y privilegios para cambiar fecha, hora y zona.
//!
//! Solo lee el testigo y activa privilegios el tiempo de la llamada.
//! Nada cambia sin un clic y nada se deshace al cerrar Dokka.

use windows::Win32::Foundation::{HANDLE, LUID};
use windows::Win32::Security::{
    AdjustTokenPrivileges, GetTokenInformation, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED,
    TOKEN_ADJUST_PRIVILEGES, TOKEN_ELEVATION, TOKEN_PRIVILEGES, TOKEN_QUERY, TokenElevation,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::core::PCWSTR;

/// `true` si el proceso está elevado (`TokenElevation`).
pub(crate) fn is_elevated() -> bool {
    let mut token = HANDLE::default();
    // SAFETY: el proceso propio entrega un testigo válido que se cierra abajo.
    let _ = unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) };
    if token.is_invalid() {
        return false;
    }
    let mut elevation = TOKEN_ELEVATION::default();
    let mut returned: u32 = 0;
    // SAFETY: `elevation` es un `TOKEN_ELEVATION` válido y se lee su tamaño.
    let read = unsafe {
        GetTokenInformation(
            token,
            TokenElevation,
            Some((&mut elevation as *mut TOKEN_ELEVATION).cast::<core::ffi::c_void>()),
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut returned,
        )
        .is_ok()
    };
    // SAFETY: el testigo se abrió arriba y ya no se usa.
    unsafe {
        let _ = windows::Win32::Foundation::CloseHandle(token);
    }
    read && elevation.TokenIsElevated != 0
}

/// Activa el privilegio `name` solo durante `action` y lo revierte.
pub(crate) fn with_privilege(name: PCWSTR, action: impl FnOnce() -> bool) -> bool {
    let mut token = HANDLE::default();
    // SAFETY: el proceso propio entrega un testigo válido que se cierra abajo.
    let _ = unsafe {
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        )
    };
    if token.is_invalid() {
        return false;
    }
    let mut luid = LUID::default();
    // SAFETY: `luid` es válido y solo se rellena con el identificador.
    let known = unsafe { LookupPrivilegeValueW(None, name, &mut luid).is_ok() };
    if !known {
        // SAFETY: el testigo se abrió arriba y ya no se usa.
        unsafe {
            let _ = windows::Win32::Foundation::CloseHandle(token);
        }
        return false;
    }
    let enable = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [windows::Win32::Security::LUID_AND_ATTRIBUTES {
            Luid: luid,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };
    // SAFETY: `enable` describe un privilegio válido para este testigo.
    let _ = unsafe {
        AdjustTokenPrivileges(
            token,
            false,
            Some(&enable as *const TOKEN_PRIVILEGES),
            0,
            None,
            None,
        )
    };
    let done = action();
    let disable = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [windows::Win32::Security::LUID_AND_ATTRIBUTES {
            Luid: luid,
            Attributes: windows::Win32::Security::TOKEN_PRIVILEGES_ATTRIBUTES(0),
        }],
    };
    // SAFETY: revierte el mismo privilegio que se activó arriba.
    let _ = unsafe {
        AdjustTokenPrivileges(
            token,
            false,
            Some(&disable as *const TOKEN_PRIVILEGES),
            0,
            None,
            None,
        )
    };
    // SAFETY: el testigo se abrió arriba y ya no se usa.
    unsafe {
        let _ = windows::Win32::Foundation::CloseHandle(token);
    }
    done
}
