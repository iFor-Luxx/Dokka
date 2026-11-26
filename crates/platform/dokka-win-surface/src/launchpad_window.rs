use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, HWND_TOPMOST, SW_HIDE, SW_SHOWNORMAL, SWP_NOACTIVATE, SWP_NOMOVE,
    SWP_NOSIZE, SWP_SHOWWINDOW, SetForegroundWindow, SetWindowPos, ShowWindow,
};

use crate::place_window::{fullscreen_size, place_fullscreen};
use crate::{Surface, SurfaceError};

impl Surface {
    /// Tamaño lógico del Launchpad para la cuadrícula.
    pub fn launchpad_size(&self) -> [f32; 2] {
        self.launchpad_size
    }

    /// Muestra la pantalla completa justo debajo del dock.
    pub fn launchpad_show(&mut self) -> Result<(), SurfaceError> {
        let _ = place_fullscreen(self.launchpad)?;
        self.launchpad_size = fullscreen_size(self.launchpad, self.scale);
        // SAFETY: ambas ventanas son propias y siguen vivas.
        unsafe {
            SetWindowPos(
                self.launchpad,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            )?;
            let _ = ShowWindow(self.launchpad, SW_SHOWNORMAL);
            // El dock queda encima del Launchpad.
            SetWindowPos(
                self.window,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            )?;
        }
        self.launchpad_visible = true;
        Ok(())
    }

    /// Oculta la pantalla completa del Launchpad.
    pub fn launchpad_hide(&mut self) {
        if !self.launchpad_visible {
            return;
        }
        // SAFETY: la ventana es propia.
        unsafe {
            let _ = ShowWindow(self.launchpad, SW_HIDE);
        }
        self.launchpad_visible = false;
    }

    /// Toma el foco para recibir Esc y notar si se pierde.
    pub fn launchpad_opened(&mut self) -> Result<(), SurfaceError> {
        // SAFETY: consulta y cambio de foco de ventanas del mismo escritorio.
        unsafe {
            let current = GetForegroundWindow();
            if current != self.launchpad && current != self.window {
                self.launchpad_previous = Some(current);
            }
            if GetForegroundWindow() != self.launchpad {
                let _ = SetForegroundWindow(self.launchpad);
            }
            // Tomar el foco sube la ventana: se vuelve a colocar justo debajo del dock.
            SetWindowPos(
                self.launchpad,
                Some(self.window),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            )?;
            Ok(())
        }
    }

    /// Devuelve el foco anterior al cerrar, sin robar lo recién pulsado.
    pub fn launchpad_closed(&mut self) {
        let Some(previous) = self.launchpad_previous.take() else {
            return;
        };
        if previous.is_invalid() {
            return;
        }
        // SAFETY: solo devuelve el foco si Dokka sigue al frente.
        unsafe {
            if GetForegroundWindow() == self.launchpad {
                let _ = SetForegroundWindow(previous);
            }
        }
    }

    pub(crate) fn launchpad_window(&self) -> HWND {
        self.launchpad
    }
}
