use windows::Win32::Graphics::Gdi::{
    CombineRgn, CreateRectRgn, DeleteObject, RGN_OR, SetWindowRgn,
};

use crate::Surface;

impl Surface {
    /// Limita la barra a la unión de rectángulos lógicos `[x, y, ancho, alto]`:
    /// fuera de ellos no se ve ni recibe el ratón. La ventana puede ser más
    /// alta que la franja reservada para dibujar el menú desplegado debajo.
    pub fn menubar_set_active_areas(&mut self, areas: &[[f32; 4]]) {
        if self.menubar.is_invalid() {
            return;
        }
        let scale = self.menubar_scale.max(1.0);
        let pixels: Vec<[i32; 4]> = areas
            .iter()
            .map(|&[x, y, width, height]| {
                [
                    (x * scale).floor() as i32,
                    (y * scale).floor() as i32,
                    ((x + width) * scale).ceil() as i32,
                    ((y + height) * scale).ceil() as i32,
                ]
            })
            .collect();
        if self.menubar_active == pixels {
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
            SetWindowRgn(self.menubar, Some(region), true);
        }
        self.menubar_active = pixels;
    }
}
