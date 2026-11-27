//! Las piezas que forman el dock en ejecución.

use std::error::Error;
use std::sync::Arc;

use dokka_contracts::WindowSource;
use dokka_dock::{DOCK_BOTTOM_MARGIN, Dock, DockTime};
use dokka_launchpad::Launchpad;
use dokka_model::AppId;
use dokka_previews::Previews;
use dokka_settings::Settings;
use dokka_state::DockState;
use dokka_theme::{Theme, light};
use dokka_win_apps::{
    AppPicker, DesktopCatalog, IconCache, NameCache, SystemAppActions, SystemLauncher,
    SystemRecycleBin, recycle_bin_size,
};
use dokka_win_composition::SceneRenderer;
use dokka_win_control::SystemWindowControl;
use dokka_win_media::MediaMonitor;
use dokka_win_surface::{Surface, SurfaceOptions, local_time};
use dokka_win_tracking::SystemWindows;

pub(crate) struct DockSession {
    pub(crate) state: DockState,
    pub(crate) dock: Dock,
    pub(crate) previews: Previews,
    pub(crate) launchpad: Launchpad,
    pub(crate) settings: Settings,
    /// `true` entre el botón abajo y el clic sobre Preferencias; filtra los
    /// botones sueltos sin pulsación previa (el arrastre nativo se traga el
    /// suyo y el cursor puede salir y volver a entrar).
    pub(crate) settings_pressed: bool,
    /// Última escena de Preferencias ya dibujada: si no cambió, no se repinta
    /// para no hacer parpadear el cristal.
    pub(crate) settings_painted: Option<dokka_settings::PaintedSettings>,
    pub(crate) catalog: DesktopCatalog,
    pub(crate) theme: Arc<Theme>,
    pub(crate) explorer: AppId,
    pub(crate) launcher: SystemLauncher,
    pub(crate) control: SystemWindowControl,
    pub(crate) app_actions: SystemAppActions,
    pub(crate) recycle_bin: SystemRecycleBin,
    pub(crate) media: MediaMonitor,
    pub(crate) picker: AppPicker,
    taskbar_hidden: bool,
    taskbar_auto: bool,
    windows: SystemWindows,
    pub(crate) icons: IconCache,
    pub(crate) names: NameCache,
    pub(crate) renderer: SceneRenderer,
}

impl DockSession {
    pub(crate) fn new(renderer: SceneRenderer, explorer: AppId) -> Self {
        // Único lugar donde se elige el tema.
        let theme = Arc::new(light());
        Self {
            state: DockState::default(),
            dock: Dock::new(explorer.clone(), Arc::clone(&theme)),
            previews: Previews::new(Arc::clone(&theme)),
            launchpad: Launchpad::new(),
            settings: Settings::new(),
            settings_pressed: false,
            settings_painted: None,
            catalog: DesktopCatalog::new(),
            theme,
            explorer,
            launcher: SystemLauncher,
            control: SystemWindowControl,
            app_actions: SystemAppActions,
            recycle_bin: SystemRecycleBin,
            media: MediaMonitor::start(),
            picker: AppPicker::new(),
            taskbar_hidden: false,
            taskbar_auto: false,
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
            self.dock.set_trash(recycle_bin_size());
        }
    }

    pub(crate) fn is_animating(&self) -> bool {
        self.dock.is_animating()
            || self.previews.is_waiting()
            || (self.launchpad.is_open() && self.launchpad.is_moving())
            || (self.launchpad.is_open() && self.launchpad.is_zooming())
    }

    /// Ajusta la ventana al número de iconos.
    pub(crate) fn resize(&self, surface: &mut Surface) -> Result<(), Box<dyn Error>> {
        let size = self.dock.surface_size(self.state.apps());
        surface.resize(SurfaceOptions {
            width: size.width,
            height: size.height,
            bottom_margin: DOCK_BOTTOM_MARGIN,
            over_taskbar: self.taskbar_hidden,
        })?;
        Ok(())
    }

    /// Guarda si la barra está oculta; quien la cambia recoloca con `resize`.
    pub(crate) fn set_taskbar_hidden(&mut self, hidden: bool) {
        self.taskbar_hidden = hidden;
    }

    pub(crate) fn taskbar_hidden(&self) -> bool {
        self.taskbar_hidden
    }

    pub(crate) fn taskbar_auto(&self) -> bool {
        self.taskbar_auto
    }

    /// Guarda si la barra se oculta sola al abrir Dokka.
    pub(crate) fn set_taskbar_auto(&mut self, auto: bool) {
        self.taskbar_auto = auto;
    }
}
