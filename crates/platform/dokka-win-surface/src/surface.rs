use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;

use dokka_model::DockPosition;

use crate::SurfaceError;
use crate::create_window::{
    create_launchpad_window, create_menubar_window, create_preview_window, create_settings_window,
    create_window,
};
use crate::place_window::{fullscreen_size, place_dock_window, place_fullscreen};

/// Medidas lógicas de la superficie del dock, anclada a su borde.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceOptions {
    pub width: f32,
    pub height: f32,
    pub bottom_margin: f32,
    /// Con `true` usa el monitor completo; si no, el área de trabajo.
    pub over_taskbar: bool,
    pub position: DockPosition,
    /// 0 visible, 1 escondido: desplaza la ventana hacia fuera.
    pub hide: f32,
}

/// Referencia a la ventana para otras piezas de `platform`.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceHandle(HWND);

impl SurfaceHandle {
    pub(crate) fn new(window: HWND) -> Self {
        Self(window)
    }

    pub fn raw(self) -> HWND {
        self.0
    }
}

/// Ventana de Dokka, con otra superpuesta que se muestra cuando hace falta.
#[derive(Debug)]
pub struct Surface {
    pub(crate) window: HWND,
    pub(crate) above: HWND,
    /// Pantalla completa del Launchpad, oculta hasta que se abre.
    pub(crate) launchpad: HWND,
    /// Vista grande de la ventana bajo el cursor, oculta hasta que se usa.
    pub(crate) preview: HWND,
    /// `true` mientras la vista grande está visible.
    pub(crate) preview_visible: bool,
    /// `true` mientras el Launchpad está visible.
    pub(crate) launchpad_visible: bool,
    /// Tamaño lógico del Launchpad para dibujar la cuadrícula.
    pub(crate) launchpad_size: [f32; 2],
    /// Ventana de Preferencias, oculta hasta que se abre.
    pub(crate) settings: HWND,
    /// `true` mientras Preferencias está visible.
    pub(crate) settings_visible: bool,
    /// `true` cuando Preferencias ya se centró en su sitio.
    pub(crate) settings_placed: bool,
    /// Escala de la ventana de Preferencias para pasar a píxeles.
    pub(crate) settings_scale: f32,
    /// Posición en pantalla de la superpuesta mientras está visible.
    pub(crate) above_position: Option<[i32; 4]>,
    pub(crate) scale: f32,
    /// Zonas activas en píxeles, para no repetir llamadas iguales.
    pub(crate) active_areas: Vec<[i32; 4]>,
    /// Foco anterior al abrir el menú, para devolverlo al cerrarlo.
    pub(crate) previous_focus: Option<HWND>,
    /// Foco anterior al abrir el Launchpad.
    pub(crate) launchpad_previous: Option<HWND>,
    /// Ventana de la barra superior, oculta hasta que se muestra.
    pub(crate) menubar: HWND,
    /// `true` mientras la barra está visible (aunque la tape un pantalla completa).
    pub(crate) menubar_visible: bool,
    /// `true` si ya se registró como AppBar (nunca dos veces).
    pub(crate) menubar_registered: bool,
    /// Escala de la barra; la misma que la del dock en el principal.
    pub(crate) menubar_scale: f32,
    /// Zonas activas de la barra en píxeles, para no repetir llamadas iguales.
    pub(crate) menubar_active: Vec<[i32; 4]>,
    /// Foco anterior al abrir el menú de la barra.
    pub(crate) menubar_previous: Option<HWND>,
    /// `true` mientras una app a pantalla completa oculta la barra.
    pub(crate) menubar_fullscreen: bool,
    /// Temporizador de vigilancia del cursor, solo con panel abierto o pendiente.
    pub(crate) watch_timer: Option<usize>,
}

impl Surface {
    pub fn create(options: SurfaceOptions) -> Result<Self, SurfaceError> {
        let window = create_window(None)?;
        let scale = place_dock_window(window, options)?;
        let launchpad = create_launchpad_window()?;
        let launchpad_scale = place_fullscreen(launchpad)?;
        let launchpad_size = fullscreen_size(launchpad, launchpad_scale);
        let settings = create_settings_window()?;
        let menubar = create_menubar_window()?;
        Ok(Self {
            window,
            above: create_window(Some(window))?,
            launchpad,
            preview: create_preview_window()?,
            preview_visible: false,
            launchpad_visible: false,
            launchpad_size,
            settings,
            settings_visible: false,
            settings_placed: false,
            settings_scale: 1.0,
            above_position: None,
            scale,
            active_areas: Vec::new(),
            previous_focus: None,
            launchpad_previous: None,
            menubar,
            menubar_visible: false,
            menubar_registered: false,
            // Se actualiza en `menubar_show` con el DPI real de la barra.
            menubar_scale: scale,
            menubar_active: Vec::new(),
            menubar_previous: None,
            menubar_fullscreen: false,
            watch_timer: None,
        })
    }

    pub fn handle(&self) -> SurfaceHandle {
        SurfaceHandle::new(self.window)
    }

    pub fn launchpad_handle(&self) -> SurfaceHandle {
        SurfaceHandle::new(self.launchpad)
    }

    /// Píxeles físicos por unidad lógica.
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Pide terminar `run`.
    pub fn close(&self) {
        // SAFETY: encola el fin del bucle de este hilo.
        unsafe { PostQuitMessage(0) };
    }

    pub fn resize(&mut self, options: SurfaceOptions) -> Result<(), SurfaceError> {
        self.scale = place_dock_window(self.window, options)?;
        Ok(())
    }
}
