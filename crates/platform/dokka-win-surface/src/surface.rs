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
}

/// Referencia a la ventana para otras piezas de `platform`.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceHandle(HWND);

impl SurfaceHandle {
    pub fn raw(self) -> HWND {
        self.0
    }
}

/// Ventana de Dokka.
#[derive(Debug)]
pub struct Surface {
    pub(crate) window: HWND,
    pub(crate) scale: f32,
    /// Zonas activas en píxeles, para no repetir llamadas iguales.
    pub(crate) active_areas: Vec<[i32; 4]>,
}

impl Surface {
    pub fn create(options: SurfaceOptions) -> Result<Self, SurfaceError> {
        let window = create_window()?;
        let scale = place_at_bottom_center(window, options)?;
        Ok(Self {
            window,
            scale,
            active_areas: Vec::new(),
        })
    }

    pub fn handle(&self) -> SurfaceHandle {
        SurfaceHandle(self.window)
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
