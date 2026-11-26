use dokka_scene::{Color, Panel, Scene};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::blur_stage::BlurStage;
use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::label_layer::LabelLayer;
use crate::layer_sync::{sync_dividers, sync_dots, sync_icons, sync_labels, sync_texts};
use crate::overlay_layers::OverlayLayers;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::text_layer::TextLayer;

/// Elementos visuales creados para una escena.
///
/// El fondo cuelga de un contenedor y el resto del contenido de otro: el fondo
/// solo se desvanece y el contenido además escala y se desenfoca, sin
/// reconstruir capas. El contenido cuelga de un `holder` propio en su mismo
/// sitio: con desenfoque activo el `holder` se oculta y solo se ve la copia
/// desenfocada, sin que la captura pierda nitidez.
/// Cada capa se inserta arriba: el fondo queda bajo el contenido, y dentro del
/// contenido el orden es paneles, planos, resaltado, líneas, iconos, puntos y textos.
pub(crate) struct SceneLayers {
    backdrop: ContainerVisual,
    holder: ContainerVisual,
    content: ContainerVisual,
    root: ContainerVisual,
    blur: BlurStage,
    panel: PanelLayer,
    panels: Vec<PanelLayer>,
    panel_styles: Vec<PanelStyle>,
    flats: Vec<PanelLayer>,
    flat_styles: Vec<PanelStyle>,
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
        // El fondo se crea primero para quedar debajo del contenido. El
        // contenido cuelga del `holder`, en su sitio del orden de pintado.
        let backdrop = compositor.CreateContainerVisual()?;
        root.Children()?.InsertAtTop(&backdrop)?;
        let holder = compositor.CreateContainerVisual()?;
        root.Children()?.InsertAtTop(&holder)?;
        let content = compositor.CreateContainerVisual()?;
        holder.Children()?.InsertAtTop(&content)?;
        let panel = PanelLayer::add(compositor, &backdrop, &scene.panel)?;
        // Cada capa se inserta arriba: los paneles quedan sobre el fondo y
        // bajo el resaltado, las líneas, los iconos, los puntos y los textos.
        // Los planos van encima de los paneles: son tinte sin cristal sobre el
        // cristal de la misma ventana (buscador, selección y hover de
        // Preferencias). El resto de escenas no los usa.
        let panels = scene
            .panels
            .iter()
            .map(|panel| PanelLayer::add(compositor, &content, panel))
            .collect::<Result<_>>()?;
        let flats = scene
            .flat_panels
            .iter()
            .map(|panel| PanelLayer::add_flat(compositor, &content, panel))
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
        let mut layers = Self {
            backdrop,
            holder,
            content,
            root: root.clone(),
            blur: BlurStage::new(),
            panel,
            panels,
            panel_styles: scene.panels.iter().map(panel_style_of).collect(),
            flats,
            flat_styles: scene.flat_panels.iter().map(panel_style_of).collect(),
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
        layers.apply_stage(compositor, scene)?;
        Ok(layers)
    }

    /// `true` si la escena tiene la misma forma y basta con actualizar.
    /// La opacidad, la escala y el desenfoque no reconstruyen: los aplica `update`.
    /// Los iconos, textos, puntos, líneas y etiquetas pueden encoger: sobran
    /// capas que se ocultan sin reconstruir; si crecen, se reconstruye una vez
    /// y la reserva ya cubre las vueltas. Así cambiar de página o escribir en
    /// el Launchpad no rehace cristales ni el desenfoque.
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
            // Lo plano y el cristal van en listas distintas: un cambio de un
            // lado al otro reconstruye en vez de reutilizar el cristal.
            && self.flat_styles.len() == scene.flat_panels.len()
            && self
                .flat_styles
                .iter()
                .zip(&scene.flat_panels)
                .all(|(style, panel)| *style == panel_style_of(panel))
            && self.dots.len() >= scene.dots.len()
            && self.dividers.len() >= scene.dividers.len()
            && self.texts.len() >= scene.texts.len()
            && self.labels.len() >= scene.labels.len()
            && self.icons.len() >= scene.icons.len()
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
        for (layer, panel) in self.flats.iter().zip(&scene.flat_panels) {
            layer.update(panel)?;
        }
        if let (Some(layer), Some(panel)) = (&self.highlight, &scene.highlight) {
            layer.update(panel)?;
        }
        sync_dividers(&self.dividers, &scene.dividers)?;
        sync_icons(&mut self.icons, compositor, painter, &scene.icons)?;
        sync_dots(&mut self.dots, compositor, &scene.dots)?;
        sync_texts(&mut self.texts, compositor, painter, &scene.texts)?;
        if let (Some(layers), Some(overlay)) = (&mut self.overlay, &scene.overlay) {
            layers.update(compositor, painter, overlay)?;
        }
        if let (Some(layers), Some(submenu)) = (&mut self.submenu, &scene.submenu) {
            layers.update(compositor, painter, submenu)?;
        }
        sync_labels(&mut self.labels, compositor, painter, &scene.labels)?;
        self.apply_stage(compositor, scene)?;
        Ok(())
    }

    /// Desvanece el fondo y deja escala, opacidad y desenfoque del contenido en
    /// manos de `BlurStage`: en reposo lo deja normal y sin efectos activos, y
    /// en transición muestra la copia desenfocada. Sin desenfoque ni efecto
    /// disponible, aplica el zoom y el desvanecido clásicos.
    fn apply_stage(&mut self, compositor: &Compositor, scene: &Scene) -> Result<()> {
        let backdrop = self.backdrop.clone();
        let holder = self.holder.clone();
        let content = self.content.clone();
        let root = self.root.clone();
        self.blur
            .apply(compositor, &root, &backdrop, &holder, &content, scene)
    }
}

/// Lo que fija el aspecto del resaltado al crearse.
fn highlight_style_of(panel: Panel) -> (dokka_scene::Color, f32) {
    (panel.tint, panel.corner_radius)
}
