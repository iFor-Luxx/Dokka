use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowRect, SW_HIDE, SW_MINIMIZE, SW_SHOWNORMAL, SWP_NOACTIVATE, SWP_NOSIZE, SWP_NOZORDER,
    SetForegroundWindow, SetWindowPos, ShowWindow,
};

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
    pub fn settings_show(&mut self) -> Result<(), SurfaceError> {
        // SAFETY: la ventana es propia y sigue viva mientras sea visible u oculta.
        unsafe {
            let _ = ShowWindow(self.settings, SW_SHOWNORMAL);
            // Ventana normal, sin ser la más alta: solo se pone al frente ahora.
            let _ = SetForegroundWindow(self.settings);
        }
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

    /// Escala de la ventana de Preferencias para pasar a píxeles.
    pub fn settings_scale(&self) -> f32 {
        self.settings_scale.max(1.0)
    }

    /// Mueve la ventana lo arrastrado en unidades lógicas.
    pub fn settings_drag_by(&mut self, dx: f32, dy: f32) -> Result<(), SurfaceError> {
        if dx == 0.0 && dy == 0.0 {
            return Ok(());
        }
        let scale = self.settings_scale();
        // SAFETY: la ventana es propia y `rect` es escribible.
        unsafe {
            let mut rect = RECT::default();
            GetWindowRect(self.settings, &mut rect)?;
            SetWindowPos(
                self.settings,
                None,
                rect.left + (dx * scale).round() as i32,
                rect.top + (dy * scale).round() as i32,
                0,
                0,
                SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
            )?;
        }
        Ok(())
    }

    pub(crate) fn settings_window(&self) -> HWND {
        self.settings
    }
}
