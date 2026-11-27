//! Qué fija el aspecto de un panel flotante y cómo se escala.

use dokka_scene::{Overlay, Panel};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;
use windows_numerics::Vector3;

use crate::panel_layer::PanelLayer;

/// `true` si el fondo lleva cristal: tinte o borde con alfa visible.
/// Transparente (los dos a 0) no crea capa, como el centro y el buscador.
pub(crate) fn has_glass(panel: &Panel) -> bool {
    panel.tint.alpha != 0 || panel.border.alpha != 0
}

/// Crea el cristal del fondo, salvo que sea transparente.
pub(crate) fn panel_layer(
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
pub(crate) fn pill_style_of(panel: Panel) -> (dokka_scene::Color, dokka_scene::Color, f32, f32) {
    (
        panel.tint,
        panel.border,
        panel.corner_radius,
        panel.border_width,
    )
}

/// Lo que fija el aspecto del resaltado al crearse: `update` no lo cambia.
pub(crate) fn highlight_style_of(panel: Panel) -> (dokka_scene::Color, f32) {
    (panel.tint, panel.corner_radius)
}

/// Escala el contenedor sobre el icono, sin mover el resto de la escena.
pub(crate) fn apply_scale(container: &ContainerVisual, overlay: &Overlay) -> Result<()> {
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
