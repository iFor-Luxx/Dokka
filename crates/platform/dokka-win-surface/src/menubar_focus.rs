use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};

use crate::foreground::foreground_window;
use crate::Surface;

impl Surface {
    /// Toma el foco para saber cuándo cerrar y recibir el teclado: la pérdida
    /// llega como `MenubarDismissed`. Une la entrada al hilo activo para que
    /// Windows conceda el primer plano y fija el foco del teclado.
    pub fn menubar_menu_opened(&mut self) {
        if self.menubar.is_invalid() {
            return;
        }
        // SAFETY: consulta de solo lectura de la ventana activa.
        unsafe {
            let current = GetForegroundWindow();
            if current != self.menubar {
                self.menubar_previous = Some(current);
            }
        }
        foreground_window(self.menubar);
        // SAFETY: la ventana es propia y sigue viva.
        unsafe {
            let _ = SetFocus(Some(self.menubar));
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
