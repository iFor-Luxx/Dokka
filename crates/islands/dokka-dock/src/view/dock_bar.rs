use std::collections::HashMap;
use std::sync::Arc;

use dokka_components::tooltip::tooltip;
use dokka_model::{AppId, DockApp};
use dokka_scene::{Bitmap, Divider, Dot, Icon, IconImage, Panel, Rect, Scene};

use super::dock_style as style;
use super::minimized_window::minimized_window;
use crate::Dock;
use crate::logic::{Entry, hovered_label, place_dock};

impl Dock {
    /// Escena actual. Una app sin imagen muestra un cuadro neutro.
    pub fn scene(
        &self,
        apps: &[DockApp],
        images: &HashMap<AppId, Arc<Bitmap>>,
        names: &HashMap<AppId, String>,
    ) -> Scene {
        let placement = place_dock(self, apps);
        let mut icons = Vec::with_capacity(placement.slots.len());
        let mut thumbnails = Vec::new();
        for &(entry, slot) in &placement.slots {
            let bounds = match entry {
                // La miniatura ocupa el hueco y el icono pasa a ser su insignia.
                Entry::Minimized(_, window) => {
                    let (thumbnail, badge) = minimized_window(window, slot);
                    thumbnails.push(thumbnail);
                    badge
                }
                _ => slot,
            };
            icons.push(self.icon(entry, bounds, images));
        }

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
                color: self.theme.dock.running_dot,
            })
            .collect();

        let labels = hovered_label(self, &placement, names)
            .map(|(text, slot)| tooltip(&self.theme, &text, slot.x + slot.width / 2.0, slot.y))
            .into_iter()
            .collect();

        Scene {
            panel: Panel {
                bounds: placement.bar,
                corner_radius: style::CORNER_RADIUS,
                border_width: style::BORDER_WIDTH,
                tint: self.theme.dock.background,
                border: self.theme.dock.border,
            },
            panels: Vec::new(),
            icons,
            dots,
            dividers: vec![Divider {
                bounds: placement.divider,
                color: self.theme.dock.divider,
            }],
            thumbnails,
            texts: Vec::new(),
            highlight: None,
            overlay: None,
            submenu: None,
            labels,
            // El dock no usa el zoom del Launchpad: siempre en reposo.
            opacity: 1.0,
            scale: 1.0,
        }
    }

    fn icon(&self, entry: Entry<'_>, bounds: Rect, images: &HashMap<AppId, Arc<Bitmap>>) -> Icon {
        Icon {
            x: bounds.x,
            y: bounds.y,
            side: bounds.width,
            image: self.image_for(entry, images),
            placeholder: self.theme.dock.placeholder,
            placeholder_radius: style::PLACEHOLDER_RADIUS,
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
