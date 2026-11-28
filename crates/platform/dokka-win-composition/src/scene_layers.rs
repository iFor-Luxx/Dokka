use dokka_scene::{Color, Panel, Scene, Shadow};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::blur_stage::BlurStage;
use crate::cursor_layer::CursorLayer;
use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::label_layer::LabelLayer;
use crate::overlay_layers::OverlayLayers;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::shadow_layer::ShadowLayer;
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
    pub(crate) panel: PanelLayer,
    /// Estilo con el que se creó el fondo.
    pub(crate) panel_style: PanelStyle,
    /// Sombra bajo el fondo, si se pidió (Preferencias). Falla sola.
    pub(crate) panel_shadow: Option<ShadowLayer>,
    /// Sombra pedida para el fondo, exista o no la capa.
    pub(crate) panel_shadow_spec: Option<Shadow>,
    pub(crate) panels: Vec<PanelLayer>,
    pub(crate) panel_styles: Vec<PanelStyle>,
    pub(crate) flats: Vec<PanelLayer>,
    pub(crate) flat_styles: Vec<PanelStyle>,
    pub(crate) highlight: Option<PanelLayer>,
    pub(crate) highlight_style: Option<(dokka_scene::Color, f32)>,
    pub(crate) dividers: Vec<DividerLayer>,
    pub(crate) cursors: Vec<CursorLayer>,
    pub(crate) icons: Vec<IconLayer>,
    pub(crate) dots: Vec<DotLayer>,
    pub(crate) texts: Vec<TextLayer>,
    pub(crate) labels: Vec<LabelLayer>,
    pub(crate) overlay: Option<OverlayLayers>,
    pub(crate) submenu: Option<OverlayLayers>,
}

/// Lo que fija el aspecto de un panel extra al crearse: `update` no lo reconstruye.
type PanelStyle = (Color, Color, f32, f32);

pub(crate) fn panel_style_of(panel: &Panel) -> PanelStyle {
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
        // La sombra va debajo del cristal: se crea antes para quedar al fondo.
        // Si no se puede crear, el fondo se ve sin ella.
        let panel_shadow_spec = scene.panel_shadow;
        let panel_shadow = match scene.panel_shadow {
            Some(shadow) => {
                ShadowLayer::add(compositor, &backdrop, &scene.panel, &shadow, &[]).ok()
            }
            None => None,
        };
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
        let cursors = scene
            .cursors
            .iter()
            .map(|cursor| CursorLayer::add(compositor, &content, cursor))
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
            panel_style: panel_style_of(&scene.panel),
            panel_shadow,
            panel_shadow_spec,
            panels,
            panel_styles: scene.panels.iter().map(panel_style_of).collect(),
            flats,
            flat_styles: scene.flat_panels.iter().map(panel_style_of).collect(),
            highlight,
            highlight_style: scene.highlight.map(highlight_style_of),
            dividers,
            cursors,
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

    /// Salida del centro por piezas en su overlay, una sola vez.
    pub(crate) fn start_control_dismiss(
        &self,
        compositor: &Compositor,
        dismiss: &dokka_scene::ControlDismiss,
    ) -> Result<()> {
        let Some(overlay) = &self.overlay else {
            return Err(windows::core::Error::from_thread());
        };
        overlay.start_dismiss(compositor, dismiss)
    }

    /// Desvanece el fondo y deja escala, opacidad y desenfoque del contenido en
    /// manos de `BlurStage`: en reposo lo deja normal y sin efectos activos, y
    /// en transición muestra la copia desenfocada. Sin desenfoque ni efecto
    /// disponible, aplica el zoom y el desvanecido clásicos.
    pub(crate) fn apply_stage(&mut self, compositor: &Compositor, scene: &Scene) -> Result<()> {
        let backdrop = self.backdrop.clone();
        let holder = self.holder.clone();
        let content = self.content.clone();
        let root = self.root.clone();
        self.blur
            .apply(compositor, &root, &backdrop, &holder, &content, scene)
    }
}

/// Lo que fija el aspecto del resaltado al crearse.
pub(crate) fn highlight_style_of(panel: Panel) -> (dokka_scene::Color, f32) {
    (panel.tint, panel.corner_radius)
}
