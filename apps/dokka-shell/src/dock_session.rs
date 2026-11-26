//! Las piezas que forman el dock en ejecución.

use std::error::Error;

use dokka_contracts::WindowSource;
use dokka_dock::{DOCK_BOTTOM_MARGIN, Dock, DockTime};
use dokka_model::AppId;
use dokka_previews::Previews;
use dokka_scene::Scene;
use dokka_state::DockState;
use dokka_win_apps::{IconCache, NameCache, SystemLauncher, recycle_bin_has_items};
use dokka_win_composition::SceneRenderer;
use dokka_win_control::SystemWindowControl;
use dokka_win_media::MediaMonitor;
use dokka_win_surface::{Surface, SurfaceOptions, local_time};
use dokka_win_tracking::SystemWindows;

use crate::media::update_media;

pub(crate) struct DockSession {
    pub(crate) state: DockState,
    pub(crate) dock: Dock,
    pub(crate) previews: Previews,
    pub(crate) explorer: AppId,
    pub(crate) launcher: SystemLauncher,
    pub(crate) control: SystemWindowControl,
    pub(crate) media: MediaMonitor,
    windows: SystemWindows,
    icons: IconCache,
    names: NameCache,
    renderer: SceneRenderer,
}

impl DockSession {
    pub(crate) fn new(renderer: SceneRenderer, explorer: AppId) -> Self {
        Self {
            state: DockState::default(),
            dock: Dock::new(explorer.clone()),
            previews: Previews::default(),
            explorer,
            launcher: SystemLauncher,
            control: SystemWindowControl,
            media: MediaMonitor::start(),
            windows: SystemWindows,
            icons: IconCache::new(),
            names: NameCache::default(),
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
        self.dock.set_time(DockTime {
            year: now.year,
            month: now.month,
            day: now.day,
            weekday: now.weekday,
            hour: now.hour,
            minute: now.minute,
            second: now.second,
        });
        self.previews.set_today(now.year, now.month, now.day);
        // Consultar la papelera cuesta más: basta cada pocos segundos.
        if now.second.is_multiple_of(3) {
            self.dock.set_trash_full(recycle_bin_has_items());
        }
    }

    pub(crate) fn is_animating(&self) -> bool {
        self.dock.is_animating() || self.previews.is_waiting()
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
        update_media(self);
        let mut scene = self.scene();
        scene.overlay = self.previews.overlay();
        // Con un panel abierto, la etiqueta sobra.
        if scene.overlay.is_some() {
            scene.labels.clear();
        }
        self.renderer.draw(&scene)?;
        let controls = scene.overlay.and_then(|overlay| overlay.controls);
        surface.show_above(controls.map(|controls| {
            let area = controls.panel.bounds;
            [area.x, area.y, area.width, area.height]
        }))?;
        let areas: Vec<[f32; 4]> = [
            Some(self.dock.active_area(self.state.apps())),
            self.previews.area(),
        ]
        .into_iter()
        .flatten()
        .map(|area| [area.x, area.y, area.width, area.height])
        .collect();
        surface.set_active_areas(&areas);
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
        let names = self
            .state
            .apps()
            .iter()
            .filter_map(|app| {
                let name = self.names.name_for(&app.info.executable)?;
                Some((app.info.id.clone(), name))
            })
            .collect();
        self.dock.scene(self.state.apps(), &images, &names)
    }
}
