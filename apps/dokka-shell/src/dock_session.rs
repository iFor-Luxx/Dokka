//! Las piezas que forman el dock en ejecución.

use std::error::Error;

use dokka_contracts::WindowSource;
use dokka_dock::{DOCK_BOTTOM_MARGIN, Dock};
use dokka_scene::Scene;
use dokka_state::DockState;
use dokka_win_apps::{IconCache, SystemLauncher};
use dokka_win_composition::SceneRenderer;
use dokka_win_control::SystemWindowControl;
use dokka_win_surface::{Surface, SurfaceOptions};
use dokka_win_tracking::SystemWindows;

pub(crate) struct DockSession {
    pub(crate) state: DockState,
    pub(crate) dock: Dock,
    pub(crate) launcher: SystemLauncher,
    pub(crate) control: SystemWindowControl,
    windows: SystemWindows,
    icons: IconCache,
    renderer: SceneRenderer,
}

impl DockSession {
    pub(crate) fn new(renderer: SceneRenderer) -> Self {
        Self {
            state: DockState::default(),
            dock: Dock::default(),
            launcher: SystemLauncher,
            control: SystemWindowControl,
            windows: SystemWindows,
            icons: IconCache::new(),
            renderer,
        }
    }

    /// Aplica los cambios observados; indica si hubo alguno.
    pub(crate) fn sync(&mut self) -> Result<bool, Box<dyn Error>> {
        let observed = self.windows.open_windows()?;
        let events = self.state.changes_for(&observed);
        let changed = !events.is_empty();
        for event in events {
            self.state.apply(event);
        }
        if changed {
            self.dock.apps_changed(self.state.apps());
        }
        Ok(changed)
    }

    /// Ajusta la ventana al número de apps.
    pub(crate) fn resize(&self, surface: &mut Surface) -> Result<(), Box<dyn Error>> {
        let size = self.dock.surface_size(self.state.apps().len());
        surface.resize(SurfaceOptions {
            width: size.width,
            height: size.height,
            bottom_margin: DOCK_BOTTOM_MARGIN,
        })?;
        Ok(())
    }

    /// Dibuja el instante actual.
    pub(crate) fn render(&mut self, surface: &mut Surface) -> Result<(), Box<dyn Error>> {
        let scene = self.scene();
        self.renderer.draw(&scene)?;
        let area = self.dock.active_area(self.state.apps());
        surface.set_active_area(area.x, area.y, area.width, area.height);
        Ok(())
    }

    fn scene(&mut self) -> Scene {
        let images = self
            .state
            .apps()
            .iter()
            .filter_map(|app| {
                let image = self.icons.icon_for(&app.info.executable)?;
                Some((app.info.id.clone(), image))
            })
            .collect();
        self.dock.scene(self.state.apps(), &images)
    }
}
