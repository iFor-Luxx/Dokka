use dokka_scene::Scene;
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::label_layer::LabelLayer;
use crate::overlay_layers::OverlayLayers;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;

/// Elementos visuales creados para una escena.
pub(crate) struct SceneLayers {
    panel: PanelLayer,
    dividers: Vec<DividerLayer>,
    icons: Vec<IconLayer>,
    dots: Vec<DotLayer>,
    labels: Vec<LabelLayer>,
    overlay: Option<OverlayLayers>,
}

impl SceneLayers {
    pub(crate) fn build(
        compositor: &Compositor,
        painter: &Painter,
        root: &ContainerVisual,
        scene: &Scene,
    ) -> Result<Self> {
        root.Children()?.RemoveAll()?;
        let panel = PanelLayer::add(compositor, root, &scene.panel)?;
        let dividers = scene
            .dividers
            .iter()
            .map(|divider| DividerLayer::add(compositor, root, divider))
            .collect::<Result<_>>()?;
        let icons = scene
            .icons
            .iter()
            .map(|icon| IconLayer::add(compositor, painter, root, icon))
            .collect::<Result<_>>()?;
        let dots = scene
            .dots
            .iter()
            .map(|dot| DotLayer::add(compositor, root, dot))
            .collect::<Result<_>>()?;
        let overlay = scene
            .overlay
            .as_ref()
            .map(|overlay| OverlayLayers::add(compositor, root, overlay))
            .transpose()?;
        // Van al final para quedar por encima de los iconos.
        let labels = scene
            .labels
            .iter()
            .map(|_| LabelLayer::add(compositor, root))
            .collect::<Result<_>>()?;
        Ok(Self {
            panel,
            dividers,
            icons,
            dots,
            labels,
            overlay,
        })
    }

    /// `true` si la escena tiene la misma forma y basta con actualizar.
    pub(crate) fn matches(&self, scene: &Scene) -> bool {
        let same_overlay = match (&self.overlay, &scene.overlay) {
            (Some(layers), Some(overlay)) => layers.matches(overlay),
            (None, None) => true,
            _ => false,
        };
        same_overlay
            && self.dots.len() == scene.dots.len()
            && self.dividers.len() == scene.dividers.len()
            && self.labels.len() == scene.labels.len()
            && self.icons.len() == scene.icons.len()
            && self
                .icons
                .iter()
                .zip(&scene.icons)
                .all(|(layer, icon)| layer.accepts(icon))
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        scene: &Scene,
    ) -> Result<()> {
        self.panel.update(&scene.panel)?;
        for (layer, divider) in self.dividers.iter().zip(&scene.dividers) {
            layer.update(divider)?;
        }
        for (layer, icon) in self.icons.iter_mut().zip(&scene.icons) {
            layer.update(compositor, painter, icon)?;
        }
        for (layer, dot) in self.dots.iter().zip(&scene.dots) {
            layer.update(dot)?;
        }
        if let (Some(layers), Some(overlay)) = (&mut self.overlay, &scene.overlay) {
            layers.update(compositor, painter, overlay)?;
        }
        for (layer, label) in self.labels.iter_mut().zip(&scene.labels) {
            layer.update(compositor, painter, label)?;
        }
        Ok(())
    }
}
