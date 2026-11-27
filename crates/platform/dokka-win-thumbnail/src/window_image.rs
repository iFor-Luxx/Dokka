//! Miniatura viva de DWM entregada como visual de composición.

use std::ffi::c_void;
use std::sync::OnceLock;

use windows::UI::Composition::Visual;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::DirectComposition::IDCompositionVisual;
use windows::Win32::Graphics::Dwm::{DWM_THUMBNAIL_PROPERTIES, DwmUnregisterThumbnail};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::core::{Error, HRESULT, IUnknown, Interface, PCSTR, Result, w};
use windows_numerics::Vector2;

use crate::LiveCompositor;

/// `DwmpCreateSharedThumbnailVisual`, exportada solo por número.
const ORDINAL: usize = 147;
/// Tipo de miniatura que admite verse como visual.
const SHARED_VISUAL: u32 = 2;
/// Destino, origen, opacidad, visible, solo cliente y `ENABLE3D`.
const FLAGS: u32 = 0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x0400_0000;

type CreateThumbnailVisual = unsafe extern "system" fn(
    HWND,
    HWND,
    u32,
    *const DWM_THUMBNAIL_PROPERTIES,
    *mut c_void,
    *mut *mut c_void,
    *mut isize,
) -> HRESULT;

/// Imagen viva de una ventana, a su tamaño real. Al soltarla se da de baja.
pub struct LiveImage {
    visual: Visual,
    thumbnail: isize,
}

impl LiveImage {
    /// `host` es la ventana propia donde se dibujará; `source`, la mostrada,
    /// de `width` × `height` píxeles contando sus bordes invisibles.
    pub fn new(
        compositor: &LiveCompositor,
        host: isize,
        source: isize,
        (width, height): (i32, i32),
    ) -> Result<Self> {
        let create = create_function().ok_or_else(Error::from_thread)?;
        let full = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };
        let properties = DWM_THUMBNAIL_PROPERTIES {
            dwFlags: FLAGS,
            rcDestination: full,
            rcSource: full,
            opacity: 255,
            fVisible: true.into(),
            fSourceClientAreaOnly: false.into(),
        };
        let mut raw: *mut c_void = std::ptr::null_mut();
        let mut thumbnail = 0isize;
        // SAFETY: las entradas viven durante la llamada y las dos salidas son
        // locales; la firma es la del ejemplo público.
        let child = unsafe {
            create(
                HWND(host as *mut c_void),
                HWND(source as *mut c_void),
                SHARED_VISUAL,
                &properties,
                compositor.device.as_raw(),
                &mut raw,
                &mut thumbnail,
            )
            .ok()?;
            if raw.is_null() {
                return Err(Error::from_thread());
            }
            IUnknown::from_raw(raw)
        };
        let wrapped = || -> Result<Visual> {
            let child: IDCompositionVisual = child.cast()?;
            // SAFETY: ambos visuales son del mismo dispositivo y están vivos.
            let holder = unsafe {
                let holder = compositor.device.CreateVisual()?;
                holder.AddVisual(&child, true, None)?;
                holder
            };
            let visual: Visual = holder.cast()?;
            visual.SetSize(Vector2 {
                X: width as f32,
                Y: height as f32,
            })?;
            Ok(visual)
        };
        match wrapped() {
            Ok(visual) => Ok(Self { visual, thumbnail }),
            Err(error) => {
                release(thumbnail);
                Err(error)
            }
        }
    }

    pub fn visual(&self) -> &Visual {
        &self.visual
    }
}

impl Drop for LiveImage {
    fn drop(&mut self) {
        release(self.thumbnail);
    }
}

fn release(thumbnail: isize) {
    // SAFETY: registro propio, dado de baja una sola vez.
    unsafe {
        let _ = DwmUnregisterThumbnail(thumbnail);
    }
}

/// Busca la función una sola vez; `None` si este Windows no la trae.
fn create_function() -> Option<CreateThumbnailVisual> {
    static FUNCTION: OnceLock<Option<CreateThumbnailVisual>> = OnceLock::new();
    *FUNCTION.get_or_init(|| {
        // SAFETY: `dwmapi.dll` queda cargada mientras viva el proceso; un
        // ordinal ausente devuelve `None` antes de convertir el puntero.
        unsafe {
            let library = LoadLibraryW(w!("dwmapi.dll")).ok()?;
            let function = GetProcAddress(library, PCSTR(ORDINAL as *const u8))?;
            Some(std::mem::transmute::<
                unsafe extern "system" fn() -> isize,
                CreateThumbnailVisual,
            >(function))
        }
    })
}
