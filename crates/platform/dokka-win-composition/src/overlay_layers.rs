use dokka_scene::Overlay;
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::dot_layer::DotLayer;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::text_layer::TextLayer;

/// Elementos visuales de un panel flotante.
pub(crate) struct OverlayLayers {
    panel: PanelLayer,
    dots: Vec<DotLayer>,
    texts: Vec<TextLayer>,
}

impl OverlayLayers {
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        overlay: &Overlay,
    ) -> Result<Self> {
        Ok(Self {
            panel: PanelLayer::add(compositor, parent, &overlay.panel)?,
            dots: overlay
                .dots
                .iter()
                .map(|dot| DotLayer::add(compositor, parent, dot))
                .collect::<Result<_>>()?,
            texts: overlay
                .texts
                .iter()
                .map(|_| TextLayer::add(compositor, parent))
                .collect::<Result<_>>()?,
        })
    }

    pub(crate) fn matches(&self, overlay: &Overlay) -> bool {
        self.dots.len() == overlay.dots.len() && self.texts.len() == overlay.texts.len()
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        overlay: &Overlay,
    ) -> Result<()> {
        self.panel.update(&overlay.panel)?;
        for (layer, dot) in self.dots.iter().zip(&overlay.dots) {
            layer.update(dot)?;
        }
        for (layer, text) in self.texts.iter_mut().zip(&overlay.texts) {
            layer.update(compositor, painter, text)?;
        }
        Ok(())
    }
}
