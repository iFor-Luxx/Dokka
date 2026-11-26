//! Pide el icono grande del archivo que declara un `.url`.
//!
//! `SHDefExtractIconW` devuelve el tamaño pedido cuando existe; si no,
//! `ExtractIconExW` da el grande del sistema y se escala al dibujar.

use std::path::Path;

use dokka_scene::Bitmap;
use windows::Win32::UI::Shell::{ExtractIconExW, SHDefExtractIconW};
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, HICON};
use windows::core::{HSTRING, Result};

use crate::hicon_bitmap::read_hicon;

/// Icono del archivo al lado pedido, o error para usar el genérico.
pub(crate) fn load_file_icon(path: &Path, index: i32, side: i32) -> Result<Bitmap> {
    if let Some(bitmap) = from_shdef(path, index, side) {
        return Ok(bitmap);
    }
    let bitmap = from_extract(path, index, side)?;
    Ok(bitmap)
}

fn from_shdef(path: &Path, index: i32, side: i32) -> Option<Bitmap> {
    let name = HSTRING::from(path.as_os_str());
    let mut large = HICON::default();
    // SAFETY: `large` recibe el icono y se destruye tras convertirlo.
    let done = unsafe {
        SHDefExtractIconW(
            &name,
            index,
            0,
            Some(&mut large as *mut _),
            None,
            side as u32,
        )
        .is_ok()
            && !large.is_invalid()
    };
    if !done {
        return None;
    }
    let bitmap = read_hicon(large, side).ok();
    // SAFETY: el icono lo creó `SHDefExtractIconW` en esta llamada.
    unsafe {
        let _ = DestroyIcon(large);
    }
    bitmap
}

fn from_extract(path: &Path, index: i32, side: i32) -> Result<Bitmap> {
    let name = HSTRING::from(path.as_os_str());
    let mut large = HICON::default();
    // SAFETY: `large` recibe el icono y se destruye tras convertirlo.
    let (count, valid) = unsafe {
        let count = ExtractIconExW(&name, index, Some(&mut large as *mut _), None, 1);
        (count, !large.is_invalid())
    };
    if count == 0 || !valid {
        // SAFETY: si hay icono aunque falle el recuento, no se pierde.
        unsafe {
            if valid {
                let _ = DestroyIcon(large);
            }
        }
        return Err(windows::core::Error::from_thread());
    }
    let bitmap = read_hicon(large, side);
    // SAFETY: el icono lo creó `ExtractIconExW` en esta llamada.
    unsafe {
        let _ = DestroyIcon(large);
    }
    bitmap
}
