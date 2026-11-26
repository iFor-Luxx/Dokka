use std::collections::HashMap;
use std::ffi::c_void;

use dokka_scene::Thumbnail;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Dwm::{
    DWM_THUMBNAIL_PROPERTIES, DWM_TNP_OPACITY, DWM_TNP_RECTDESTINATION, DWM_TNP_VISIBLE,
    DwmQueryThumbnailSourceSize, DwmRegisterThumbnail, DwmUnregisterThumbnail,
    DwmUpdateThumbnailProperties,
};
use windows::core::Result;

use crate::fit_thumbnail::fit_thumbnail;

/// Proporción usada si la ventana no informa su tamaño.
const FALLBACK_ASPECT: f32 = 1.6;

/// Miniaturas en vivo que Windows compone sobre la ventana de Dokka.
pub(crate) struct WindowThumbnails {
    destination: HWND,
    /// Píxeles por unidad lógica.
    scale: f32,
    /// Registro de Windows por ventana de origen.
    registered: HashMap<u64, isize>,
}

impl WindowThumbnails {
    pub(crate) fn new(destination: HWND, scale: f32) -> Self {
        Self {
            destination,
            scale,
            registered: HashMap::new(),
        }
    }

    /// Deja visibles exactamente las miniaturas indicadas, en su sitio.
    pub(crate) fn show(&mut self, thumbnails: &[Thumbnail]) -> Result<()> {
        self.registered.retain(|window, handle| {
            let wanted = thumbnails.iter().any(|item| item.window == *window);
            if !wanted {
                // SAFETY: `handle` se registró aquí y se libera una sola vez.
                let _ = unsafe { DwmUnregisterThumbnail(*handle) };
            }
            wanted
        });

        for thumbnail in thumbnails {
            let handle = match self.registered.get(&thumbnail.window) {
                Some(&handle) => handle,
                None => {
                    let source = HWND(thumbnail.window as *mut c_void);
                    // SAFETY: Windows valida ambos identificadores.
                    // Una ventana que ya se cerró simplemente no se muestra.
                    let Ok(handle) = (unsafe { DwmRegisterThumbnail(self.destination, source) })
                    else {
                        continue;
                    };
                    self.registered.insert(thumbnail.window, handle);
                    handle
                }
            };
            self.place(handle, thumbnail)?;
        }
        Ok(())
    }

    fn place(&self, handle: isize, thumbnail: &Thumbnail) -> Result<()> {
        // SAFETY: `handle` es un registro vigente.
        let source = unsafe { DwmQueryThumbnailSourceSize(handle) }?;
        let aspect = if source.cx > 0 && source.cy > 0 {
            source.cx as f32 / source.cy as f32
        } else {
            FALLBACK_ASPECT
        };
        let area = fit_thumbnail(thumbnail, aspect);
        let pixels = |value: f32| (value * self.scale).round() as i32;
        let properties = DWM_THUMBNAIL_PROPERTIES {
            dwFlags: DWM_TNP_RECTDESTINATION | DWM_TNP_VISIBLE | DWM_TNP_OPACITY,
            rcDestination: RECT {
                left: pixels(area.x),
                top: pixels(area.y),
                right: pixels(area.x + area.width),
                bottom: pixels(area.y + area.height),
            },
            opacity: u8::MAX,
            fVisible: true.into(),
            ..Default::default()
        };
        // SAFETY: `properties` indica en `dwFlags` los campos que rellena.
        unsafe { DwmUpdateThumbnailProperties(handle, &properties) }
    }
}

impl Drop for WindowThumbnails {
    fn drop(&mut self) {
        for &handle in self.registered.values() {
            // SAFETY: cada registro se libera una sola vez.
            let _ = unsafe { DwmUnregisterThumbnail(handle) };
        }
    }
}
