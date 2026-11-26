use windows::UI::Composition::{CompositionGraphicsDevice, Compositor};
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct2D::{
    D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1CreateFactory, ID2D1Factory1,
};
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device,
};
use windows::Win32::Graphics::Dxgi::{IDXGIAdapter, IDXGIDevice};
use windows::Win32::System::WinRT::Composition::ICompositorInterop;
use windows::core::{Error, Interface, Result};

/// Dispositivo con el que se pintan las imágenes que muestra el compositor.
pub(crate) fn create_graphics_device(compositor: &Compositor) -> Result<CompositionGraphicsDevice> {
    let mut device: Option<ID3D11Device> = None;
    // SAFETY: los punteros de salida son válidos y el resto de argumentos son los por defecto.
    unsafe {
        D3D11CreateDevice(
            None::<&IDXGIAdapter>,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        )?;
    }
    let dxgi: IDXGIDevice = device.ok_or_else(Error::from_thread)?.cast()?;
    let interop: ICompositorInterop = compositor.cast()?;
    // SAFETY: la fábrica y los dispositivos se usan solo en este hilo.
    unsafe {
        let factory: ID2D1Factory1 = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
        interop.CreateGraphicsDevice(&factory.CreateDevice(&dxgi)?)
    }
}
