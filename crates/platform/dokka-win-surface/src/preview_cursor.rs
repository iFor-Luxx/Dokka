//! Posición real del cursor para la red de seguridad de los paneles.
//!
//! Mientras hay un panel (abierto o pendiente) el shell comprueba dónde está
//! de verdad el cursor: si el aviso de salida se perdió, el cursor manda.

use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, GetWindowRect, KillTimer, SetTimer,
};

use crate::{Surface, SurfaceEvent};

/// Cada cuánto se comprueba el cursor con el panel abierto, solo entonces.
const WATCH_MILLISECONDS: u32 = 200;

impl Surface {
    /// Posición lógica del cursor sobre la ventana del dock, si está sobre ella.
    /// Tiene en cuenta el DPI y solo vale dentro de las zonas activas: la
    /// superpuesta y la vista grande dejan pasar el ratón, así que el cursor
    /// sobre sus botones sigue contando como sobre el dock.
    pub fn cursor_over_dock(&self) -> Option<(f32, f32)> {
        // SAFETY: `point` y `area` son escribibles durante cada llamada y la
        // ventana es propia.
        unsafe {
            let mut point = POINT::default();
            if GetCursorPos(&mut point).is_err() {
                return None;
            }
            let mut area = RECT::default();
            if GetWindowRect(self.window, &mut area).is_err() {
                return None;
            }
            let x = point.x - area.left;
            let y = point.y - area.top;
            if x < 0 || y < 0 || x >= area.right - area.left || y >= area.bottom - area.top {
                return None;
            }
            if !self.active_areas.iter().any(|&[left, top, right, bottom]| {
                x >= left && x < right && y >= top && y < bottom
            }) {
                return None;
            }
            if self.scale <= 0.0 {
                return None;
            }
            Some((x as f32 / self.scale, y as f32 / self.scale))
        }
    }

    /// Activa o apaga la comprobación periódica del cursor: solo con panel.
    /// Sin panel no hay sondeo y el hilo sigue durmiendo en reposo.
    pub fn set_preview_watch(&mut self, watch: bool) {
        if watch && self.watch_timer.is_none() {
            // SAFETY: temporizador del hilo, sin callback.
            let timer = unsafe { SetTimer(None, 0, WATCH_MILLISECONDS, None) };
            self.watch_timer = Some(timer);
        } else if !watch {
            if let Some(timer) = self.watch_timer.take() {
                // SAFETY: cancela el temporizador creado arriba.
                let _ = unsafe { KillTimer(None, timer) };
            }
        }
    }

    /// Atiende el temporizador de vigilancia, si es el suyo.
    pub(crate) fn watch_event(&self, id: usize) -> Option<SurfaceEvent> {
        (self.watch_timer == Some(id)).then_some(SurfaceEvent::CursorCheck)
    }
}
