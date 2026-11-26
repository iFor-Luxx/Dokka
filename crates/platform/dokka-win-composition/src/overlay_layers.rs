use dokka_scene::{Overlay, Panel};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::text_layer::TextLayer;

/// Elementos visuales de un panel flotante.
/// Orden: panel, resaltado, líneas, puntos, iconos (punta), textos.
pub(crate) struct OverlayLayers {
    panel: PanelLayer,
    /// Apariencia con la que se creó el fondo.
    style: Panel,
    highlight: Option<PanelLayer>,
    /// Estilo con el que se creó el resaltado, si lo hay.
    highlight_style: Option<Panel>,
    dividers: Vec<DividerLayer>,
    dots: Vec<DotLayer>,
    icons: Vec<IconLayer>,
    texts: Vec<TextLayer>,
}

impl OverlayLayers {
    pub(crate) fn add(
        compositor: &Compositor,
        painter: &Painter,
        parent: &ContainerVisual,
        overlay: &Overlay,
    ) -> Result<Self> {
        let mut highlight = None;
        if let Some(panel) = &overlay.highlight {
            highlight = Some(PanelLayer::add(compositor, parent, panel)?);
        }
        Ok(Self {
            panel: PanelLayer::add(compositor, parent, &overlay.panel)?,
            style: overlay.panel,
            highlight,
            highlight_style: overlay.highlight,
            dividers: overlay
                .dividers
                .iter()
                .map(|divider| DividerLayer::add(compositor, parent, divider))
                .collect::<Result<_>>()?,
            dots: overlay
                .dots
                .iter()
                .map(|dot| DotLayer::add(compositor, parent, dot))
                .collect::<Result<_>>()?,
            icons: overlay
                .icons
                .iter()
                .map(|icon| IconLayer::add(compositor, painter, parent, icon))
                .collect::<Result<_>>()?,
            texts: overlay
                .texts
                .iter()
                .map(|_| TextLayer::add(compositor, parent))
                .collect::<Result<_>>()?,
        })
    }

    /// Un panel de otro color (calendario frente a vista previa) se crea de nuevo.
    /// El resaltado del menú tiene estilo constante, así que moverse por él no reconstruye.
    pub(crate) fn matches(&self, overlay: &Overlay) -> bool {
        self.style.tint == overlay.panel.tint
            && self.highlight.is_some() == overlay.highlight.is_some()
            && self.highlight_style.map(highlight_style_of)
                == overlay.highlight.map(highlight_style_of)
            && self.dividers.len() == overlay.dividers.len()
            && self.dots.len() == overlay.dots.len()
            && self.icons.len() == overlay.icons.len()
            && self.texts.len() == overlay.texts.len()
            && self
                .icons
                .iter()
                .zip(&overlay.icons)
                .all(|(layer, icon)| layer.accepts(icon))
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        overlay: &Overlay,
    ) -> Result<()> {
        self.panel.update(&overlay.panel)?;
        if let (Some(layer), Some(panel)) = (&self.highlight, &overlay.highlight) {
            layer.update(panel)?;
        }
        for (layer, divider) in self.dividers.iter().zip(&overlay.dividers) {
            layer.update(divider)?;
        }
        for (layer, dot) in self.dots.iter().zip(&overlay.dots) {
            layer.update(dot)?;
        }
        for (layer, icon) in self.icons.iter_mut().zip(&overlay.icons) {
            layer.update(compositor, painter, icon)?;
        }
        for (layer, text) in self.texts.iter_mut().zip(&overlay.texts) {
            layer.update(compositor, painter, text)?;
        }
        Ok(())
    }
}

/// Lo que fija el aspecto del resaltado al crearse: `update` no lo cambia.
fn highlight_style_of(panel: Panel) -> (dokka_scene::Color, f32) {
    (panel.tint, panel.corner_radius)
}
