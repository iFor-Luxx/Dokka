use dokka_scene::IconImage;
use dokka_win_text::TextPainter;
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice, Compositor};
use windows::core::Result;

use crate::graphics_device::create_graphics_device;
use crate::paint_bitmap::paint_bitmap;
use crate::paint_drawing::paint_drawing;

/// Convierte el contenido de un icono en una superficie que el compositor puede mostrar.
pub(crate) struct Painter {
    device: CompositionGraphicsDevice,
    text: TextPainter,
}

impl Painter {
    pub(crate) fn new(compositor: &Compositor) -> Result<Self> {
        Ok(Self {
            device: create_graphics_device(compositor)?,
            text: TextPainter::new()?,
        })
    }

    pub(crate) fn paint(&self, image: &IconImage) -> Result<CompositionDrawingSurface> {
        match image {
            IconImage::Pixels(bitmap) => paint_bitmap(&self.device, bitmap),
            IconImage::Drawing(drawing) => paint_drawing(&self.device, &self.text, drawing),
        }
    }
}
