use dokka_scene::{Overlay, Panel, Shadow};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::cursor_layer::CursorLayer;
use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::overlay_dismiss::PieceGroups;
use crate::overlay_style::{
    apply_scale, has_glass, highlight_style_of, panel_layer, pill_style_of,
};
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::pointer_layer::{PointerLayer, pointer_style_of};
use crate::shadow_layer::ShadowLayer;
use crate::shadow_layer::others_of;
use crate::text_layer::TextLayer;

/// Elementos visuales de un panel flotante.
/// Orden: panel, píldora, punta, resaltado, líneas, puntos, textos, iconos.
/// Todo cuelga de un contenedor para fundir el panel sin tocar el dock ni las etiquetas.
/// El fondo puede no tener cristal: cuando el tinte y el borde del `Overlay.panel`
/// son transparentes (alfa 0, como el centro de control y el buscador) no se crea
/// ninguna capa para no ver un rectángulo borroso; el resto de paneles no cambia.
pub(crate) struct OverlayLayers {
    container: ContainerVisual,
    pieces: PieceGroups,
    panel: Option<PanelLayer>,
    panel_shadow: Option<ShadowLayer>,
    panel_shadow_spec: Option<Shadow>,
    tiles: Vec<PanelLayer>,
    /// Sombras bajo cada pieza (`None` si no se pidió o falló).
    tile_shadows: Vec<Option<ShadowLayer>>,
    /// Estilo con el que se creó cada pieza.
    tile_styles: Vec<Panel>,
    /// Sombra pedida bajo las piezas, si hay.
    tile_shadow: Option<Shadow>,
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
    cursors: Vec<CursorLayer>,
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
        // La sombra del fondo excluye las piezas; las de cada pieza excluyen a
        // las demás para no oscurecerlas. Si fallan, se ve sin ellas.
        let panel_shadow_spec = overlay.panel_shadow;
        let panel_shadow = panel_shadow_spec.and_then(|shadow| {
            ShadowLayer::add(
                compositor,
                &container,
                &overlay.panel,
                &shadow,
                &overlay.tiles,
            )
            .ok()
        });
        // Cada pieza lleva lo suyo en su mismo contenedor para volar entera.
        let (pieces, map) = PieceGroups::build(compositor, &container, overlay)?;
        // Las piezas llevan filo de luz y brillo alto (Liquid Glass).
        let tiles = overlay
            .tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                PanelLayer::add_tile(compositor, pieces.parent(&container, index), tile)
            })
            .collect::<Result<Vec<_>>>()?;
        let tile_shadows = overlay
            .tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                overlay.tile_shadow.and_then(|shadow| {
                    ShadowLayer::add(
                        compositor,
                        pieces.parent(&container, index),
                        tile,
                        &shadow,
                        &others_of(&overlay.tiles, index),
                    )
                    .ok()
                })
            })
            .collect();
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
            .enumerate()
            .map(|(index, panel)| {
                PanelLayer::add_flat(
                    compositor,
                    pieces.flat_parent(&container, &map, index),
                    panel,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let dividers = overlay
            .dividers
            .iter()
            .enumerate()
            .map(|(index, divider)| {
                DividerLayer::add(
                    compositor,
                    pieces.divider_parent(&container, &map, index),
                    divider,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let cursors = overlay
            .cursors
            .iter()
            .enumerate()
            .map(|(index, cursor)| {
                CursorLayer::add(
                    compositor,
                    pieces.cursor_parent(&container, &map, index),
                    cursor,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let dots = overlay
            .dots
            .iter()
            .enumerate()
            .map(|(index, dot)| {
                DotLayer::add(compositor, pieces.dot_parent(&container, &map, index), dot)
            })
            .collect::<Result<Vec<_>>>()?;
        let texts = overlay
            .texts
            .iter()
            .enumerate()
            .map(|(index, _)| {
                TextLayer::add(compositor, pieces.text_parent(&container, &map, index))
            })
            .collect::<Result<Vec<_>>>()?;
        // Encima del resaltado y junto a los textos: no se solapan con ellos.
        let icons = overlay
            .icons
            .iter()
            .enumerate()
            .map(|(index, icon)| {
                IconLayer::add(
                    compositor,
                    painter,
                    pieces.icon_parent(&container, &map, index),
                    icon,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            container,
            pieces,
            panel,
            panel_shadow,
            panel_shadow_spec,
            tiles,
            tile_shadows,
            tile_styles: overlay.tiles.clone(),
            tile_shadow: overlay.tile_shadow,
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
            cursors,
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
            && self.panel_shadow_spec == overlay.panel_shadow
            && self.style.tint == overlay.panel.tint
            && self.tile_shadow == overlay.tile_shadow
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
            && self.cursors.len() == overlay.cursors.len()
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
        if let (Some(layer), Some(shadow)) = (&mut self.panel_shadow, &overlay.panel_shadow) {
            let _ = layer.update(compositor, &overlay.panel, shadow, &overlay.tiles);
        }
        for (layer, tile) in self.tiles.iter().zip(&overlay.tiles) {
            layer.update(tile)?;
        }
        for (index, (shadow, tile)) in self.tile_shadows.iter_mut().zip(&overlay.tiles).enumerate()
        {
            if let (Some(layer), Some(shadow)) = (shadow, &overlay.tile_shadow) {
                let _ = layer.update(compositor, tile, shadow, &others_of(&overlay.tiles, index));
            }
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
        for (layer, cursor) in self.cursors.iter_mut().zip(&overlay.cursors) {
            layer.update(compositor, painter, cursor)?;
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

    /// Lanza la salida por piezas con la orden de la isla. Falla si las
    /// piezas ya no coinciden: el shell cierra sin animación.
    pub(crate) fn start_dismiss(
        &self,
        compositor: &Compositor,
        dismiss: &dokka_scene::ControlDismiss,
    ) -> Result<()> {
        self.pieces.start(compositor, &self.tile_styles, dismiss)
    }
}
