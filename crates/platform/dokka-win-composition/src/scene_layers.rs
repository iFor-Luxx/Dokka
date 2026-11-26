use std::sync::Arc;

use dokka_scene::Scene;
use windows::UI::Composition::{CompositionGraphicsDevice, Compositor, ContainerVisual};
use windows::core::Result;

use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::panel_layer::PanelLayer;

/// Elementos visuales creados para una escena.
pub(crate) struct SceneLayers {
    panel: PanelLayer,
    icons: Vec<IconLayer>,
    dots: Vec<DotLayer>,
}

impl SceneLayers {
    pub(crate) fn build(
        compositor: &Compositor,
        device: &CompositionGraphicsDevice,
        root: &ContainerVisual,
        scene: &Scene,
    ) -> Result<Self> {
        root.Children()?.RemoveAll()?;
        let panel = PanelLayer::add(compositor, root, &scene.panel)?;
        let icons = scene
            .icons
            .iter()
            .map(|icon| IconLayer::add(compositor, device, root, icon))
            .collect::<Result<_>>()?;
        let dots = scene
            .dots
            .iter()
            .map(|dot| DotLayer::add(compositor, root, dot))
            .collect::<Result<_>>()?;
        Ok(Self { panel, icons, dots })
    }

    /// `true` si la escena usa las mismas imágenes y puede solo recolocarse.
    pub(crate) fn matches(&self, scene: &Scene) -> bool {
        self.dots.len() == scene.dots.len()
            && self.icons.len() == scene.icons.len()
            && self.icons.iter().zip(&scene.icons).all(|(layer, icon)| {
                match (&layer.image, &icon.image) {
                    (Some(old), Some(new)) => Arc::ptr_eq(old, new),
                    (None, None) => true,
                    _ => false,
                }
            })
    }

    pub(crate) fn update(&self, scene: &Scene) -> Result<()> {
        self.panel.update(&scene.panel)?;
        for (layer, icon) in self.icons.iter().zip(&scene.icons) {
            layer.update(icon)?;
        }
        for (layer, dot) in self.dots.iter().zip(&scene.dots) {
            layer.update(dot)?;
        }
        Ok(())
    }
}
