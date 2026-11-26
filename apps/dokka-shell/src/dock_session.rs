//! Conexión entre las ventanas observadas, el estado y la escena del dock.

use std::error::Error;

use dokka_contracts::WindowSource;
use dokka_dock::{DOCK_BOTTOM_MARGIN, dock_scene};
use dokka_scene::Scene;
use dokka_state::DockState;
use dokka_win_apps::IconCache;
use dokka_win_surface::SurfaceOptions;
use dokka_win_tracking::SystemWindows;

#[derive(Default)]
pub(crate) struct DockSession {
    state: DockState,
    windows: SystemWindows,
    icons: IconCache,
}

impl DockSession {
    /// Aplica los cambios observados; indica si hubo alguno.
    pub(crate) fn sync(&mut self) -> Result<bool, Box<dyn Error>> {
        let observed = self.windows.open_windows()?;
        let events = self.state.changes_for(&observed);
        let changed = !events.is_empty();
        for event in events {
            self.state.apply(event);
        }
        Ok(changed)
    }

    pub(crate) fn scene(&mut self) -> Scene {
        let images = self
            .state
            .apps()
            .iter()
            .filter_map(|app| {
                let image = self.icons.icon_for(&app.info.executable)?;
                Some((app.info.id.clone(), image))
            })
            .collect();
        dock_scene(self.state.apps(), &images)
    }
}

pub(crate) fn surface_options(scene: &Scene) -> SurfaceOptions {
    SurfaceOptions {
        width: scene.panel.size.width,
        height: scene.panel.size.height,
        bottom_margin: DOCK_BOTTOM_MARGIN,
    }
}
