use dokka_scene::{IconImage, Label, Text};
use dokka_win_text::TextPainter;
use windows::UI::Composition::{CompositionDrawingSurface, CompositionGraphicsDevice, Compositor};
use windows::core::Result;
use windows_numerics::Vector2;

use crate::graphics_device::create_graphics_device;
use crate::paint_bitmap::paint_bitmap;
use crate::paint_drawing::paint_drawing;
use crate::paint_label::paint_label;
use crate::paint_text::paint_text;

/// Convierte contenido en superficies que el compositor puede mostrar.
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

    pub(crate) fn paint_text(&self, text: &Text) -> Result<(CompositionDrawingSurface, Vector2)> {
        paint_text(&self.device, &self.text, text)
    }

    /// Superficie de la etiqueta y su tamaño lógico.
    pub(crate) fn paint_label(
        &self,
        label: &Label,
    ) -> Result<(CompositionDrawingSurface, Vector2)> {
        paint_label(&self.device, &self.text, label)
    }
}
