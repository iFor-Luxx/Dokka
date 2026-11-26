use windows::Win32::Foundation::{HWND, RECT};
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
    let mut work_area = RECT::default();
    // SAFETY: `window` es válida y `work_area` es un `RECT` escribible,
    // como pide SPI_GETWORKAREA.
    unsafe {
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some((&raw mut work_area).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )?;

        let scale = GetDpiForWindow(window) as f32 / BASE_DPI;
        let width = (options.width * scale).round() as i32;
        let height = (options.height * scale).round() as i32;
        let margin = (options.bottom_margin * scale).round() as i32;
        let x = work_area.left + (work_area.right - work_area.left - width) / 2;
        let y = work_area.bottom - height - margin;

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
