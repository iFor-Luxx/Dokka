use std::collections::HashMap;
use std::sync::Arc;

use dokka_components::tooltip::{tooltip, tooltip_side};
use dokka_model::{AppId, DockApp, DockPosition};
use dokka_scene::{Bitmap, Divider, Dot, Icon, IconImage, Panel, Rect, Scene};

use super::dock_style as style;
use super::minimized_window::minimized_window;
use crate::Dock;
use crate::logic::{Entry, home_line, hovered_label, place_dock};

impl Dock {
    /// Escena actual. Una app sin imagen muestra un cuadro neutro.
    /// Escondido con línea solo dibuja la cápsula al borde.
    pub fn scene(
        &self,
        apps: &[DockApp],
        images: &HashMap<AppId, Arc<Bitmap>>,
        names: &HashMap<AppId, String>,
    ) -> Scene {
        if self.hide.is_hidden() {
            return self.line_scene(apps);
        }
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

        // Sin indicadores la lista se encoge; `SceneLayers` la reutiliza sin problema.
        let dots: Vec<Dot> = if self.preferences.show_indicators {
            placement
                .slots
                .iter()
                .filter(|(entry, _)| match entry {
                    Entry::App(app) | Entry::Explorer(app) => app.is_running(),
                    _ => false,
                })
                .map(|(_, bounds)| {
                    let diameter = style::dot_diameter(self);
                    match self.preferences.position {
                        DockPosition::Left => Dot {
                            x: placement.bar.x + style::dot_from_bottom(self) - diameter,
                            y: bounds.y + (bounds.height - diameter) / 2.0,
                            diameter,
                            color: self.theme.dock.running_dot,
                        },
                        DockPosition::Right => Dot {
                            x: placement.bar.x + placement.bar.width - style::dot_from_bottom(self),
                            y: bounds.y + (bounds.height - diameter) / 2.0,
                            diameter,
                            color: self.theme.dock.running_dot,
                        },
                        _ => Dot {
                            x: bounds.x + (bounds.width - diameter) / 2.0,
                            y: placement.bar.y + placement.bar.height
                                - style::dot_from_bottom(self),
                            diameter,
                            color: self.theme.dock.running_dot,
                        },
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        let labels = hovered_label(self, &placement, names)
            .map(|(text, slot)| match self.preferences.position {
                DockPosition::Left => tooltip_side(
                    &self.theme,
                    &text,
                    slot.y + slot.height / 2.0,
                    slot.x + slot.width,
                    true,
                ),
                DockPosition::Right => tooltip_side(
                    &self.theme,
                    &text,
                    slot.y + slot.height / 2.0,
                    slot.x,
                    false,
                ),
                _ => tooltip(&self.theme, &text, slot.x + slot.width / 2.0, slot.y),
            })
            .into_iter()
            .collect();

        Scene {
            panel: Panel {
                bounds: placement.bar,
                corner_radius: style::corner_radius(self),
                border_width: dokka_theme::dock::BORDER_WIDTH,
                tint: self.theme.dock.background,
                border: self.theme.dock.border,
            },
            panels: Vec::new(),
            flat_panels: Vec::new(),
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
            blur: 0.0,
        }
    }

    fn icon(&self, entry: Entry<'_>, bounds: Rect, images: &HashMap<AppId, Arc<Bitmap>>) -> Icon {
        Icon {
            x: bounds.x,
            y: bounds.y,
            side: bounds.width,
            image: self.image_for(entry, images),
            placeholder: self.theme.dock.placeholder,
            placeholder_radius: style::placeholder_radius(self),
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
                if crate::logic::is_settings_app(app) {
                    return Some(IconImage::Drawing(Arc::clone(&self.icons.settings)));
                }
                return images.get(&app.info.id).cloned().map(IconImage::Pixels);
            }
        };
        Some(IconImage::Drawing(Arc::clone(drawing)))
    }

    /// Solo la línea al borde, con su cristal y sin iconos: la ventana recorta
    /// el resto con la zona activa, así no tapa clics ni deja rectángulo.
    fn line_scene(&self, apps: &[DockApp]) -> Scene {
        let size = self.surface_size(apps);
        let bounds = home_line(self, size);
        Scene {
            panel: Panel {
                bounds,
                corner_radius: bounds.height / 2.0,
                border_width: 1.0,
                tint: self.theme.dock.home_line,
                border: self.theme.dock.home_line_border,
            },
            panels: Vec::new(),
            flat_panels: Vec::new(),
            icons: Vec::new(),
            dots: Vec::new(),
            dividers: Vec::new(),
            thumbnails: Vec::new(),
            texts: Vec::new(),
            highlight: None,
            overlay: None,
            submenu: None,
            labels: Vec::new(),
            opacity: 1.0,
            scale: 1.0,
            blur: 0.0,
        }
    }
}
