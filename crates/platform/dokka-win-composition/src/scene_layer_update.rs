//! Reutilización y actualización de las capas de una escena.
//!
//! `matches` dice si la escena que llega tiene la misma forma y basta con
//! actualizar (`update`); si no, `SceneLayers::build` reconstruye. La
//! opacidad, la escala y el desenfoque no reconstruyen: los aplica `update`.
//! Los iconos, textos, puntos, líneas y etiquetas pueden encoger: sobran
//! capas que se ocultan sin reconstruir (`layer_sync`); si crecen, se
//! reconstruye una vez y la reserva ya cubre las vueltas.

use dokka_scene::Scene;
use windows::UI::Composition::Compositor;
use windows::core::Result;

use crate::header_veil_layer::veil_matches;
use crate::layer_sync::{
    sync_cursors, sync_dividers, sync_dots, sync_icons, sync_labels, sync_texts,
};
use crate::painter::Painter;
use crate::scene_layers::{SceneLayers, highlight_style_of, panel_style_of};

impl SceneLayers {
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
            && self.panel_style == panel_style_of(&scene.panel)
            && self.panel_shadow_spec == scene.panel_shadow
            && veil_matches(&self.veil, &scene.header_veil)
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
            && self.cursors.len() >= scene.cursors.len()
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
        if let (Some(layer), Some(shadow)) = (&mut self.panel_shadow, &scene.panel_shadow) {
            let _ = layer.update(compositor, &scene.panel, shadow, &[]);
        }
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
        sync_cursors(&mut self.cursors, compositor, painter, &scene.cursors)?;
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
        if let (Some(layer), Some(veil)) = (&mut self.veil, &scene.header_veil) {
            layer.update(compositor, painter, &scene.panel, veil)?;
        }
        self.apply_stage(compositor, scene)?;
        Ok(())
    }
}
