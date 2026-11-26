//! Las piezas que forman el dock en ejecución.

use std::error::Error;

use dokka_contracts::WindowSource;
use dokka_dock::{DOCK_BOTTOM_MARGIN, Dock};
use dokka_model::AppId;
use dokka_scene::Scene;
use dokka_state::DockState;
use dokka_win_apps::{IconCache, SystemLauncher, recycle_bin_has_items};
use dokka_win_composition::SceneRenderer;
use dokka_win_control::SystemWindowControl;
use dokka_win_surface::{Surface, SurfaceOptions, local_time};
use dokka_win_tracking::SystemWindows;

pub(crate) struct DockSession {
    pub(crate) state: DockState,
    pub(crate) dock: Dock,
    pub(crate) explorer: AppId,
    pub(crate) launcher: SystemLauncher,
    pub(crate) control: SystemWindowControl,
    windows: SystemWindows,
    icons: IconCache,
    renderer: SceneRenderer,
}

impl DockSession {
    pub(crate) fn new(renderer: SceneRenderer, explorer: AppId) -> Self {
        Self {
            state: DockState::default(),
            dock: Dock::new(explorer.clone()),
            explorer,
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

    pub(crate) fn update_clock_and_trash(&mut self) {
        let now = local_time();
        self.dock
            .set_time(now.month, now.day, now.hour, now.minute, now.second);
        // Consultar la papelera cuesta más: basta cada pocos segundos.
        if now.second.is_multiple_of(3) {
            self.dock.set_trash_full(recycle_bin_has_items());
        }
    }

    /// Ajusta la ventana al número de iconos.
    pub(crate) fn resize(&self, surface: &mut Surface) -> Result<(), Box<dyn Error>> {
        let size = self.dock.surface_size(self.state.apps());
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
