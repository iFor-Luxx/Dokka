use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowRect, HWND_TOPMOST, SW_HIDE, SWP_NOACTIVATE, SWP_SHOWWINDOW, SetWindowPos, ShowWindow,
};

use crate::{Surface, SurfaceError, SurfaceHandle};

impl Surface {
    /// Ventana superpuesta, para lo que deba verse sobre las miniaturas.
    pub fn above_handle(&self) -> SurfaceHandle {
        SurfaceHandle::new(self.above)
    }

    /// Coloca la ventana superpuesta en `[x, y, ancho, alto]` lógicos de la principal,
    /// o la oculta.
    pub fn show_above(&mut self, area: Option<[f32; 4]>) -> Result<(), SurfaceError> {
        let Some([x, y, width, height]) = area else {
            if self.above_position.take().is_some() {
                // SAFETY: la ventana es propia.
                let _ = unsafe { ShowWindow(self.above, SW_HIDE) };
            }
            return Ok(());
        };

        let pixels = |value: f32| (value * self.scale).round() as i32;
        let mut main = RECT::default();
        // SAFETY: la ventana es propia y `main` es escribible.
        unsafe { GetWindowRect(self.window, &mut main)? };
        let position = [
            main.left + pixels(x),
            main.top + pixels(y),
            pixels(width),
            pixels(height),
        ];
        if self.above_position == Some(position) {
            return Ok(());
        }
        let [left, top, width, height] = position;
        // SAFETY: la ventana es propia.
        unsafe {
            SetWindowPos(
                self.above,
                Some(HWND_TOPMOST),
                left,
                top,
                width,
                height,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            )?;
        }
        self.above_position = Some(position);
        Ok(())
    }
}
