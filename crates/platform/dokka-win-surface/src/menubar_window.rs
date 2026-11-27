use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTOPRIMARY, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::core::Result;

const BASE_DPI: f32 = 96.0;

/// Rectángulo completo del monitor principal en píxeles.
pub(crate) fn primary_monitor_rect(window: HWND) -> Result<RECT> {
    // SAFETY: `window` es válida y `info` declara su propio tamaño.
    unsafe {
        let monitor = MonitorFromWindow(window, MONITOR_DEFAULTTOPRIMARY);
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(monitor, &mut info).as_bool() {
            return Err(windows::core::Error::from_thread());
        }
        Ok(info.rcMonitor)
    }
}

/// Escala de la ventana de la barra (igual que la del dock en el principal).
pub(crate) fn menubar_dpi_scale(window: HWND) -> f32 {
    // SAFETY: `window` es válida y su DPI es legible.
    unsafe { GetDpiForWindow(window) as f32 / BASE_DPI }
}
