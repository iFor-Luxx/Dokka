use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTOPRIMARY, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_TOPMOST, SPI_GETWORKAREA, SWP_NOACTIVATE, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    SetWindowPos, SystemParametersInfoW,
};
use windows::core::Result;

use crate::SurfaceOptions;

const BASE_DPI: f32 = 96.0;

/// Coloca la ventana en el monitor principal y devuelve la escala usada.
pub(crate) fn place_at_bottom_center(window: HWND, options: SurfaceOptions) -> Result<f32> {
    let base = base_area(window, options.over_taskbar)?;
    // SAFETY: `window` es válida y la escala sale de su DPI real.
    unsafe {
        let scale = GetDpiForWindow(window) as f32 / BASE_DPI;
        let width = (options.width * scale).round() as i32;
        let height = (options.height * scale).round() as i32;
        let margin = (options.bottom_margin * scale).round() as i32;
        let x = base.left + (base.right - base.left - width) / 2;
        let y = base.bottom - height - margin;

        SetWindowPos(
            window,
            Some(HWND_TOPMOST),
            x,
            y,
            width,
            height,
            SWP_NOACTIVATE,
        )?;
        Ok(scale)
    }
}

/// Rectángulo donde anclar el dock: el monitor completo si se tapa la barra,
/// o el área de trabajo si la barra sigue visible.
fn base_area(window: HWND, over_taskbar: bool) -> Result<RECT> {
    if over_taskbar {
        // SAFETY: `window` es válida y `info` es un `MONITORINFO` con su tamaño.
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
    } else {
        // SAFETY: `work_area` es un `RECT` escribible, como pide SPI_GETWORKAREA.
        unsafe {
            let mut work_area = RECT::default();
            SystemParametersInfoW(
                SPI_GETWORKAREA,
                0,
                Some((&raw mut work_area).cast()),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            )?;
            Ok(work_area)
        }
    }
}
