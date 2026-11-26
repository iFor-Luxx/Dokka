use std::collections::HashMap;
use std::sync::Arc;

use dokka_model::{AppId, DockApp};
use dokka_scene::{Bitmap, Divider, Dot, Icon, IconImage, Panel, Scene};

use super::dock_style as style;
use crate::Dock;
use crate::logic::{Entry, place_dock};

impl Dock {
    /// Escena actual. Una app sin imagen muestra un cuadro neutro.
    pub fn scene(&self, apps: &[DockApp], images: &HashMap<AppId, Arc<Bitmap>>) -> Scene {
        let placement = place_dock(self, apps);
        let icons = placement
            .slots
            .iter()
            .map(|&(entry, bounds)| Icon {
                x: bounds.x,
                y: bounds.y,
                side: bounds.width,
                image: self.image_for(entry, images),
                placeholder: style::PLACEHOLDER,
                placeholder_radius: style::PLACEHOLDER_RADIUS,
            })
            .collect();

        let dot_y = placement.bar.y + placement.bar.height - style::DOT_FROM_BOTTOM;
        let dots = placement
            .slots
            .iter()
            .filter(|(entry, _)| match entry {
                Entry::App(app) | Entry::Explorer(app) => app.is_running(),
                _ => false,
            })
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
            dividers: vec![Divider {
                bounds: placement.divider,
                color: style::DIVIDER_COLOR,
            }],
        }
    }

    fn image_for(
        &self,
        entry: Entry<'_>,
        images: &HashMap<AppId, Arc<Bitmap>>,
    ) -> Option<IconImage> {
        let drawing = match entry {
            Entry::Explorer(_) => &self.icons.explorer,
            Entry::Launchpad => &self.icons.launchpad,
            Entry::Calendar => &self.icons.calendar,
            Entry::Clock => &self.icons.clock,
            Entry::Trash => &self.icons.trash,
            Entry::App(app) | Entry::Minimized(app, _) => {
                return images.get(&app.info.id).cloned().map(IconImage::Pixels);
            }
        };
        Some(IconImage::Drawing(Arc::clone(drawing)))
    }
}
