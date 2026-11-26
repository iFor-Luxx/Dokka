use dokka_scene::{Controls, Icon, Panel, Rect};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::icon_layer::IconLayer;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;

/// Botones dibujados en la ventana que va sobre las miniaturas.
pub(crate) struct ControlsLayer {
    panel: PanelLayer,
    icons: Vec<IconLayer>,
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
            icons: local
                .icons
                .iter()
                .map(|icon| IconLayer::add(compositor, painter, root, icon))
                .collect::<Result<_>>()?,
        })
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

/// La ventana de los botones empieza donde empieza su panel: se resta ese origen.
fn in_own_window(controls: &Controls) -> Controls {
    let origin = controls.panel.bounds;
    Controls {
        panel: Panel {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                ..origin
            },
            ..controls.panel
        },
        icons: controls
            .icons
            .iter()
            .map(|icon| Icon {
                x: icon.x - origin.x,
                y: icon.y - origin.y,
                ..icon.clone()
            })
            .collect(),
    }
}
