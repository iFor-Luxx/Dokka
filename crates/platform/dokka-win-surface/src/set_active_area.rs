use windows::Win32::Graphics::Gdi::{
    CombineRgn, CreateRectRgn, DeleteObject, RGN_OR, SetWindowRgn,
};

use crate::Surface;

impl Surface {
    /// Limita la ventana a la unión de rectángulos lógicos `[x, y, ancho, alto]`:
    /// fuera de ellos no se ve ni recibe el mouse.
    pub fn set_active_areas(&mut self, areas: &[[f32; 4]]) {
        let pixels: Vec<[i32; 4]> = areas
            .iter()
            .map(|&[x, y, width, height]| {
                [
                    (x * self.scale).floor() as i32,
                    (y * self.scale).floor() as i32,
                    ((x + width) * self.scale).ceil() as i32,
                    ((y + height) * self.scale).ceil() as i32,
                ]
            })
            .collect();
        if self.active_areas == pixels {
            return;
        }
        // SAFETY: las regiones intermedias se borran aquí; la final pasa a ser de Windows.
        unsafe {
            let region = CreateRectRgn(0, 0, 0, 0);
            for &[left, top, right, bottom] in &pixels {
                let part = CreateRectRgn(left, top, right, bottom);
                CombineRgn(Some(region), Some(region), Some(part), RGN_OR);
                let _ = DeleteObject(part.into());
            }
            SetWindowRgn(self.window, Some(region), true);
        }
        self.active_areas = pixels;
    }
}
