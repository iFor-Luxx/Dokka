use dokka_model::{AppId, AppWindow, DockApp, WindowId};

use super::place_dock;
use crate::Dock;
use crate::view::style;

/// Lo que debe pasar tras un clic sobre una app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClickAction {
    Launch,
    Activate(WindowId),
    Minimize(Vec<WindowId>),
}

impl Dock {
    /// App pulsada y acción a realizar. `new_instance` pide abrir otra copia.
    pub fn click(
        &self,
        x: f32,
        y: f32,
        apps: &[DockApp],
        foreground: Option<WindowId>,
        new_instance: bool,
    ) -> Option<(AppId, ClickAction)> {
        let placement = place_dock(self, apps);
        let bottom = placement.bar.y + placement.bar.height;
        let reach = style::GAP / 2.0;
        let index = placement.icons.iter().position(|icon| {
            x >= icon.x - reach && x <= icon.x + icon.width + reach && y >= icon.y && y <= bottom
        })?;
        let app = apps.get(index)?;
        let action = if new_instance {
            ClickAction::Launch
        } else {
            action_for(&app.windows, foreground)
        };
        Some((app.info.id.clone(), action))
    }
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
