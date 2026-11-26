use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};

use crate::{Surface, SurfaceError};

impl Surface {
    /// Toma el foco para que el menú reciba Esc y avisos de desactivación.
    pub fn menu_opened(&mut self) -> Result<(), SurfaceError> {
        // SAFETY: consulta y cambio de foco de ventanas del mismo escritorio.
        unsafe {
            let current = GetForegroundWindow();
            if current != self.window {
                self.previous_focus = Some(current);
                let _ = SetForegroundWindow(self.window);
            }
            Ok(())
        }
    }

    /// Devuelve el foco anterior, sin robar el de lo que se acaba de pulsar.
    pub fn menu_closed(&mut self) {
        let Some(previous) = self.previous_focus.take() else {
            return;
        };
        // SAFETY: solo devuelve el foco si Dokka sigue siendo la ventana activa.
        unsafe {
            if GetForegroundWindow() == self.window {
                let _ = SetForegroundWindow(previous);
            }
        }
    }

    /// Olvida el foco anterior sin devolverlo: Preferencias se abre mientras
    /// Dokka aún es la ventana activa, así Windows le concede el primer plano.
    pub fn menu_dismissed(&mut self) {
        self.previous_focus = None;
    }
}
