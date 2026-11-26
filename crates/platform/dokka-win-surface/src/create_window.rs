use windows::Win32::Foundation::{ERROR_CLASS_ALREADY_EXISTS, GetLastError, HWND};
use windows::Win32::Graphics::Dwm::{DWMWA_USE_HOSTBACKDROPBRUSH, DwmSetWindowAttribute};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, IDC_ARROW, LoadCursorW, RegisterClassExW, WNDCLASSEXW, WS_EX_NOACTIVATE,
    WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};
use windows::core::{BOOL, Error, Result, w};

use crate::window_procedure::window_procedure;

/// Crea una ventana oculta y sin tamaño. Con `owner`, queda siempre encima de ella
/// y deja pasar el mouse hacia ella.
pub(crate) fn create_window(owner: Option<HWND>) -> Result<HWND> {
    create_with_style(
        owner,
        WS_EX_NOREDIRECTIONBITMAP | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
    )
}

/// Ventana del Launchpad: a pantalla completa, sí toma el foco para Esc.
pub(crate) fn create_launchpad_window() -> Result<HWND> {
    create_with_style(
        None,
        WS_EX_NOREDIRECTIONBITMAP | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
    )
}

/// Vista grande: a pantalla completa, sin foco y transparente al mouse.
pub(crate) fn create_preview_window() -> Result<HWND> {
    create_with_style(
        None,
        WS_EX_NOREDIRECTIONBITMAP
            | WS_EX_TOPMOST
            | WS_EX_TOOLWINDOW
            | WS_EX_NOACTIVATE
            | WS_EX_TRANSPARENT,
    )
}

fn create_with_style(
    owner: Option<HWND>,
    extended: windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE,
) -> Result<HWND> {
    let class_name = w!("DokkaSurface");
    // SAFETY: las cadenas son estáticas, las estructuras viven durante cada llamada
    // y `window_procedure` cumple la firma que exige Windows.
    unsafe {
        // Falla si ya estaba declarado; en ese caso sirve el valor vigente.
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

        let instance = GetModuleHandleW(None)?;
        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(window_procedure),
            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: class_name,
            ..Default::default()
        };
        // La segunda ventana reutiliza la clase ya registrada.
        if RegisterClassExW(&class) == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS {
            return Err(Error::from_thread());
        }

        let window = CreateWindowExW(
            extended,
            class_name,
            w!("Dokka"),
            WS_POPUP,
            0,
            0,
            0,
            0,
            owner,
            None,
            Some(instance.into()),
            None,
        )?;

        // Permite que la composición vea lo que hay detrás para el efecto de cristal.
        let enabled = BOOL::from(true);
        DwmSetWindowAttribute(
            window,
            DWMWA_USE_HOSTBACKDROPBRUSH,
            (&raw const enabled).cast(),
            size_of::<BOOL>() as u32,
        )?;
        Ok(window)
    }
}
