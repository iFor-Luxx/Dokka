//! Pantalla principal: resolución y nombre del monitor.
//!
//! La resolución sale de `GetSystemMetrics`; el nombre del primer
//! dispositivo activo. Si algo falla, queda en `None`.

use windows::Win32::Graphics::Gdi::{DISPLAY_DEVICEW, EnumDisplayDevicesW};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use windows::core::PCWSTR;

use super::placeholders::clean;

/// Resolución como "1920 × 1080", o `None`.
pub(crate) fn resolution() -> Option<String> {
    // SAFETY: pide dos métricas del sistema; no toca memoria.
    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    // SAFETY: igual que arriba.
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    if width <= 0 || height <= 0 {
        return None;
    }
    Some(format!("{width} × {height}"))
}

/// Nombre del monitor principal ("Monitor PnP genérico"), o `None`.
pub(crate) fn display_name() -> Option<String> {
    let mut device = DISPLAY_DEVICEW {
        cb: size_of::<DISPLAY_DEVICEW>() as u32,
        ..Default::default()
    };
    // SAFETY: `device` declara su tamaño y es escribible; índice 0 válido.
    let ok = unsafe { EnumDisplayDevicesW(PCWSTR::null(), 0, &mut device, 0) };
    if !ok.as_bool() {
        return None;
    }
    let end = device
        .DeviceString
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(device.DeviceString.len());
    clean(Some(String::from_utf16_lossy(&device.DeviceString[..end])))
}
