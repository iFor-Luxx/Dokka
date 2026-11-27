use dokka_scene::{Overlay, Panel};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;
use windows_numerics::Vector3;

use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::pointer_layer::{PointerLayer, pointer_style_of};
use crate::text_layer::TextLayer;

/// Elementos visuales de un panel flotante.
/// Orden: panel, píldora, punta, resaltado, líneas, puntos, textos, iconos.
/// Todo cuelga de un contenedor para fundir el panel sin tocar el dock ni las etiquetas.
/// El fondo puede no tener cristal: cuando el tinte y el borde del `Overlay.panel`
/// son transparentes (alfa 0, como el centro de control y el buscador) no se crea
/// ninguna capa para no ver un rectángulo borroso; el resto de paneles no cambia.
pub(crate) struct OverlayLayers {
    container: ContainerVisual,
    panel: Option<PanelLayer>,
    tiles: Vec<PanelLayer>,
    /// Estilo con el que se creó cada pieza.
    tile_styles: Vec<Panel>,
    /// Apariencia con la que se creó el fondo.
    style: Panel,
    pill: Option<PanelLayer>,
    /// Panel con el que se creó la píldora, si la hay.
    pill_style: Option<Panel>,
    pointer: Option<PointerLayer>,
    /// Altura y lado con los que se creó la punta, si la hay.
    pointer_style: Option<(f32, dokka_scene::PointerSide)>,
    highlight: Option<PanelLayer>,
    /// Estilo con el que se creó el resaltado, si lo hay.
    highlight_style: Option<Panel>,
    flats: Vec<PanelLayer>,
    /// Estilo con el que se creó cada plano.
    flat_styles: Vec<Panel>,
    dividers: Vec<DividerLayer>,
    dots: Vec<DotLayer>,
    texts: Vec<TextLayer>,
    icons: Vec<IconLayer>,
}

