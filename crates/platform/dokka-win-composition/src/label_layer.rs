use dokka_scene::Label;
use windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::painter::Painter;

/// Etiqueta ya pintada; solo se repinta si cambia su texto.
pub(crate) struct LabelLayer {
    visual: SpriteVisual,
    text: Option<String>,
    size: Vector2,
}

impl LabelLayer {
    pub(crate) fn add(compositor: &Compositor, parent: &ContainerVisual) -> Result<Self> {
        let visual = compositor.CreateSpriteVisual()?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self {
            visual,
            text: None,
            size: Vector2::default(),
        })
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        label: &Label,
    ) -> Result<()> {
        if self.text.as_ref() != Some(&label.text) {
            let (surface, size) = painter.paint_label(label)?;
            self.visual
                .SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
            self.visual.SetSize(size)?;
            self.size = size;
            self.text = Some(label.text.clone());
        }
        self.visual.SetOffset(Vector3 {
            // No dejar que se corte por el borde izquierdo.
            X: (label.center_x - self.size.X / 2.0).max(0.0),
            Y: label.bottom - self.size.Y,
            Z: 0.0,
        })
    }
}
