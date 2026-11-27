//! Pantalla completa (vídeos, juegos, presentaciones): ahí no hay genio.

use std::ffi::c_void;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, IsIconic, IsZoomed};

use crate::geometry::{PixelRect, monitor_rect_for_window};

/// `true` si `frame` tapa el monitor entero, sin dejar ni la barra.
pub(crate) fn covers(frame: PixelRect, screen: PixelRect) -> bool {
    frame.left <= screen.left
        && frame.top <= screen.top
        && frame.right >= screen.right
        && frame.bottom >= screen.bottom
}

/// Ventana abierta a pantalla completa ahora mismo. Una maximizada normal no
/// cuenta. Si no se puede saber, `false`.
pub(crate) fn is_fullscreen(window: isize) -> bool {
    let handle = HWND(window as *mut c_void);
    let mut area = RECT::default();
    // SAFETY: consultas de solo lectura; `area` es escribible y un handle
    // destruido hace fallar la llamada.
    unsafe {
        if IsIconic(handle).as_bool()
            || IsZoomed(handle).as_bool()
            || GetWindowRect(handle, &mut area).is_err()
        {
            return false;
        }
    }
    let [left, top, right, bottom] = monitor_rect_for_window(window);
    covers(
        PixelRect::from_win(area),
        PixelRect {
            left,
            top,
            right,
            bottom,
        },
    )
}
