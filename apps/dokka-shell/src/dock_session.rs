//! Las piezas que forman el dock en ejecución.

use std::error::Error;
use std::sync::Arc;

use dokka_contracts::WindowSource;
use dokka_dock::{Dock, DockTime};
use dokka_launchpad::Launchpad;
use dokka_menubar::Menubar;
use dokka_model::{AppId, Preferences, WindowId};
use dokka_previews::Previews;
use dokka_settings::Settings;
use dokka_state::DockState;
use dokka_theme::{Theme, light};
use dokka_win_apps::{
    AppPicker, DesktopCatalog, IconCache, NameCache, SystemAppActions, SystemLauncher,
    SystemRecycleBin, recycle_bin_size,
};
use dokka_win_composition::SceneRenderer;
use dokka_win_control::{SystemBrightness, SystemWindowControl, foreground_window};
use dokka_win_media::MediaMonitor;
use dokka_win_surface::{Surface, local_time};
use dokka_win_tracking::SystemWindows;
use dokka_win_tray::TrayHost;

pub(crate) struct DockSession {
    pub(crate) state: DockState,
    pub(crate) dock: Dock,
    pub(crate) menubar: Menubar,
    /// Última barra ya dibujada: si no cambia, no se repinta para no hacer
    /// parpadear (la huella incluye mes, hover y apertura).
    pub(crate) menubar_painted: Option<dokka_menubar::MenubarPainted>,
    /// `Tick` vistos por la barra: red e idioma se releen cada pocos.
    pub(crate) menubar_ticks: u64,
    /// Fuente de la sesión multimedia actual, para enviarle las órdenes.
    pub(crate) menubar_music_source: Option<String>,
    /// Bandeja de segundo plano, si arrancó; `None` con ella apagada.
    pub(crate) background_host: Option<TrayHost>,
    /// Última revisión pasada a la isla; solo se pasa si cambia.
    pub(crate) background_revision: u64,
    /// El error de la bandeja ya se avisó una vez.
    pub(crate) background_error_logged: bool,
    /// El error de salidas de audio ya se avisó una vez.
    pub(crate) audio_error_logged: bool,
    pub(crate) previews: Previews,
    /// Ventana al frente en el último `Refresh`: otro primer plano también
    /// pide cerrar el panel, aunque la lista siga igual.
    pub(crate) foreground: Option<WindowId>,
    pub(crate) launchpad: Launchpad,
    pub(crate) settings: Settings,
    /// Valores vigentes de Preferencias.
    pub(crate) preferences: Preferences,
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
    /// Brillo DDC del monitor principal, en su propio hilo.
    pub(crate) brightness: SystemBrightness,
    pub(crate) picker: AppPicker,
    pub(crate) taskbar_hidden: bool,
    taskbar_auto: bool,
    windows: SystemWindows,
    /// Valor bruto de la ventana propia de Preferencias, si ya se conoce.
    settings_raw: Option<u64>,
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
            menubar: Menubar::new(Arc::clone(&theme)),
            menubar_painted: None,
            menubar_ticks: 0,
            menubar_music_source: None,
            background_host: None,
            background_revision: 0,
            background_error_logged: false,
            audio_error_logged: false,
            previews: Previews::new(Arc::clone(&theme)),
            foreground: None,
            launchpad: Launchpad::new(),
            settings: Settings::new(),
            preferences: Preferences::default(),
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
            brightness: SystemBrightness::start(),
            picker: AppPicker::new(),
            taskbar_hidden: false,
            taskbar_auto: false,
            windows: SystemWindows::default(),
            settings_raw: None,
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

    /// Guarda la ventana al frente y dice si cambió desde la última vez.
    pub(crate) fn poll_foreground_changed(&mut self) -> bool {
        let current = foreground_window();
        if current == self.foreground {
            return false;
        }
        self.foreground = current;
        true
    }

    /// Relee si una app ocupa la pantalla principal y lo guarda en el dock.
    /// Si no se puede saber, queda visible (falla sola). Dice si cambió.
    pub(crate) fn refresh_occlusion(&mut self) -> bool {
        let occluded = dokka_win_tracking::foreground_occludes_primary();
        if occluded == self.dock.is_occluded() {
            return false;
        }
        self.dock.set_occluded(occluded);
        true
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
            || self.menubar.is_control_closing()
            || (self.launchpad.is_open() && self.launchpad.is_moving())
            || (self.launchpad.is_open() && self.launchpad.is_zooming())
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

    /// Recuerda la ventana propia de Preferencias para incluirla minimizada
    /// en el dock. Solo ella va al dock del proceso propio.
    pub(crate) fn remember_settings_window(&mut self, handle: dokka_win_surface::SurfaceHandle) {
        self.settings_raw = Some(handle.raw().0 as u64);
        self.windows.set_settings_window(handle.raw());
    }

    /// `true` si este identificador bruto es la ventana de Preferencias.
    pub(crate) fn is_settings_window_id(&self, window: u64) -> bool {
        self.settings_raw.is_some_and(|known| known == window)
    }

    /// `true` si esta ventana es la propia de Preferencias.
    pub(crate) fn is_settings_window(&self, window: WindowId, surface: &Surface) -> bool {
        window.value() == surface.settings_handle().raw().0 as u64
    }

    /// `true` si esta app es la ventana de Preferencias (no se fija ni se lanza).
    pub(crate) fn is_settings_app(info: &dokka_model::AppInfo) -> bool {
        info.id.as_str() == dokka_win_tracking::SETTINGS_APP_ID
    }

    /// Guarda si la barra se oculta sola al abrir Dokka.
    pub(crate) fn set_taskbar_auto(&mut self, auto: bool) {
        self.taskbar_auto = auto;
    }
}
