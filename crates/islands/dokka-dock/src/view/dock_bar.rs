use std::collections::HashMap;
use std::sync::Arc;

use dokka_model::{AppId, DockApp};
use dokka_scene::{Bitmap, Dot, Icon, Panel, Scene};

use super::dock_style as style;
use crate::Dock;
use crate::logic::place_dock;

impl Dock {
    /// Escena actual. Una app sin imagen muestra un cuadro neutro.
    pub fn scene(&self, apps: &[DockApp], images: &HashMap<AppId, Arc<Bitmap>>) -> Scene {
        let placement = place_dock(self, apps);
        let icons = apps
            .iter()
            .zip(&placement.icons)
            .map(|(app, bounds)| Icon {
                x: bounds.x,
                y: bounds.y,
                side: bounds.width,
                image: images.get(&app.info.id).cloned(),
                placeholder: style::PLACEHOLDER,
                placeholder_radius: style::PLACEHOLDER_RADIUS,
            })
            .collect();

        let dot_y = placement.bar.y + placement.bar.height - style::DOT_FROM_BOTTOM;
        let dots = apps
            .iter()
            .zip(&placement.icons)
            .filter(|(app, _)| app.is_running())
            .map(|(_, bounds)| Dot {
                x: bounds.x + (bounds.width - style::DOT_DIAMETER) / 2.0,
                y: dot_y,
                diameter: style::DOT_DIAMETER,
                color: style::DOT_COLOR,
            })
            .collect();

        Scene {
            panel: Panel {
                bounds: placement.bar,
                corner_radius: style::CORNER_RADIUS,
                tint: style::TINT,
                border: style::BORDER,
            },
            icons,
            dots,
        }
    }
}
