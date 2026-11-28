//! Reutilización y actualización de las capas de un panel flotante.
//!
//! `matches` dice si la escena que llega tiene la misma forma y basta con
//! actualizar (`update`); si no, `OverlayLayers::add` reconstruye. La
//! opacidad, la posición y la escala no reconstruyen: las aplica `update`.
//! Durante la salida por piezas no se llama a `update`: las anima
//! Composition y al terminar o cancelar se reconstruye limpio, así que
//! ninguna capa reutilizada queda alterada.

use dokka_scene::Overlay;
use windows::UI::Composition::Compositor;
use windows::core::Result;

use crate::overlay_layers::OverlayLayers;
use crate::overlay_style::{apply_scale, has_glass, highlight_style_of, pill_style_of};
use crate::painter::Painter;
use crate::pointer_layer::pointer_style_of;
use crate::shadow_layer::others_of;

impl OverlayLayers {
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
}
