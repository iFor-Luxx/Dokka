use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, KillTimer, MSG, SW_SHOWNOACTIVATE, SetTimer, ShowWindow,
    TranslateMessage, WM_TIMER,
};

use crate::SurfaceError;
use crate::create_window::create_window;
use crate::place_window::place_at_bottom_center;

/// Espera para agrupar ráfagas de avisos en una sola actualización.
const SETTLE_MILLISECONDS: u32 = 200;

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
    window: HWND,
    scale: f32,
}

impl Surface {
    pub fn create(options: SurfaceOptions) -> Result<Self, SurfaceError> {
        let window = create_window()?;
        let scale = place_at_bottom_center(window, options)?;
        Ok(Self { window, scale })
    }

    pub fn handle(&self) -> SurfaceHandle {
        SurfaceHandle(self.window)
    }

    /// Píxeles físicos por unidad lógica.
    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn resize(&mut self, options: SurfaceOptions) -> Result<(), SurfaceError> {
        self.scale = place_at_bottom_center(self.window, options)?;
        Ok(())
    }

    /// Muestra la ventana y atiende mensajes hasta que se cierre.
    /// Llama a `on_refresh` poco después de recibir `refresh_message` en este hilo.
    pub fn run(
        &mut self,
        refresh_message: u32,
        mut on_refresh: impl FnMut(&mut Self),
    ) -> Result<(), SurfaceError> {
        let mut message = MSG::default();
        let mut pending_timer = None;
        // SAFETY: la ventana pertenece a este hilo.
        let _ = unsafe { ShowWindow(self.window, SW_SHOWNOACTIVATE) };
        loop {
            // SAFETY: `message` es escribible durante la llamada.
            match unsafe { GetMessageW(&mut message, None, 0, 0) }.0 {
                0 => return Ok(()),
                -1 => return Err(windows::core::Error::from_thread().into()),
                _ => {}
            }
            if message.hwnd.is_invalid() && message.message == refresh_message {
                // No se reinicia el temporizador: avisos continuos no deben aplazarlo siempre.
                if pending_timer.is_none() {
                    // SAFETY: temporizador del hilo, sin callback.
                    pending_timer = Some(unsafe { SetTimer(None, 0, SETTLE_MILLISECONDS, None) });
                }
            } else if message.message == WM_TIMER && pending_timer == Some(message.wParam.0) {
                // SAFETY: cancela el temporizador creado arriba.
                let _ = unsafe { KillTimer(None, message.wParam.0) };
                pending_timer = None;
                on_refresh(self);
            } else {
                // SAFETY: `message` acaba de ser rellenado por Windows.
                unsafe {
                    let _ = TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
        }
    }
}
