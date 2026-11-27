use dokka_model::{AppId, DockApp, DockPosition, WindowId};

use super::decide_click::entry_at;
use super::{Entry, place_dock, position_or_bottom};
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
/// Abajo usa `center_x`/`icons_top`/`bar_top` como hoy; en laterales
/// usa `center_y`/`icons_side`/`bar_side` con el lado del dock.
#[derive(Debug, Clone, PartialEq)]
pub struct HoverTarget {
    pub kind: HoverKind,
    pub position: DockPosition,
    pub center_x: f32,
    /// Altura máxima que alcanza un icono agrandado.
    pub icons_top: f32,
    /// Borde superior de la barra.
    pub bar_top: f32,
    pub center_y: f32,
    /// Borde del icono hacia dentro (derecha a la izquierda, izquierda a la derecha).
    pub icons_side: f32,
    /// Borde de la barra hacia dentro.
    pub bar_side: f32,
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
        let position = position_or_bottom(self);
        let (_, slot) = placement.slots.iter().find(|(_, slot)| {
            x >= slot.x - gap / 2.0
                && x <= slot.x + slot.width + gap / 2.0
                && y >= slot.y - gap / 2.0
                && y <= slot.y + slot.height + gap / 2.0
        })?;
        let bar_cross = match position {
            DockPosition::Left => 0.0,
            _ => style::headroom(self),
        };
        let baseline = bar_cross + (style::height(self) + style::slot(self)) / 2.0;
        let icons_top = baseline - style::slot(self) * style::maximum_scale(self);
        // Hacia dentro: el icono más grande llega hasta el borde interior.
        let icons_side = match position {
            DockPosition::Left => {
                let inset = (style::height(self) - style::slot(self)) / 2.0;
                inset + style::slot(self) * style::maximum_scale(self)
            }
            DockPosition::Right => baseline - style::slot(self) * style::maximum_scale(self),
            _ => 0.0,
        };
        let bar_side = match position {
            DockPosition::Left => placement.bar.x + placement.bar.width,
            DockPosition::Right => placement.bar.x,
            _ => placement.bar.y,
        };
        Some(HoverTarget {
            kind,
            position,
            center_x: slot.x + slot.width / 2.0,
            icons_top,
            bar_top: placement.bar.y,
            center_y: slot.y + slot.height / 2.0,
            icons_side,
            bar_side,
        })
    }
}
