//! Dueño del icono: de qué programa viene cada aviso.
//!
//! Solo se consulta mientras el icono no se conoce, no en cada `MODIFY`: hay apps
//! que reenvían su aviso sin parar. La ventana dueña da el proceso con
//! `GetWindowThreadProcessId` + `OpenProcess` +
//! `QueryFullProcessImageNameW`; de su ruta sale si es del Explorador y,
//! cuando el icono llega sin texto, el nombre de la app.

use std::ffi::{OsString, c_void};
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use dokka_win_apps::NameCache;
use windows::Win32::Foundation::{CloseHandle, HWND};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
};
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
use windows::core::PWSTR;

/// Ruta del ejecutable dueño de la ventana; `None` si no se puede consultar.
pub(crate) fn executable_of(hwnd_raw: u32) -> Option<PathBuf> {
    if hwnd_raw == 0 {
        return None;
    }
    let window = HWND(hwnd_raw as usize as *mut c_void);
    let mut owner = 0;
    // SAFETY: `owner` es escribible durante la llamada.
    unsafe { GetWindowThreadProcessId(window, Some(&mut owner)) };
    if owner == 0 {
        return None;
    }
    process_path(owner)
}

/// `true` si la ruta es del Explorador: sus iconos son de Windows,
/// no de apps en segundo plano.
pub(crate) fn is_explorer(executable: &Path) -> bool {
    executable
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("explorer.exe"))
}

/// Nombre de la app: su descripción de versión y, si no la tiene,
/// el nombre del archivo sin extensión.
pub(crate) fn app_title(executable: &Path, names: &mut NameCache) -> Option<String> {
    if let Some(description) = names.name_for(executable) {
        return Some(description);
    }
    let stem = executable.file_stem()?.to_string_lossy().trim().to_owned();
    (!stem.is_empty()).then_some(stem)
}

/// Ruta del ejecutable de un proceso; `None` si no deja consultarse.
fn process_path(process: u32) -> Option<PathBuf> {
    let mut buffer = [0_u16; 1024];
    let mut length = buffer.len() as u32;
    // SAFETY: el handle se cierra aquí mismo y `length` describe el búfer.
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process).ok()?;
        let result = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut length,
        );
        let _ = CloseHandle(handle);
        result.ok()?;
    }
    Some(OsString::from_wide(&buffer[..length as usize]).into())
}
