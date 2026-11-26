use std::ffi::c_void;

use dokka_contracts::{ContractError, WindowControl};
use dokka_model::WindowId;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, IsIconic, IsWindow, SW_MINIMIZE, SW_RESTORE, SetForegroundWindow,
    ShowWindowAsync,
};

/// Control de ventanas mediante Windows.
#[derive(Debug, Default)]
pub struct SystemWindowControl;

/// Ventana que tiene el foco ahora, si hay alguna.
pub fn foreground_window() -> Option<WindowId> {
    // SAFETY: consulta sin argumentos.
    let window = unsafe { GetForegroundWindow() };
    (!window.is_invalid()).then(|| WindowId::new(window.0 as u64))
}

fn existing_window(id: WindowId) -> Result<HWND, ContractError> {
    let window = HWND(id.value() as *mut c_void);
    // SAFETY: `IsWindow` admite cualquier valor.
    if unsafe { IsWindow(Some(window)) }.as_bool() {
        Ok(window)
    } else {
        Err(ContractError::NotFound)
    }
}

impl WindowControl for SystemWindowControl {
    fn activate(&self, id: WindowId) -> Result<(), ContractError> {
        let window = existing_window(id)?;
        // SAFETY: `window` existe. La versión asíncrona evita colgarse con apps bloqueadas.
        // El clic en el dock da a este proceso permiso para cambiar el foco.
        let focused = unsafe {
            if IsIconic(window).as_bool() {
                let _ = ShowWindowAsync(window, SW_RESTORE);
            }
            SetForegroundWindow(window).as_bool()
        };
        if focused {
            Ok(())
        } else {
            Err(ContractError::Failed(
                "Windows no permitió traer la ventana".into(),
            ))
        }
    }

    fn minimize(&self, id: WindowId) -> Result<(), ContractError> {
        let window = existing_window(id)?;
        // SAFETY: `window` existe.
        let _ = unsafe { ShowWindowAsync(window, SW_MINIMIZE) };
        Ok(())
    }
}
