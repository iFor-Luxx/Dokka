use dokka_model::{AppId, AppWindow, DockApp, WindowId};

use super::{Entry, Placement, place_dock};
use crate::Dock;
use crate::view::dock_style as style;

/// Lo que debe pasar con una app tras un clic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClickAction {
    Launch,
    Activate(WindowId),
    Minimize(Vec<WindowId>),
}

/// Lo que se pidió al pulsar en el dock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DockAction {
    App(AppId, ClickAction),
    Restore(WindowId),
    Launchpad,
    Calendar,
    Clock,
    Trash,
}

impl Dock {
    /// Acción del punto pulsado. `new_instance` pide abrir otra copia de la app.
    pub fn click(
        &self,
        x: f32,
        y: f32,
        apps: &[DockApp],
        foreground: Option<WindowId>,
        new_instance: bool,
    ) -> Option<DockAction> {
        let placement = place_dock(self, apps);
        Some(match entry_at(&placement, x, y, style::gap(self))? {
            Entry::App(app) | Entry::Explorer(app) => {
                let action = if new_instance {
                    ClickAction::Launch
                } else {
                    action_for(&app.windows, foreground)
                };
                DockAction::App(app.info.id.clone(), action)
            }
            Entry::Minimized(_, window) => DockAction::Restore(window),
            Entry::Launchpad => DockAction::Launchpad,
            Entry::Calendar => DockAction::Calendar,
            Entry::Clock => DockAction::Clock,
            Entry::Trash => DockAction::Trash,
        })
    }
}

pub(crate) fn entry_at<'a>(
    placement: &Placement<'a>,
    x: f32,
    y: f32,
    gap: f32,
) -> Option<Entry<'a>> {
    let bottom = placement.bar.y + placement.bar.height;
    let reach = gap / 2.0;
    placement
        .slots
        .iter()
        .find(|(_, icon)| {
            x >= icon.x - reach && x <= icon.x + icon.width + reach && y >= icon.y && y <= bottom
        })
        .map(|&(entry, _)| entry)
}

/// Cerrada: abrir. Al frente: minimizar. En otro caso: traer al frente.
fn action_for(windows: &[AppWindow], foreground: Option<WindowId>) -> ClickAction {
    let visible = || windows.iter().filter(|window| !window.is_minimized);
    if visible().any(|window| Some(window.id) == foreground) {
        return ClickAction::Minimize(visible().map(|window| window.id).collect());
    }
    match visible().next().or_else(|| windows.first()) {
        Some(window) => ClickAction::Activate(window.id),
        None => ClickAction::Launch,
    }
}
