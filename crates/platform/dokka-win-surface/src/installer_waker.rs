//! Despierta la ventana del instalador desde el hilo de trabajo.

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::PostMessageW;

use super::installer_window::InstallerWindow;

/// Aviso que se puede llevar a otro hilo: guarda la ventana como número.
#[derive(Debug, Clone, Copy)]
pub struct InstallerWaker {
    window: isize,
    message: u32,
}

impl InstallerWaker {
    /// Pide a la ventana que recoja el avance. Si ya no existe, no pasa nada.
    pub fn wake(self) {
        let window = HWND(self.window as *mut core::ffi::c_void);
        // SAFETY: `PostMessageW` es segura entre hilos y no usa punteros propios;
        // con una ventana ya cerrada solo devuelve error.
        let _ = unsafe { PostMessageW(Some(window), self.message, WPARAM(0), LPARAM(0)) };
    }
}

impl InstallerWindow {
    /// `message` es el mismo que se pasa a `run`.
    pub fn waker(&self, message: u32) -> InstallerWaker {
        InstallerWaker {
            window: self.window().0 as isize,
            message,
        }
    }
}
