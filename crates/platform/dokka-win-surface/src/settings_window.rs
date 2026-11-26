use windows::Win32::Foundation::{HWND, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::{
    HTCAPTION, SW_HIDE, SW_MINIMIZE, SW_SHOWNORMAL, SendMessageW, ShowWindow, WM_NCLBUTTONDOWN,
};

use crate::foreground::foreground_window;
use crate::place_window::place_centered;
use crate::{Surface, SurfaceError, SurfaceHandle};

impl Surface {
    pub fn settings_handle(&self) -> SurfaceHandle {
        SurfaceHandle::new(self.settings)
    }

    /// Centra la ventana una sola vez; el shell pasa las medidas del tema.
    pub fn settings_place(&mut self, width: f32, height: f32) -> Result<(), SurfaceError> {
        if self.settings_placed {
            return Ok(());
        }
        self.settings_scale = place_centered(self.settings, width, height)?;
        self.settings_placed = true;
        Ok(())
    }

    /// Muestra la ventana; si ya estaba visible, la trae al frente.
    /// `SW_SHOWNORMAL` también restaura si estaba minimizada.
    pub fn settings_show(&mut self) -> Result<(), SurfaceError> {
        // SAFETY: la ventana es propia y sigue viva mientras sea visible u oculta.
        unsafe {
            let _ = ShowWindow(self.settings, SW_SHOWNORMAL);
        }
        // El dock no activa a Dokka, así que el primer plano puede ser negado.
        foreground_window(self.settings);
        self.settings_visible = true;
        Ok(())
    }

    /// Oculta la ventana sin destruirla: destruir una ventana sin dueña
    /// terminaría la app. Dokka sigue en marcha.
    pub fn settings_hide(&mut self) {
        if !self.settings_visible {
            return;
        }
        // SAFETY: la ventana es propia.
        unsafe {
            let _ = ShowWindow(self.settings, SW_HIDE);
        }
        self.settings_visible = false;
    }

    /// Minimiza la ventana con el semáforo amarillo.
    pub fn settings_minimize(&mut self) {
        // SAFETY: la ventana es propia.
        unsafe {
            let _ = ShowWindow(self.settings, SW_MINIMIZE);
        }
    }

    /// Empieza el arrastre nativo desde la franja superior: Windows mueve la
    /// ventana hasta soltar el botón, sin temporizadores ni posiciones a mano.
    pub fn settings_begin_drag(&self) {
        // SAFETY: la ventana es propia; el mensaje entra en su bucle modal y
        // vuelve al soltar el botón.
        unsafe {
            let _ = ReleaseCapture();
            let _ = SendMessageW(
                self.settings,
                WM_NCLBUTTONDOWN,
                Some(WPARAM(HTCAPTION as usize)),
                None,
            );
        }
    }

    /// Escala de la ventana de Preferencias para pasar a píxeles.
    pub fn settings_scale(&self) -> f32 {
        self.settings_scale.max(1.0)
    }

    pub(crate) fn settings_window(&self) -> HWND {
        self.settings
    }
}
