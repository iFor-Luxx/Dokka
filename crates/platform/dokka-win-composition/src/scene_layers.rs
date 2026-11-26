use dokka_scene::{Color, Panel, Scene};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;
use windows_numerics::Vector3;

use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::label_layer::LabelLayer;
use crate::overlay_layers::OverlayLayers;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::text_layer::TextLayer;

/// Elementos visuales creados para una escena.
///
/// El fondo cuelga de un contenedor y el resto del contenido de otro: el fondo
/// solo se desvanece y el contenido además escala, sin reconstruir capas.
/// Cada capa se inserta arriba: el fondo queda bajo el contenido, y dentro del
/// contenido el orden es paneles, resaltado, líneas, iconos, puntos y textos.
pub(crate) struct SceneLayers {
    backdrop: ContainerVisual,
    content: ContainerVisual,
    panel: PanelLayer,
    panels: Vec<PanelLayer>,
    panel_styles: Vec<PanelStyle>,
    highlight: Option<PanelLayer>,
    highlight_style: Option<(dokka_scene::Color, f32)>,
    dividers: Vec<DividerLayer>,
    icons: Vec<IconLayer>,
    dots: Vec<DotLayer>,
    texts: Vec<TextLayer>,
    labels: Vec<LabelLayer>,
    overlay: Option<OverlayLayers>,
    submenu: Option<OverlayLayers>,
}

/// Lo que fija el aspecto de un panel extra al crearse: `update` no lo reconstruye.
type PanelStyle = (Color, Color, f32, f32);

fn panel_style_of(panel: &Panel) -> PanelStyle {
    (
        panel.tint,
        panel.border,
        panel.corner_radius,
        panel.border_width,
    )
}

impl SceneLayers {
    pub(crate) fn build(
        compositor: &Compositor,
        painter: &Painter,
        root: &ContainerVisual,
        scene: &Scene,
    ) -> Result<Self> {
        root.Children()?.RemoveAll()?;
        // El fondo se crea primero para quedar debajo del contenido.
        let backdrop = compositor.CreateContainerVisual()?;
        root.Children()?.InsertAtTop(&backdrop)?;
        let content = compositor.CreateContainerVisual()?;
        root.Children()?.InsertAtTop(&content)?;
        let panel = PanelLayer::add(compositor, &backdrop, &scene.panel)?;
        // Cada capa se inserta arriba: los paneles quedan sobre el fondo y
        // bajo el resaltado, las líneas, los iconos, los puntos y los textos.
        let panels = scene
            .panels
            .iter()
            .map(|panel| PanelLayer::add(compositor, &content, panel))
            .collect::<Result<_>>()?;
        let highlight = scene
            .highlight
            .as_ref()
            .map(|panel| PanelLayer::add(compositor, &content, panel))
            .transpose()?;
        let dividers = scene
            .dividers
            .iter()
            .map(|divider| DividerLayer::add(compositor, &content, divider))
            .collect::<Result<_>>()?;
        let icons = scene
            .icons
            .iter()
            .map(|icon| IconLayer::add(compositor, painter, &content, icon))
            .collect::<Result<_>>()?;
        let dots = scene
            .dots
            .iter()
            .map(|dot| DotLayer::add(compositor, &content, dot))
            .collect::<Result<_>>()?;
        let texts = scene
            .texts
            .iter()
            .map(|_| TextLayer::add(compositor, &content))
            .collect::<Result<_>>()?;
        let overlay = scene
            .overlay
            .as_ref()
            .map(|overlay| OverlayLayers::add(compositor, painter, &content, overlay))
            .transpose()?;
        // El submenú reutiliza las mismas capas y va encima del menú.
        let submenu = scene
            .submenu
            .as_ref()
            .map(|submenu| OverlayLayers::add(compositor, painter, &content, submenu))
            .transpose()?;
        // Van al final para quedar por encima de los iconos.
        let labels = scene
            .labels
            .iter()
            .map(|_| LabelLayer::add(compositor, &content))
            .collect::<Result<_>>()?;
        let layers = Self {
            backdrop,
            content,
            panel,
            panels,
            panel_styles: scene.panels.iter().map(panel_style_of).collect(),
            highlight,
            highlight_style: scene.highlight.map(highlight_style_of),
            dividers,
            icons,
            dots,
            texts,
            labels,
            overlay,
            submenu,
        };
        layers.apply_stage(scene)?;
        Ok(layers)
    }

