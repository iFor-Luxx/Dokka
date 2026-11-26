use std::ffi::c_void;

use dokka_contracts::{ContractError, WindowControl};
use dokka_model::WindowId;
use windows::Win32::Foundation::{HWND, LPARAM, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GetForegroundWindow, GetWindowLongPtrW, GetWindowPlacement, GetWindowRect,
    HWND_NOTOPMOST, HWND_TOPMOST, IsIconic, IsWindow, PostMessageW, SW_MINIMIZE, SW_RESTORE,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetForegroundWindow, SetWindowPos, ShowWindowAsync,
    WINDOWPLACEMENT, WM_CLOSE, WPF_RESTORETOMAXIMIZED, WS_EX_TOPMOST,
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
    fn front_window(&self, windows: &[WindowId]) -> Option<WindowId> {
        crate::front_window::front_of(windows)
    }

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

    fn close(&self, id: WindowId) -> Result<(), ContractError> {
        let window = existing_window(id)?;
        // SAFETY: `window` existe; `PostMessageW` no bloquea como `SendMessageW`.
        // La app decide si pregunta antes de cerrarse.
        unsafe { PostMessageW(Some(window), WM_CLOSE, WPARAM(0), LPARAM(0)) }
            .map_err(|error| ContractError::Failed(error.to_string()))
    }

    fn set_topmost(&self, id: WindowId, topmost: bool) -> Result<(), ContractError> {
        let window = existing_window(id)?;
        let place = if topmost {
            HWND_TOPMOST
        } else {
            HWND_NOTOPMOST
        };
        // SAFETY: `window` existe y no se mueve ni cambia de tamaño.
        unsafe {
            SetWindowPos(
                window,
                Some(place),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            )
        }
        .map_err(|error| ContractError::Failed(error.to_string()))
    }

    fn is_topmost(&self, id: WindowId) -> Result<bool, ContractError> {
        let window = existing_window(id)?;
        // SAFETY: `window` existe; solo lee su estilo extendido.
        let style = unsafe { GetWindowLongPtrW(window, GWL_EXSTYLE) } as u32;
        Ok(style & WS_EX_TOPMOST.0 != 0)
    }
}

/// Rectángulo real de la ventana en píxeles de pantalla `[izquierda, arriba,
/// derecha, abajo]`. `None` si no existe o no tiene tamaño. Para una ventana
/// minimizada devuelve su posición restaurada, sin restaurarla ni activarla.
pub fn window_rect(id: WindowId) -> Option<[i32; 4]> {
    let window = HWND(id.value() as *mut c_void);
    // SAFETY: son consultas de solo lectura; un handle destruido devuelve cero.
    unsafe {
        if !IsWindow(Some(window)).as_bool() {
            return None;
        }
        if IsIconic(window).as_bool() {
            return restored_rect(window);
        }
        let mut area = RECT::default();
        if GetWindowRect(window, &mut area).is_err() {
            return None;
        }
        if area.right <= area.left || area.bottom <= area.top {
            return None;
        }
        Some([area.left, area.top, area.right, area.bottom])
    }
}

/// Posición restaurada de una ventana minimizada, sin tocar su estado.
/// `rcNormalPosition` usa coordenadas del área de trabajo; si estaba
/// maximizada se usa el área de trabajo de su monitor.
fn restored_rect(window: HWND) -> Option<[i32; 4]> {
    // SAFETY: `window` existe y cada estructura declara su propio tamaño.
    unsafe {
        let mut placement = WINDOWPLACEMENT {
            length: size_of::<WINDOWPLACEMENT>() as u32,
            ..Default::default()
        };
        if GetWindowPlacement(window, &mut placement).is_err() {
            return None;
        }
        // Minimizada, `showCmd` ya no dice si volverá maximizada: lo dice la marca.
        if placement.flags.0 & WPF_RESTORETOMAXIMIZED.0 != 0 {
            let monitor = MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST);
            let mut info = MONITORINFO {
                cbSize: size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            if !GetMonitorInfoW(monitor, &mut info).as_bool() {
                return None;
            }
            let area = info.rcWork;
            if area.right <= area.left || area.bottom <= area.top {
                return None;
            }
            return Some([area.left, area.top, area.right, area.bottom]);
        }
        let area = placement.rcNormalPosition;
        if area.right <= area.left || area.bottom <= area.top {
            return None;
        }
        Some([area.left, area.top, area.right, area.bottom])
    }
}
