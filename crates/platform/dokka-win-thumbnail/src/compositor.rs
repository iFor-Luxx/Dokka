//! Compositor que además es dispositivo DirectComposition, requisito para
//! recibir miniaturas como visuales.

use std::ffi::c_void;

use windows::UI::Composition::{CompositionTarget, Compositor};
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::Graphics::Direct2D::{D2D1CreateDevice, ID2D1Device};
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice,
};
use windows::Win32::Graphics::DirectComposition::{
    IDCompositionDesktopDevice, IDCompositionTarget,
};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::System::WinRT::RoGetActivationFactory;
use windows::core::{Error, GUID, HRESULT, HSTRING, IInspectable, IUnknown, Interface, Result};

/// `IInteropCompositorFactoryPartner` (privada).
const FACTORY_PARTNER: GUID = GUID::from_u128(0x22118adf_23f1_4801_bcfa_66cbf48cc51b);
/// `IInteropCompositorPartner` (privada).
const COMPOSITOR_PARTNER: GUID = GUID::from_u128(0xe7894c70_af56_4f52_b382_4b3cd263dc6f);
/// Ranura de `CreateInteropCompositor`: tras `IUnknown` (3) e `IInspectable` (3).
const CREATE_SLOT: usize = 6;

type CreateInteropCompositor = unsafe extern "system" fn(
    *mut c_void,
    *mut c_void,
    *mut c_void,
    *const GUID,
    *mut *mut c_void,
) -> HRESULT;

/// Destino de composición de una ventana; mientras viva, se ve su árbol.
pub struct LiveTarget {
    _device_target: IDCompositionTarget,
    target: CompositionTarget,
}

impl LiveTarget {
    pub fn target(&self) -> &CompositionTarget {
        &self.target
    }
}

pub struct LiveCompositor {
    compositor: Compositor,
    pub(crate) device: IDCompositionDesktopDevice,
}

impl LiveCompositor {
    /// El hilo necesita su cola de despacho, como cualquier compositor.
    pub fn new() -> Result<Self> {
        let d2d = graphics_device()?;
        let class = HSTRING::from("Windows.UI.Composition.Compositor");
        // SAFETY: la factoría se pide por su nombre de clase; cada puntero de
        // salida es local y se envuelve al momento en una interfaz propia. La
        // firma y la ranura vienen del ejemplo público de la interfaz privada.
        let partner = unsafe {
            let factory: IInspectable = RoGetActivationFactory(&class)?;
            let mut raw_factory: *mut c_void = std::ptr::null_mut();
            factory.query(&FACTORY_PARTNER, &mut raw_factory).ok()?;
            if raw_factory.is_null() {
                return Err(Error::from_thread());
            }
            // Suelta esta referencia al salir.
            let factory = IUnknown::from_raw(raw_factory);
            let vtable = *(factory.as_raw() as *const *const usize);
            let create: CreateInteropCompositor = std::mem::transmute(*vtable.add(CREATE_SLOT));
            let mut raw: *mut c_void = std::ptr::null_mut();
            create(
                factory.as_raw(),
                d2d.as_raw(),
                std::ptr::null_mut(),
                &COMPOSITOR_PARTNER,
                &mut raw,
            )
            .ok()?;
            if raw.is_null() {
                return Err(Error::from_thread());
            }
            IUnknown::from_raw(raw)
        };
        Ok(Self {
            compositor: partner.cast()?,
            device: partner.cast()?,
        })
    }

    pub fn compositor(&self) -> &Compositor {
        &self.compositor
    }

    /// Enlaza una ventana propia para dibujar en ella.
    pub fn target_for(&self, window: isize) -> Result<LiveTarget> {
        // SAFETY: la ventana es del llamante y sigue viva.
        let device_target = unsafe {
            self.device
                .CreateTargetForHwnd(HWND(window as *mut c_void), true)?
        };
        Ok(LiveTarget {
            target: device_target.cast()?,
            _device_target: device_target,
        })
    }

    /// Hace visibles los cambios hechos desde la última vez.
    pub fn commit(&self) -> Result<()> {
        // SAFETY: dispositivo propio y vivo.
        unsafe { self.device.Commit() }
    }
}

fn graphics_device() -> Result<ID2D1Device> {
    let mut d3d = None;
    // SAFETY: D3D solo escribe en su ranura de salida.
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut d3d),
            None,
            None,
        )?;
    }
    let dxgi: IDXGIDevice = d3d.ok_or_else(Error::from_thread)?.cast()?;
    // SAFETY: `dxgi` es una interfaz viva del dispositivo recién creado.
    unsafe { D2D1CreateDevice(&dxgi, None) }
}
