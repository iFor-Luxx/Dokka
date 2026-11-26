use std::thread;

use dokka_contracts::{ContractError, RecycleBin};
use windows::Win32::Foundation::ERROR_CANCELLED;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows::Win32::UI::Shell::SHEmptyRecycleBinW;
use windows::core::{HRESULT, PCWSTR};

/// La papelera de reciclaje del sistema.
#[derive(Debug, Default)]
pub struct SystemRecycleBin;

impl RecycleBin for SystemRecycleBin {
    fn empty(&self) -> Result<(), ContractError> {
        // `SHEmptyRecycleBinW` muestra su propia confirmación y no vuelve hasta
        // que el usuario responde: el hilo que la llama espera de todos modos.
        // Se usa un hilo propio con COM iniciado para no tocar el apartamento
        // COM del llamante; aun así, conviene llamarla fuera del hilo de la
        // interfaz porque el contrato es síncrono y espera al usuario.
        match thread::spawn(empty_once).join() {
            Ok(result) => result,
            Err(_) => Err(ContractError::Failed(
                "no se pudo vaciar la papelera".to_owned(),
            )),
        }
    }
}

/// Libera COM al salir del hilo de la papelera.
struct ComGuard;

impl Drop for ComGuard {
    fn drop(&mut self) {
        // SAFETY: este hilo inicializó COM y sigue vivo.
        unsafe { CoUninitialize() };
    }
}

/// Vacía la papelera con la confirmación de Windows.
///
/// `dwflags` es 0 a propósito: nunca `SHERB_NOCONFIRMATION`. Si ya está vacía,
/// Windows devuelve éxito; si el usuario cancela (`ERROR_CANCELLED`), tampoco
/// es error.
fn empty_once() -> Result<(), ContractError> {
    // SAFETY: inicializa COM solo para este hilo.
    let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    let _com = ComGuard;
    // SAFETY: sin ventana dueña y sin ruta son todas las papeleras.
    let emptied = unsafe { SHEmptyRecycleBinW(None, PCWSTR::null(), 0) };
    match emptied {
        Ok(()) => Ok(()),
        Err(error) if error.code() == HRESULT::from_win32(ERROR_CANCELLED.0) => Ok(()),
        Err(error) => Err(ContractError::Failed(format!(
            "no se pudo vaciar la papelera: {error}"
        ))),
    }
}
