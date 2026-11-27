use dokka_model::{AppId, DockApp, WindowId};

use super::decide_click::entry_at;
use super::{Entry, place_dock};
use crate::Dock;
use crate::view::dock_style as style;

/// Contenido que merece un panel flotante.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverKind {
    Calendar,
    /// Ventanas abiertas de una app, con su título.
    Windows {
        app: AppId,
        windows: Vec<(WindowId, String)>,
    },
}

/// Lo que hay bajo el cursor y dónde apoyar su panel.
#[derive(Debug, Clone, PartialEq)]
pub struct HoverTarget {
    pub kind: HoverKind,
    pub center_x: f32,
    /// Altura máxima que alcanza un icono agrandado.
    pub icons_top: f32,
    /// Borde superior de la barra.
    pub bar_top: f32,
}

impl Dock {
    pub fn hover_target(&self, x: f32, y: f32, apps: &[DockApp]) -> Option<HoverTarget> {
        let placement = place_dock(self, apps);
        let entry = entry_at(&placement, x, y, style::gap(self))?;
        let kind = match entry {
            Entry::Calendar => HoverKind::Calendar,
            Entry::App(app) | Entry::Explorer(app) if app.is_running() => HoverKind::Windows {
                app: app.info.id.clone(),
                windows: app
                    .windows
                    .iter()
                    .map(|window| (window.id, window.title.clone()))
                    .collect(),
            },
            Entry::Minimized(app, id) => HoverKind::Windows {
                app: app.info.id.clone(),
                windows: app
                    .windows
                    .iter()
                    .filter(|window| window.id == id)
                    .map(|window| (window.id, window.title.clone()))
                    .collect(),
            },
            _ => return None,
        };
        let gap = style::gap(self);
        let (_, slot) = placement
            .slots
            .iter()
            .find(|(_, slot)| x >= slot.x - gap / 2.0 && x <= slot.x + slot.width + gap / 2.0)?;
        let baseline = style::headroom(self) + (style::height(self) + style::slot(self)) / 2.0;
        Some(HoverTarget {
            kind,
            center_x: slot.x + slot.width / 2.0,
            icons_top: baseline - style::slot(self) * style::maximum_scale(self),
            bar_top: placement.bar.y,
        })
    }
}