impl OverlayLayers {
    pub(crate) fn add(
        compositor: &Compositor,
        painter: &Painter,
        parent: &ContainerVisual,
        overlay: &Overlay,
    ) -> Result<Self> {
        let container = compositor.CreateContainerVisual()?;
        parent.Children()?.InsertAtTop(&container)?;
        container.SetOpacity(overlay.opacity)?;
        apply_scale(&container, overlay)?;
        let panel = panel_layer(compositor, &container, &overlay.panel)?;
        // Las piezas llevan filo de luz y brillo alto (Liquid Glass).
        let tiles = overlay
            .tiles
            .iter()
            .map(|tile| PanelLayer::add_tile(compositor, &container, tile))
            .collect::<Result<Vec<_>>>()?;
        // Encima del panel: el fondo de la píldora tapa ese tramo de tinte.
        let mut pill = None;
        if let Some(pill_panel) = &overlay.pill {
            pill = Some(PanelLayer::add(compositor, &container, pill_panel)?);
        }
        // Encima de la píldora para que su cristal tape ese tramo de borde y de tinte.
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
            // Plano, sin cristal: el tinte de la píldora es translúcido y el
            // cristal muestrearía el fondo y lo taparía. El tinte del menú
            // es opaco y se ve igual sin cristal.
            highlight = Some(PanelLayer::add_flat(
                compositor,
                &container,
                highlight_style,
            )?);
        }
        let flats = overlay
            .flat_panels
            .iter()
            .map(|panel| PanelLayer::add_flat(compositor, &container, panel))
            .collect::<Result<Vec<_>>>()?;
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
        // Encima del resaltado y junto a los textos: no se solapan con ellos.
        let icons = overlay
            .icons
            .iter()
            .map(|icon| IconLayer::add(compositor, painter, &container, icon))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            container,
            panel,
            tiles,
            tile_styles: overlay.tiles.clone(),
            style: overlay.panel,
            pill,
            pill_style: overlay.pill,
            pointer,
            pointer_style: overlay.pointer.map(pointer_style_of),
            highlight,
            highlight_style: overlay.highlight,
            flats,
            flat_styles: overlay.flat_panels.clone(),
            dividers,
            dots,
            texts,
            icons,
        })
    }

    /// Un panel de otro color (calendario frente a vista previa) se crea de nuevo.
    /// El resaltado del menú tiene estilo constante, así que moverse por él no reconstruye;
    /// la punta se mueve con `update`. La opacidad, la posición y la escala no reconstruyen.
    /// Un fondo transparente tampoco reconstruye a uno con cristal: el cristal
    /// se decide por el alfa del tinte y del borde, como al crearse.
    pub(crate) fn matches(&self, overlay: &Overlay) -> bool {
        self.panel.is_some() == has_glass(&overlay.panel)
            && self.style.tint == overlay.panel.tint
            && self.pill.is_some() == overlay.pill.is_some()
            && self.pill_style.map(pill_style_of) == overlay.pill.map(pill_style_of)
            && self.pointer.is_some() == overlay.pointer.is_some()
            && self.pointer_style == overlay.pointer.map(pointer_style_of)
            && self.highlight.is_some() == overlay.highlight.is_some()
            && self.highlight_style.map(highlight_style_of)
                == overlay.highlight.map(highlight_style_of)
            && self.tile_styles.len() == overlay.tiles.len()
            && self
                .tile_styles
                .iter()
                .zip(&overlay.tiles)
                .all(|(old, new)| pill_style_of(*old) == pill_style_of(*new))
            && self.flat_styles.len() == overlay.flat_panels.len()
            && self
                .flat_styles
                .iter()
                .zip(&overlay.flat_panels)
                .all(|(old, new)| pill_style_of(*old) == pill_style_of(*new))
            && self.dividers.len() == overlay.dividers.len()
            && self.dots.len() == overlay.dots.len()
            && self.texts.len() == overlay.texts.len()
            && self.icons.len() == overlay.icons.len()
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
        self.container.SetOpacity(overlay.opacity)?;
        apply_scale(&self.container, overlay)?;
        if let Some(layer) = &self.panel {
            layer.update(&overlay.panel)?;
        }
        for (layer, tile) in self.tiles.iter().zip(&overlay.tiles) {
            layer.update(tile)?;
        }
        if let (Some(layer), Some(pill)) = (&self.pill, &overlay.pill) {
            layer.update(pill)?;
        }
        if let (Some(layer), Some(pointer)) = (&self.pointer, &overlay.pointer) {
            layer.update(&overlay.panel, pointer)?;
        }
        if let (Some(layer), Some(panel)) = (&self.highlight, &overlay.highlight) {
            layer.update(panel)?;
        }
        for (layer, panel) in self.flats.iter().zip(&overlay.flat_panels) {
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
        for (layer, icon) in self.icons.iter_mut().zip(&overlay.icons) {
            layer.update(compositor, painter, icon)?;
        }
        Ok(())
    }
}

/// `true` si el fondo lleva cristal: tinte o borde con alfa visible.
/// Transparente (los dos a 0) no crea capa, como el centro y el buscador.
fn has_glass(panel: &Panel) -> bool {
    panel.tint.alpha != 0 || panel.border.alpha != 0
}

/// Crea el cristal del fondo, salvo que sea transparente.
fn panel_layer(
    compositor: &Compositor,
    parent: &ContainerVisual,
    panel: &Panel,
) -> Result<Option<PanelLayer>> {
    if has_glass(panel) {
        PanelLayer::add(compositor, parent, panel).map(Some)
    } else {
        Ok(None)
    }
}

/// Lo que fija el aspecto de la píldora al crearse: `update` no lo cambia.
fn pill_style_of(panel: Panel) -> (dokka_scene::Color, dokka_scene::Color, f32, f32) {
    (
        panel.tint,
        panel.border,
        panel.corner_radius,
        panel.border_width,
    )
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
