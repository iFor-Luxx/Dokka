use std::collections::HashMap;
use std::sync::Arc;

use dokka_model::{AppId, DockApp};
use dokka_scene::{Bitmap, Icon, Panel, Scene, Size};

use super::dock_style as style;
use crate::logic::bar_width;

/// Construye la escena del dock. Una app sin imagen muestra un cuadro neutro.
pub fn dock_scene(apps: &[DockApp], images: &HashMap<AppId, Arc<Bitmap>>) -> Scene {
    let size = Size {
        width: bar_width(apps.len(), style::SLOT, style::GAP, style::PADDING),
        height: style::HEIGHT,
    };
    let icons = apps
        .iter()
        .enumerate()
        .map(|(index, app)| Icon {
            x: style::PADDING + index as f32 * (style::SLOT + style::GAP),
            y: (style::HEIGHT - style::SLOT) / 2.0,
            side: style::SLOT,
            image: images.get(&app.info.id).cloned(),
            placeholder: style::PLACEHOLDER,
            placeholder_radius: style::PLACEHOLDER_RADIUS,
        })
        .collect();

    Scene {
        panel: Panel {
            size,
            corner_radius: style::CORNER_RADIUS,
            tint: style::TINT,
            border: style::BORDER,
        },
        icons,
    }
}
