//! Tamaño lógico disponible para encoger el dock en vertical.

use crate::Surface;
use crate::place_window::work_area_pixels;

impl Surface {
    /// Alto y ancho lógicos del área donde vive el dock.
    /// `[0.0, 0.0]` si no se puede saber: quien llama no encoge (falla sola).
    pub fn work_area_logical(&self, over_taskbar: bool) -> [f32; 2] {
        let [width_px, height_px] = work_area_pixels(self.window, over_taskbar);
        if self.scale <= 0.0 || width_px <= 0 || height_px <= 0 {
            return [0.0, 0.0];
        }
        [width_px as f32 / self.scale, height_px as f32 / self.scale]
    }
}
