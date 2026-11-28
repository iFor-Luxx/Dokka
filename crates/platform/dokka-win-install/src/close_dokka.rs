//! Cierre limpio de Dokka antes de tocar sus archivos.
//!
//! Busca las ventanas de la clase `DokkaSurface` (ver
//! `dokka-win-surface/src/create_window.rs`), pide a cada hilo que salga con
//! `WM_QUIT` y espera a que su proceso termine (unos 10 s). Así Dokka pasa
//! por su salida normal y restaura la barra de tareas y la reserva superior.
//! Nunca termina un proceso a la fuerza.

use dokka_contracts::InstallFailure;
use windows::Win32::Foundation::ERROR_INVALID_PARAMETER;
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, WAIT_OBJECT_0, WPARAM};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_SYNCHRONIZE, WaitForSingleObject};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, GetWindowThreadProcessId, PostThreadMessageW, WM_QUIT,
};
use windows::core::{BOOL, HRESULT};

use crate::install_error::from_windows;

/// Espera máxima por proceso, en milisegundos.
const CLOSE_TIMEOUT_MS: u32 = 10_000;
/// Clase de las ventanas de Dokka.
const SURFACE_CLASS: &str = "DokkaSurface";

/// Hilos y procesos de Dokka encontrados al enumerar ventanas.
struct DokkaProcesses {
    threads: Vec<u32>,
    ids: Vec<u32>,
}

/// Cierra todo Dokka que siga abierto. Sin sus ventanas es éxito inmediato.
pub(crate) fn close_running_dokka() -> Result<(), InstallFailure> {
    let mut found = DokkaProcesses {
        threads: Vec::new(),
        ids: Vec::new(),
    };
    // SAFETY: `found` vive durante `EnumWindows` y `collect_dokka` solo añade.
    let listed = unsafe {
        EnumWindows(
            Some(collect_dokka),
            LPARAM(&mut found as *mut DokkaProcesses as isize),
        )
    };
    if let Err(error) = listed {
        return Err(from_windows(error));
    }
    let own = std::process::id();
    for (index, &thread) in found.threads.iter().enumerate() {
        let Some(&id) = found.ids.get(index) else {
            continue;
        };
        if id == own {
            continue;
        }
        request_quit(thread);
        wait_for_exit(id)?;
    }
    Ok(())
}

/// Recoge el hilo y el proceso de cada ventana de la clase de Dokka.
unsafe extern "system" fn collect_dokka(window: HWND, param: LPARAM) -> BOOL {
    // SAFETY: `param` es el `DokkaProcesses` del llamador durante `EnumWindows`.
    let found = unsafe { &mut *(param.0 as *mut DokkaProcesses) };
    let mut name = [0_u16; 64];
    // SAFETY: `name` es un buffer local válido y Windows no lo retiene.
    let length = unsafe { GetClassNameW(window, &mut name) };
    if length <= 0 {
        return BOOL::from(true);
    }
    let length = (length as usize).min(name.len());
    let is_dokka = name[..length]
        .iter()
        .copied()
        .eq(SURFACE_CLASS.encode_utf16());
    if !is_dokka {
        return BOOL::from(true);
    }
    let mut id = 0_u32;
    // SAFETY: `id` es un `u32` válido y solo se escribe en la llamada.
    let thread = unsafe { GetWindowThreadProcessId(window, Some(&mut id)) };
    if thread != 0 && id != 0 && !found.ids.contains(&id) {
        found.ids.push(id);
        found.threads.push(thread);
    }
    BOOL::from(true)
}

/// Pide al hilo que salga con `WM_QUIT`. Si ya terminó, no es error.
fn request_quit(thread: u32) {
    // SAFETY: el hilo tenía ventana y `WM_QUIT` solo le pide salir.
    let _ = unsafe { PostThreadMessageW(thread, WM_QUIT, WPARAM(0), LPARAM(0)) };
}

/// Espera a que el proceso termine. Si sigue vivo, no se pudo cerrar.
fn wait_for_exit(id: u32) -> Result<(), InstallFailure> {
    // SAFETY: solo pide el derecho de espera y el proceso se cierra abajo.
    let process = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, false, id) };
    let process = match process {
        Ok(process) => process,
        Err(error) if error.code() == HRESULT::from_win32(ERROR_INVALID_PARAMETER.0) => {
            // Se cerró solo entre listar y abrir: ya está.
            return Ok(());
        }
        Err(error) => return Err(from_windows(error)),
    };
    // SAFETY: el proceso se abrió arriba y ya no se usa tras esperar.
    let waited = unsafe { WaitForSingleObject(process, CLOSE_TIMEOUT_MS) };
    // SAFETY: el proceso se abrió arriba y ya terminó de usarse.
    let _ = unsafe { CloseHandle(process) };
    if waited == WAIT_OBJECT_0 {
        Ok(())
    } else {
        Err(InstallFailure::CannotCloseDokka)
    }
}
