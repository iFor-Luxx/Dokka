use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};

use crate::Surface;

impl Surface {
    /// Toma el foco para saber cuándo cerrar: la pérdida llega como `MenubarDismissed`.
    pub fn menubar_menu_opened(&mut self) {
        if self.menubar.is_invalid() {
            return;
        }
        // SAFETY: consulta y cambio de foco de ventanas del mismo escritorio.
        unsafe {
            let current = GetForegroundWindow();
            if current != self.menubar {
                self.menubar_previous = Some(current);
                let _ = SetForegroundWindow(self.menubar);
            }
        }
    }

    /// Devuelve el foco anterior, sin robar el de lo que se acaba de pulsar.
    pub fn menubar_menu_closed(&mut self) {
        let Some(previous) = self.menubar_previous.take() else {
            return;
        };
        // SAFETY: solo devuelve el foco si la barra sigue siendo la activa.
        unsafe {
            if GetForegroundWindow() == self.menubar {
                let _ = SetForegroundWindow(previous);
            }
        }
    }

    /// Olvida el foco anterior sin devolverlo.
    pub fn menubar_menu_dismissed(&mut self) {
        self.menubar_previous = None;
    }
}
