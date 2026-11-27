use std::collections::HashMap;
use std::ffi::c_void;

use dokka_scene::{Rect, Thumbnail};
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

/// Ventana de origen y cuántas veces apareció antes en la misma escena.
type Key = (u64, usize);

/// Miniaturas en vivo que Windows compone sobre la ventana de Dokka.
pub(crate) struct WindowThumbnails {
    destination: HWND,
    /// Píxeles por unidad lógica.
    scale: f32,
    registered: HashMap<Key, isize>,
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
    /// Las del panel comparten la opacidad del panel; las del dock van opacas.
    /// Las del panel se escalan sobre el icono con la misma transformación
    /// que el contenedor del panel; las del dock no se tocan.
    pub(crate) fn show(
        &mut self,
        dock: &[Thumbnail],
        floating: &[Thumbnail],
        floating_opacity: f32,
        floating_scale: f32,
        floating_origin: (f32, f32),
    ) -> Result<()> {
        let opacity = (floating_opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
        let scaled: Vec<Thumbnail> = floating
            .iter()
            .map(|item| scaled_thumbnail(item, floating_scale, floating_origin))
            .collect();
        let all: Vec<(&Thumbnail, u8)> = dock
            .iter()
            .map(|item| (item, (item.opacity.clamp(0.0, 1.0) * 255.0).round() as u8))
            .chain(scaled.iter().map(|item| (item, opacity)))
            .collect();
        // Una misma ventana puede verse en el dock y en un panel a la vez.
        let keys: Vec<Key> = all
            .iter()
            .enumerate()
            .map(|(index, (item, _))| {
                let before = all[..index]
                    .iter()
                    .filter(|(other, _)| other.window == item.window)
                    .count();
                (item.window, before)
            })
            .collect();
        self.registered.retain(|key, handle| {
            let wanted = keys.contains(key);
            if !wanted {
                // SAFETY: `handle` se registró aquí y se libera una sola vez.
                let _ = unsafe { DwmUnregisterThumbnail(*handle) };
            }
            wanted
        });

        for (key, (thumbnail, opacity)) in keys.into_iter().zip(all) {
            let handle = match self.registered.get(&key) {
                Some(&handle) => handle,
                None => {
                    let source = HWND(thumbnail.window as *mut c_void);
                    // SAFETY: Windows valida ambos identificadores.
                    // Una ventana que ya se cerró simplemente no se muestra.
                    let Ok(handle) = (unsafe { DwmRegisterThumbnail(self.destination, source) })
                    else {
                        continue;
                    };
                    self.registered.insert(key, handle);
                    handle
                }
            };
            self.place(handle, thumbnail, opacity)?;
        }
        Ok(())
    }

    fn place(&self, handle: isize, thumbnail: &Thumbnail, opacity: u8) -> Result<()> {
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
            opacity,
            fVisible: true.into(),
            ..Default::default()
        };
        // SAFETY: `properties` indica en `dwFlags` los campos que rellena.
        unsafe { DwmUpdateThumbnailProperties(handle, &properties) }
    }
}

/// Misma transformación que el contenedor del panel: escala sobre el icono.
fn scaled_thumbnail(item: &Thumbnail, scale: f32, origin: (f32, f32)) -> Thumbnail {
    Thumbnail {
        window: item.window,
        bounds: Rect {
            x: origin.0 + (item.bounds.x - origin.0) * scale,
            y: origin.1 + (item.bounds.y - origin.1) * scale,
            width: item.bounds.width * scale,
            height: item.bounds.height * scale,
        },
        reserved_corner: item.reserved_corner * scale,
        opacity: item.opacity,
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
