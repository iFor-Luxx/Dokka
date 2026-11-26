use windows::Win32::Graphics::Gdi::{CreateRectRgn, SetWindowRgn};

use crate::Surface;

impl Surface {
    /// Limita la ventana a un rectángulo lógico: fuera no se ve ni recibe el mouse.
    pub fn set_active_area(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let pixels = |value: f32| value * self.scale;
        let area = [
            pixels(x).floor() as i32,
            pixels(y).floor() as i32,
            pixels(x + width).ceil() as i32,
            pixels(y + height).ceil() as i32,
        ];
        if self.active_area == Some(area) {
            return;
        }
        self.active_area = Some(area);
        // SAFETY: la ventana es propia. Windows pasa a ser dueño de la región creada.
        unsafe {
            let region = CreateRectRgn(area[0], area[1], area[2], area[3]);
            SetWindowRgn(self.window, Some(region), true);
        }
    }
}
