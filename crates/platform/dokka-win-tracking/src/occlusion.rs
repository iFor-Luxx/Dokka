//! Si la ventana al frente ocupa el monitor principal: maximizada o a
//! pantalla completa. El escritorio, las ventanas normales y las de Dokka
//! no cuentan. Si no se puede saber, `false` y el dock queda visible.

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Dwm::{DWMWA_EXTENDED_FRAME_BOUNDS, DwmGetWindowAttribute};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::MONITORINFOF_PRIMARY;
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetForegroundWindow, GetWindowRect, GetWindowThreadProcessId, IsZoomed,
};

/// `true` si una app ajena ocupa todo el monitor principal.
pub fn foreground_occludes_primary() -> bool {
    let Some(window) = foreground_window() else {
        return false;
    };
    if is_desktop(window) {
        return false;
    }
    let Some(monitor) = monitor_of(window) else {
        return false;
    };
    if !is_primary(&monitor) {
        return false;
    }
    // SAFETY: consultas de solo lectura sobre la ventana al frente.
    unsafe {
        if IsZoomed(window).as_bool() {
            return true;
        }
    }
    let Some(frame) = frame_of(window) else {
        return false;
    };
    frame.left <= monitor.left
        && frame.top <= monitor.top
        && frame.right >= monitor.right
        && frame.bottom >= monitor.bottom
}

/// Ventana al frente si no es de Dokka; `None` si no se puede saber.
fn foreground_window() -> Option<HWND> {
    // SAFETY: consulta sin argumentos; el handle se valida antes de usarse.
    let window = unsafe { GetForegroundWindow() };
    if window.is_invalid() {
        return None;
    }
    let mut owner = 0;
    // SAFETY: `owner` es escribible durante la llamada.
    unsafe { GetWindowThreadProcessId(window, Some(&mut owner)) };
    // SAFETY: solo consulta el identificador del proceso actual.
    if owner == unsafe { GetCurrentProcessId() } {
        return None;
    }
    Some(window)
}

/// Rectángulo completo del monitor de la ventana, si se puede saber.
fn monitor_of(window: HWND) -> Option<RECT> {
    // SAFETY: `window` es la ventana al frente y `info` declara su tamaño.
    unsafe {
        let monitor = MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(monitor, &mut info).as_bool() {
            return None;
        }
        Some(info.rcMonitor)
    }
}

/// `true` si el rectángulo es del monitor principal.
fn is_primary(monitor: &RECT) -> bool {
    // SAFETY: punto dentro del monitor dado; la estructura declara su tamaño.
    unsafe {
        let point = windows::Win32::Foundation::POINT {
            x: (monitor.left + monitor.right) / 2,
            y: (monitor.top + monitor.bottom) / 2,
        };
        let handle =
            windows::Win32::Graphics::Gdi::MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(handle, &mut info).as_bool() {
            return false;
        }
        info.dwFlags & MONITORINFOF_PRIMARY != 0
    }
}

/// Marco real de la ventana: bordes extendidos de DWM si se puede, si no el
/// rectángulo clásico. `None` si no se puede saber (falla sola: visible).
fn frame_of(window: HWND) -> Option<RECT> {
    let mut frame = RECT::default();
    // SAFETY: `frame` es escribible y se declara su tamaño exacto.
    let extended = unsafe {
        DwmGetWindowAttribute(
            window,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            (&raw mut frame).cast(),
            size_of::<RECT>() as u32,
        )
    };
    if extended.is_ok() {
        return Some(frame);
    }
    // SAFETY: `frame` es escribible durante la llamada.
    if unsafe { GetWindowRect(window, &mut frame) }.is_err() {
        return None;
    }
    Some(frame)
}

fn is_desktop(window: HWND) -> bool {
    let mut name = [0_u16; 64];
    // SAFETY: Windows no escribe más allá del tamaño del búfer entregado.
    let length = unsafe { GetClassNameW(window, &mut name) };
    let class = String::from_utf16_lossy(&name[..length.max(0) as usize]);
    class == "Progman" || class == "WorkerW"
}
