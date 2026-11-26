use std::collections::HashMap;
use std::ffi::c_void;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Dwm::{
    DWM_THUMBNAIL_PROPERTIES, DWM_TNP_OPACITY, DWM_TNP_RECTDESTINATION, DWM_TNP_VISIBLE,
    DwmQueryThumbnailSourceSize, DwmRegisterThumbnail, DwmUnregisterThumbnail,
    DwmUpdateThumbnailProperties,
};
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
use windows::core::Result;

/// Vista grande de la ventana bajo el cursor, en su sitio y tamaño reales.
pub(crate) struct LargePreview {
    destination: HWND,
    registered: HashMap<u64, isize>,
}

impl LargePreview {
    pub(crate) fn new(destination: HWND) -> Self {
        Self {
            destination,
            registered: HashMap::new(),
        }
    }

    /// Muestra `source` en `area` (píxeles de pantalla). `false` sin imagen válida.
    pub(crate) fn show(&mut self, source: u64, area: [i32; 4]) -> Result<bool> {
        self.registered.retain(|window, handle| {
            let wanted = *window == source;
            if !wanted {
                // SAFETY: `handle` se registró aquí y se libera una sola vez.
                let _ = unsafe { DwmUnregisterThumbnail(*handle) };
            }
            wanted
        });
        let handle = match self.registered.get(&source) {
            Some(&handle) => handle,
            None => {
                let window = HWND(source as *mut c_void);
                // SAFETY: Windows valida ambos identificadores.
                let Ok(handle) = (unsafe { DwmRegisterThumbnail(self.destination, window) }) else {
                    return Ok(false);
                };
                self.registered.insert(source, handle);
                handle
            }
        };
        // SAFETY: `handle` es un registro vigente.
        let source_size = unsafe { DwmQueryThumbnailSourceSize(handle) }?;
        if source_size.cx <= 0 || source_size.cy <= 0 {
            return Ok(false);
        }
        let mut outer = RECT::default();
        // SAFETY: la ventana destino es propia y `outer` es escribible.
        unsafe { GetWindowRect(self.destination, &mut outer)? };
        let [left, top, right, bottom] = area;
        if right <= outer.left || bottom <= outer.top {
            return Ok(false);
        }
        let properties = DWM_THUMBNAIL_PROPERTIES {
            dwFlags: DWM_TNP_RECTDESTINATION | DWM_TNP_VISIBLE | DWM_TNP_OPACITY,
            rcDestination: RECT {
                left: left - outer.left,
                top: top - outer.top,
                right: right - outer.left,
                bottom: bottom - outer.top,
            },
            opacity: u8::MAX,
            fVisible: true.into(),
            ..Default::default()
        };
        // SAFETY: `properties` indica en `dwFlags` los campos que rellena.
        unsafe { DwmUpdateThumbnailProperties(handle, &properties)? };
        Ok(true)
    }

    pub(crate) fn hide(&mut self) {
        for &handle in self.registered.values() {
            // SAFETY: cada registro se libera una sola vez.
            let _ = unsafe { DwmUnregisterThumbnail(handle) };
        }
        self.registered.clear();
    }
}

impl Drop for LargePreview {
    fn drop(&mut self) {
        self.hide();
    }
}