    /// `true` si la escena tiene la misma forma y basta con actualizar.
    /// La opacidad y la escala no reconstruyen: las aplica `update`.
    pub(crate) fn matches(&self, scene: &Scene) -> bool {
        let same_overlay = match (&self.overlay, &scene.overlay) {
            (Some(layers), Some(overlay)) => layers.matches(overlay),
            (None, None) => true,
            _ => false,
        };
        // El resaltado se mueve con `update`: cambiar de fila no reconstruye.
        let same_submenu = match (&self.submenu, &scene.submenu) {
            (Some(layers), Some(submenu)) => layers.matches(submenu),
            (None, None) => true,
            _ => false,
        };
        same_overlay
            && same_submenu
            && self.highlight.is_some() == scene.highlight.is_some()
            && self.highlight_style == scene.highlight.map(highlight_style_of)
            && self.panel_styles.len() == scene.panels.len()
            && self
                .panel_styles
                .iter()
                .zip(&scene.panels)
                .all(|(style, panel)| *style == panel_style_of(panel))
            && self.dots.len() == scene.dots.len()
            && self.dividers.len() == scene.dividers.len()
            && self.texts.len() == scene.texts.len()
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
        for (layer, panel) in self.panels.iter().zip(&scene.panels) {
            layer.update(panel)?;
        }
        if let (Some(layer), Some(panel)) = (&self.highlight, &scene.highlight) {
            layer.update(panel)?;
        }
        for (layer, divider) in self.dividers.iter().zip(&scene.dividers) {
            layer.update(divider)?;
        }
        for (layer, icon) in self.icons.iter_mut().zip(&scene.icons) {
            layer.update(compositor, painter, icon)?;
        }
        for (layer, dot) in self.dots.iter_mut().zip(&scene.dots) {
            layer.update(compositor, dot)?;
        }
        for (layer, text) in self.texts.iter_mut().zip(&scene.texts) {
            layer.update(compositor, painter, text)?;
        }
        if let (Some(layers), Some(overlay)) = (&mut self.overlay, &scene.overlay) {
            layers.update(compositor, painter, overlay)?;
        }
        if let (Some(layers), Some(submenu)) = (&mut self.submenu, &scene.submenu) {
            layers.update(compositor, painter, submenu)?;
        }
        for (layer, label) in self.labels.iter_mut().zip(&scene.labels) {
            layer.update(compositor, painter, label)?;
        }
        self.apply_stage(scene)?;
        Ok(())
    }

    /// Desvanece el fondo y el contenido, y escala el contenido desde el centro
    /// del fondo. Con opacidad y escala en reposo no cambia nada visible.
    fn apply_stage(&self, scene: &Scene) -> Result<()> {
        let bounds = scene.panel.bounds;
        self.backdrop.SetOpacity(scene.opacity)?;
        self.content.SetOpacity(scene.opacity)?;
        self.content.SetScale(Vector3 {
            X: scene.scale,
            Y: scene.scale,
            Z: 1.0,
        })?;
        self.content.SetCenterPoint(Vector3 {
            X: bounds.x + bounds.width / 2.0,
            Y: bounds.y + bounds.height / 2.0,
            Z: 0.0,
        })
    }
}

/// Lo que fija el aspecto del resaltado al crearse.
fn highlight_style_of(panel: Panel) -> (dokka_scene::Color, f32) {
    (panel.tint, panel.corner_radius)
}
