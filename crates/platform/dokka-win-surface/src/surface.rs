use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;

use crate::SurfaceError;
use crate::create_window::create_window;
use crate::place_window::place_at_bottom_center;

/// Medidas lógicas de una superficie anclada abajo y centrada.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceOptions {
    pub width: f32,
    pub height: f32,
    pub bottom_margin: f32,
    /// Con `true` usa el monitor completo; si no, el área de trabajo.
    pub over_taskbar: bool,
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
    /// Posición en pantalla de la superpuesta mientras está visible.
    pub(crate) above_position: Option<[i32; 4]>,
    pub(crate) scale: f32,
    /// Zonas activas en píxeles, para no repetir llamadas iguales.
    pub(crate) active_areas: Vec<[i32; 4]>,
    /// Foco anterior al abrir el menú, para devolverlo al cerrarlo.
    pub(crate) previous_focus: Option<HWND>,
}

impl Surface {
    pub fn create(options: SurfaceOptions) -> Result<Self, SurfaceError> {
        let window = create_window(None)?;
        let scale = place_at_bottom_center(window, options)?;
        Ok(Self {
            window,
            above: create_window(Some(window))?,
            above_position: None,
            scale,
            active_areas: Vec::new(),
            previous_focus: None,
        })
    }

    pub fn handle(&self) -> SurfaceHandle {
        SurfaceHandle::new(self.window)
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
        self.scale = place_at_bottom_center(self.window, options)?;
        Ok(())
    }
}
