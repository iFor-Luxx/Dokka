use dokka_scene::{Color, Controls, Icon, Panel, Rect};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::icon_layer::IconLayer;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;

/// Botones dibujados en la ventana que va sobre las miniaturas.
pub(crate) struct ControlsLayer {
    panel: PanelLayer,
    panel_style: PanelStyle,
    icons: Vec<IconLayer>,
}

/// Lo que fija el aspecto del panel al crearse: `update` no lo reconstruye.
type PanelStyle = (Color, Color, f32, f32);

fn panel_style_of(panel: &Panel) -> PanelStyle {
    (
        panel.tint,
        panel.border,
        panel.corner_radius,
        panel.border_width,
    )
}

impl ControlsLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        painter: &Painter,
        root: &ContainerVisual,
        controls: &Controls,
    ) -> Result<Self> {
        root.Children()?.RemoveAll()?;
        let local = in_own_window(controls);
        Ok(Self {
            panel: PanelLayer::add(compositor, root, &local.panel)?,
            panel_style: panel_style_of(&controls.panel),
            icons: local
                .icons
                .iter()
                .map(|icon| IconLayer::add(compositor, painter, root, icon))
                .collect::<Result<_>>()?,
        })
    }

    /// `true` si los botones tienen la misma forma y basta con actualizar.
    /// Sin música el panel es el "hidden_panel" de solo botones; con música
    /// es la píldora: el tinte, el borde, el radio y el grosor obligan a
    /// reconstruir. El cambio de imagen (pausa/play, fijado/no fijado) sigue
    /// repintando en `update`.
    pub(crate) fn matches(&self, controls: &Controls) -> bool {
        self.icons.len() == controls.icons.len()
            && self.panel_style == panel_style_of(&controls.panel)
            && self
                .icons
                .iter()
                .zip(&controls.icons)
                .all(|(layer, icon)| layer.accepts(icon))
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        controls: &Controls,
    ) -> Result<()> {
        let local = in_own_window(controls);
        self.panel.update(&local.panel)?;
        for (layer, icon) in self.icons.iter_mut().zip(&local.icons) {
            layer.update(compositor, painter, icon)?;
        }
        Ok(())
    }
}

/// La ventana de los botones abarca el panel y sus iconos: se resta ese origen.
fn in_own_window(controls: &Controls) -> Controls {
    let origin = union_origin(controls);
    Controls {
        panel: Panel {
            bounds: Rect {
                x: controls.panel.bounds.x - origin.0,
                y: controls.panel.bounds.y - origin.1,
                ..controls.panel.bounds
            },
            ..controls.panel
        },
        icons: controls
            .icons
            .iter()
            .map(|icon| Icon {
                x: icon.x - origin.0,
                y: icon.y - origin.1,
                ..icon.clone()
            })
            .collect(),
    }
}

/// Esquina superior izquierda del panel y de todos los iconos.
fn union_origin(controls: &Controls) -> (f32, f32) {
    let mut left = controls.panel.bounds.x;
    let mut top = controls.panel.bounds.y;
    for icon in &controls.icons {
        left = left.min(icon.x);
        top = top.min(icon.y);
    }
    (left, top)
}
