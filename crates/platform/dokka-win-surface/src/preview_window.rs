use windows::Win32::UI::WindowsAndMessaging::{
    SW_HIDE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos, ShowWindow,
};

use crate::place_window::place_fullscreen;
use crate::{Surface, SurfaceError, SurfaceHandle};

impl Surface {
    /// Identificador de la vista grande para el compositor.
    pub fn preview_handle(&self) -> SurfaceHandle {
        SurfaceHandle::new(self.preview)
    }

    /// Muestra la vista grande a pantalla completa, por debajo del dock.
    pub fn preview_show(&mut self) -> Result<(), SurfaceError> {
        // Se llama en cada redibujado: reordenar solo al aparecer evita parpadeos.
        if self.preview_visible {
            return Ok(());
        }
        let _ = place_fullscreen(self.preview)?;
        // La ventana sigue oculta: `place_fullscreen` la ordena como
        // TOPMOST sin pintarla, así que no hay parpadeo visible.
        // Se muestra justo detrás de la principal; la superpuesta, dueña
        // de la principal, sigue encima sin tocarla.
        // SAFETY: ambas ventanas son propias y siguen vivas.
        unsafe {
            SetWindowPos(
                self.preview,
                Some(self.window),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            )?;
        }
        self.preview_visible = true;
        Ok(())
    }

    /// Oculta la vista grande.
    pub fn preview_hide(&mut self) {
        if !self.preview_visible {
            return;
        }
        // SAFETY: la ventana es propia.
        unsafe {
            let _ = ShowWindow(self.preview, SW_HIDE);
        }
        self.preview_visible = false;
    }
}
