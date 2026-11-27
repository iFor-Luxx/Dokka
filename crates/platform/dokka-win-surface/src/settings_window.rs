use windows::Win32::Foundation::{HWND, WPARAM};
use windows::Win32::Graphics::Dwm::{
    DWM_WINDOW_CORNER_PREFERENCE, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
    DwmSetWindowAttribute,
};
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::{
    HTCAPTION, HWND_NOTOPMOST, HWND_TOPMOST, SW_HIDE, SW_MINIMIZE, SW_SHOWNORMAL, SWP_NOMOVE,
    SWP_NOSIZE, SWP_SHOWWINDOW, SendMessageW, SetWindowPos, ShowWindow, WM_NCLBUTTONDOWN,
};

use crate::foreground::foreground_window;
use crate::place_window::{place_centered, place_centered_clamped};
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

    /// Aplica un tamaño nuevo a Preferencias (zoom verde o vuelta a normal):
    /// lo limita al área de trabajo y lo recentra sin salirse de la pantalla.
    /// La barra lateral no se mueve; solo crece la zona de la derecha.
    pub fn settings_apply_size(&mut self, width: f32, height: f32) -> Result<(), SurfaceError> {
        self.settings_scale = place_centered_clamped(self.settings, width, height)?;
        self.settings_placed = true;
        Ok(())
    }

    /// Muestra la ventana; si ya estaba visible, la trae al frente.
    /// `SW_SHOWNORMAL` también restaura si estaba minimizada.
    pub fn settings_show(&mut self) -> Result<(), SurfaceError> {
        // SAFETY: la ventana es propia y sigue viva mientras sea visible u oculta.
        unsafe {
            let _ = ShowWindow(self.settings, SW_SHOWNORMAL);
            // Sube en el orden aunque Windows niegue el primer plano; la
            // segunda llamada quita el estilo topmost y queda ventana normal.
            let _ = SetWindowPos(
                self.settings,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );
            let _ = SetWindowPos(
                self.settings,
                Some(HWND_NOTOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );
        }
        // El dock no activa a Dokka, así que el primer plano puede ser negado.
        foreground_window(self.settings);
        self.settings_visible = true;
        enable_settings_shadow(self.settings);
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

/// Sombra de la ventana de Preferencias con el sistema (DWM): esquinas
/// redondeadas con sombra suave, sin cambiar su tamaño ni sus golpes.
/// Falla sola: si no se puede activar, la ventana se ve sin ella.
fn enable_settings_shadow(window: HWND) {
    // SAFETY: la ventana es propia; la preferencia es válida y se pasa su tamaño.
    unsafe {
        let preference = DWMWCP_ROUND;
        let _ = DwmSetWindowAttribute(
            window,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            (&raw const preference).cast(),
            size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        );
    }
}
