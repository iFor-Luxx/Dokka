//! Copia de pantalla con la gráfica (Desktop Duplication).
//!
//! La imagen ya está en la gráfica: solo se recorta el rectángulo y se baja
//! a memoria. El cursor no forma parte de ella, así que no parpadea.

use std::time::{Duration, Instant};

use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_UNKNOWN;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_BOX, D3D11_CPU_ACCESS_READ, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_MAP_READ,
    D3D11_MAPPED_SUBRESOURCE, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_MODE_ROTATION_IDENTITY, DXGI_MODE_ROTATION_UNSPECIFIED,
    DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, DXGI_ERROR_NOT_FOUND, DXGI_OUTDUPL_FRAME_INFO, IDXGIAdapter1,
    IDXGIFactory1, IDXGIOutput1, IDXGIOutputDuplication, IDXGIResource,
};
use windows::core::{Error, Interface, Result};

use crate::GenieImage;
use crate::geometry::PixelRect;

/// Cuánto se espera como mucho a que llegue una imagen completa.
const FRAME_DEADLINE: Duration = Duration::from_millis(40);

/// Dispositivo ligado a un monitor. La duplicación se abre en cada copia:
/// no queda nada vigilando la pantalla entre vuelos (ni estorba a juegos).
pub(crate) struct DesktopCopier {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    output: IDXGIOutput1,
    bounds: PixelRect,
}

impl DesktopCopier {
    /// Prepara el monitor que contiene la esquina central de `rect`.
    pub(crate) fn for_rect(rect: PixelRect) -> Result<Self> {
        let (x, y) = (rect.left + rect.width() / 2, rect.top + rect.height() / 2);
        // SAFETY: enumeración de solo lectura; cada interfaz se libera sola.
        unsafe {
            let factory: IDXGIFactory1 = CreateDXGIFactory1()?;
            for adapter_index in 0.. {
                let adapter: IDXGIAdapter1 = factory.EnumAdapters1(adapter_index)?;
                for output_index in 0.. {
                    let Ok(output) = adapter.EnumOutputs(output_index) else {
                        break;
                    };
                    let desc = output.GetDesc()?;
                    let area = PixelRect::from_win(desc.DesktopCoordinates);
                    if x < area.left || x >= area.right || y < area.top || y >= area.bottom {
                        continue;
                    }
                    // Un monitor girado entrega la imagen sin girar.
                    if desc.Rotation != DXGI_MODE_ROTATION_IDENTITY
                        && desc.Rotation != DXGI_MODE_ROTATION_UNSPECIFIED
                    {
                        return Err(Error::from(DXGI_ERROR_NOT_FOUND));
                    }
                    let (mut device, mut context) = (None, None);
                    D3D11CreateDevice(
                        &adapter,
                        D3D_DRIVER_TYPE_UNKNOWN,
                        HMODULE::default(),
                        D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                        None,
                        D3D11_SDK_VERSION,
                        Some(&mut device),
                        None,
                        Some(&mut context),
                    )?;
                    let (Some(device), Some(context)) = (device, context) else {
                        return Err(Error::from(DXGI_ERROR_NOT_FOUND));
                    };
                    return Ok(Self {
                        device,
                        context,
                        output: output.cast()?,
                        bounds: area,
                    });
                }
            }
        }
        Err(Error::from(DXGI_ERROR_NOT_FOUND))
    }

    /// `true` si `rect` cae entero en el monitor preparado.
    pub(crate) fn holds(&self, rect: PixelRect) -> bool {
        rect.left >= self.bounds.left
            && rect.top >= self.bounds.top
            && rect.right <= self.bounds.right
            && rect.bottom <= self.bounds.bottom
    }

    /// Foto BGRA opaca de `rect` (píxeles de pantalla, dentro del monitor).
    pub(crate) fn copy(&self, rect: PixelRect) -> Result<GenieImage> {
        // SAFETY: llamada sobre interfaces vivas y propias de este hilo.
        let duplication = unsafe { self.output.DuplicateOutput(&self.device)? };
        let deadline = Instant::now() + FRAME_DEADLINE;
        loop {
            let mut info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut resource: Option<IDXGIResource> = None;
            // SAFETY: ambas salidas son escribibles; cada fotograma adquirido
            // se suelta una vez más abajo.
            unsafe { duplication.AcquireNextFrame(10, &mut info, &mut resource) }.or_else(
                |error| {
                    if Instant::now() < deadline {
                        Ok(())
                    } else {
                        Err(error)
                    }
                },
            )?;
            let Some(resource) = resource else {
                continue;
            };
            // Sin hora de presentación solo cambió el ratón: la imagen aún no vale.
            if info.LastPresentTime == 0 {
                release(&duplication);
                if Instant::now() >= deadline {
                    return Err(Error::from(DXGI_ERROR_NOT_FOUND));
                }
                continue;
            }
            let image = resource
                .cast::<ID3D11Texture2D>()
                .and_then(|frame| self.read(&frame, rect));
            release(&duplication);
            return image;
        }
    }

    /// Recorta `rect` en la gráfica y lo baja a memoria.
    fn read(&self, frame: &ID3D11Texture2D, rect: PixelRect) -> Result<GenieImage> {
        let (width, height) = (rect.width() as u32, rect.height() as u32);
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: 0,
            CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
            MiscFlags: 0,
        };
        let left = (rect.left - self.bounds.left) as u32;
        let top = (rect.top - self.bounds.top) as u32;
        let area = D3D11_BOX {
            left,
            top,
            front: 0,
            right: left + width,
            bottom: top + height,
            back: 1,
        };
        let mut bgra = vec![255u8; width as usize * height as usize * 4];
        // SAFETY: la textura intermedia tiene el tamaño del recorte; tras
        // `Map` cada fila mide `RowPitch` y se leen `width * 4` bytes de
        // `height` filas antes de `Unmap`.
        unsafe {
            let mut staging = None;
            self.device
                .CreateTexture2D(&desc, None, Some(&mut staging))?;
            let Some(staging) = staging else {
                return Err(Error::from(DXGI_ERROR_NOT_FOUND));
            };
            self.context
                .CopySubresourceRegion(&staging, 0, 0, 0, 0, frame, 0, Some(&area));
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            self.context
                .Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped))?;
            let row = width as usize * 4;
            for (y, line) in bgra.chunks_exact_mut(row).enumerate() {
                let source = mapped.pData.cast::<u8>().add(y * mapped.RowPitch as usize);
                line.copy_from_slice(std::slice::from_raw_parts(source, row));
                line.as_chunks_mut::<4>().0.iter_mut().for_each(|pixel| {
                    pixel[3] = 255;
                });
            }
            self.context.Unmap(&staging, 0);
        }
        Ok(GenieImage {
            width,
            height,
            bgra,
        })
    }
}

/// Suelta el fotograma adquirido; un fallo aquí no cambia nada.
fn release(duplication: &IDXGIOutputDuplication) {
    // SAFETY: se llama una vez por cada fotograma adquirido.
    unsafe {
        let _ = duplication.ReleaseFrame();
    }
}
