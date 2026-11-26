use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
    QueryFullProcessImageNameW,
};
use windows::Win32::UI::WindowsAndMessaging::{EnumChildWindows, GetWindowThreadProcessId};
use windows::core::{BOOL, PWSTR};

/// Contenedor de Windows para apps de la tienda: la app real es una ventana hija.
const STORE_APP_HOST: &str = "applicationframehost.exe";

struct HostedSearch {
    host: u32,
    hosted: Option<u32>,
}

/// Ejecutable dueño de la ventana; `None` si es de Dokka o no se puede consultar.
pub(crate) fn find_executable(window: HWND) -> Option<PathBuf> {
    let process = process_of(window);
    // SAFETY: solo consulta el identificador del proceso actual.
    if process == unsafe { GetCurrentProcessId() } {
        return None;
    }
    let path = process_path(process)?;
    let is_host = path
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case(STORE_APP_HOST));
    if !is_host {
        return Some(path);
    }
    hosted_process(window, process)
        .and_then(process_path)
        .or(Some(path))
}

fn process_of(window: HWND) -> u32 {
    let mut process = 0;
    // SAFETY: `process` es escribible durante la llamada.
    unsafe { GetWindowThreadProcessId(window, Some(&mut process)) };
    process
}

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

fn hosted_process(window: HWND, host: u32) -> Option<u32> {
    let mut search = HostedSearch { host, hosted: None };
    // SAFETY: `search` vive durante la enumeración, que es síncrona.
    unsafe {
        let _ = EnumChildWindows(
            Some(window),
            Some(find_foreign_child),
            LPARAM(&raw mut search as isize),
        );
    }
    search.hosted
}

unsafe extern "system" fn find_foreign_child(child: HWND, data: LPARAM) -> BOOL {
    // SAFETY: `data` es el puntero a la búsqueda que entregó `hosted_process`.
    let search = unsafe { &mut *(data.0 as *mut HostedSearch) };
    let process = process_of(child);
    if process != search.host {
        search.hosted = Some(process);
        return false.into();
    }
    true.into()
}
