use dokka_scene::{Overlay, Panel};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;
use windows_numerics::Vector3;

use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::pointer_layer::{PointerLayer, pointer_style_of};
use crate::text_layer::TextLayer;

/// Elementos visuales de un panel flotante.
/// Orden: panel, punta, resaltado, líneas, puntos, textos.
/// Todo cuelga de un contenedor para fundir el panel sin tocar el dock ni las etiquetas.
pub(crate) struct OverlayLayers {
    container: ContainerVisual,
    panel: PanelLayer,
    /// Apariencia con la que se creó el fondo.
    style: Panel,
    pointer: Option<PointerLayer>,
    /// Altura con la que se creó la punta, si la hay.
    pointer_style: Option<f32>,
    highlight: Option<PanelLayer>,
    /// Estilo con el que se creó el resaltado, si lo hay.
    highlight_style: Option<Panel>,
    dividers: Vec<DividerLayer>,
    dots: Vec<DotLayer>,
    texts: Vec<TextLayer>,
}

impl OverlayLayers {
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        overlay: &Overlay,
    ) -> Result<Self> {
        let container = compositor.CreateContainerVisual()?;
        parent.Children()?.InsertAtTop(&container)?;
        container.SetOpacity(overlay.opacity)?;
        apply_scale(&container, overlay)?;
        let panel = PanelLayer::add(compositor, &container, &overlay.panel)?;
        // Encima del panel para que su cristal tape ese tramo de borde y de tinte.
        let mut pointer = None;
        if let Some(style) = &overlay.pointer {
            pointer = Some(PointerLayer::add(
                compositor,
                &container,
                &overlay.panel,
                style,
            )?);
        }
        let mut highlight = None;
        if let Some(highlight_style) = &overlay.highlight {
            highlight = Some(PanelLayer::add(compositor, &container, highlight_style)?);
        }
        let dividers = overlay
            .dividers
            .iter()
            .map(|divider| DividerLayer::add(compositor, &container, divider))
            .collect::<Result<Vec<_>>>()?;
        let dots = overlay
            .dots
            .iter()
            .map(|dot| DotLayer::add(compositor, &container, dot))
            .collect::<Result<Vec<_>>>()?;
        let texts = overlay
            .texts
            .iter()
            .map(|_| TextLayer::add(compositor, &container))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            container,
            panel,
            style: overlay.panel,
            pointer,
            pointer_style: overlay.pointer.map(pointer_style_of),
            highlight,
            highlight_style: overlay.highlight,
            dividers,
            dots,
            texts,
        })
    }

    /// Un panel de otro color (calendario frente a vista previa) se crea de nuevo.
    /// El resaltado del menú tiene estilo constante, así que moverse por él no reconstruye;
    /// la punta se mueve con `update`. La opacidad, la posición y la escala no reconstruyen.
    pub(crate) fn matches(&self, overlay: &Overlay) -> bool {
        self.style.tint == overlay.panel.tint
            && self.pointer.is_some() == overlay.pointer.is_some()
            && self.pointer_style == overlay.pointer.map(pointer_style_of)
            && self.highlight.is_some() == overlay.highlight.is_some()
            && self.highlight_style.map(highlight_style_of)
                == overlay.highlight.map(highlight_style_of)
            && self.dividers.len() == overlay.dividers.len()
            && self.dots.len() == overlay.dots.len()
            && self.texts.len() == overlay.texts.len()
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        overlay: &Overlay,
    ) -> Result<()> {
        self.container.SetOpacity(overlay.opacity)?;
        apply_scale(&self.container, overlay)?;
        self.panel.update(&overlay.panel)?;
        if let (Some(layer), Some(pointer)) = (&self.pointer, &overlay.pointer) {
            layer.update(&overlay.panel, pointer)?;
        }
        if let (Some(layer), Some(panel)) = (&self.highlight, &overlay.highlight) {
            layer.update(panel)?;
        }
        for (layer, divider) in self.dividers.iter().zip(&overlay.dividers) {
            layer.update(divider)?;
        }
        for (layer, dot) in self.dots.iter_mut().zip(&overlay.dots) {
            layer.update(compositor, dot)?;
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

/// Escala el contenedor sobre el icono, sin mover el resto de la escena.
fn apply_scale(container: &ContainerVisual, overlay: &Overlay) -> Result<()> {
    container.SetScale(Vector3 {
        X: overlay.scale,
        Y: overlay.scale,
        Z: 1.0,
    })?;
    container.SetCenterPoint(Vector3 {
        X: overlay.origin.0,
        Y: overlay.origin.1,
        Z: 0.0,
    })
}
