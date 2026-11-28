//! Foto de fondo del instalador decodificada con WIC de Windows.
//!
//! La app embebe `assets/installer/playa.jpg` y la pasa como bytes; aquí se
//! deja en el `Bitmap` propio de Dokka (BGRA premultiplicado). Si falla algo,
//! devuelve `None` y la ventana usa el color crema de respaldo.

use dokka_scene::Bitmap;
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppPBGRA, IWICImagingFactory, IWICPalette,
    WICBitmapDitherTypeNone, WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnLoad,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance,
};

/// Lado máximo admitido para no reservar una imagen absurda.
const MAX_SIDE: u32 = 4096;

/// Decodifica la foto (JPEG o PNG) a píxeles BGRA premultiplicados.
/// `None` con bytes vacíos o si Windows no la puede leer.
pub(crate) fn decode_background(bytes: &[u8]) -> Option<Bitmap> {
    if bytes.is_empty() {
        return None;
    }
    // SAFETY: COM del hilo para WIC; si ya estaba iniciado, se sigue igual.
    unsafe {
        let _ = windows::Win32::System::Com::CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
    let factory: IWICImagingFactory =
        // SAFETY: crea la factoría WIC en este hilo.
        unsafe { CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).ok()? };
    // SAFETY: la factoría es válida; el flujo vive de estos bytes.
    let bitmap = unsafe {
        let stream = factory.CreateStream().ok()?;
        stream.InitializeFromMemory(bytes).ok()?;
        let decoder = factory
            .CreateDecoderFromStream(&stream, std::ptr::null(), WICDecodeMetadataCacheOnLoad)
            .ok()?;
        let frame = decoder.GetFrame(0).ok()?;
        let mut width = 0;
        let mut height = 0;
        frame.GetSize(&mut width, &mut height).ok()?;
        if width == 0 || height == 0 || width > MAX_SIDE || height > MAX_SIDE {
            return None;
        }
        let converter = factory.CreateFormatConverter().ok()?;
        converter
            .Initialize(
                &frame,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None::<&IWICPalette>,
                0.0,
                WICBitmapPaletteTypeCustom,
            )
            .ok()?;
        let stride = width.checked_mul(4)?;
        let size = (height as usize).checked_mul(stride as usize)?;
        let mut pixels = vec![0u8; size];
        converter
            .CopyPixels(std::ptr::null(), stride, &mut pixels)
            .ok()?;
        Bitmap {
            width,
            height,
            pixels,
        }
    };
    Some(bitmap)
}
