//! Gráfica principal por DXGI: el primer adaptador con salida.
//!
//! Si DXGI no responde, queda en `None` y el resto sigue.

use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1};

use super::placeholders::clean;

/// Nombre de la gráfica principal, o `None`.
pub(crate) fn graphics() -> Option<String> {
    // SAFETY: crea la factoría DXGI sin parámetros; no toca memoria propia.
    let factory: IDXGIFactory1 = unsafe { CreateDXGIFactory1().ok()? };
    // El adaptador 0 suele ser la gráfica principal; se prueba con los
    // primeros por si el primero no tiene salida.
    for index in 0..4 {
        // SAFETY: enumera un adaptador existente; si no hay, termina.
        let adapter = unsafe { factory.EnumAdapters1(index).ok()? };
        // SAFETY: la descripción la rellena DXGI en una estructura válida.
        let description = unsafe { adapter.GetDesc1().ok()? };
        let end = description
            .Description
            .iter()
            .position(|unit| *unit == 0)
            .unwrap_or(description.Description.len());
        let text = String::from_utf16_lossy(&description.Description[..end]);
        if let Some(name) = clean(Some(text)) {
            return Some(name);
        }
    }
    None
}
