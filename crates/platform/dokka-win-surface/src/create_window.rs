use windows::Win32::Foundation::{ERROR_CLASS_ALREADY_EXISTS, GetLastError, HWND};
use windows::Win32::Graphics::Dwm::{DWMWA_USE_HOSTBACKDROPBRUSH, DwmSetWindowAttribute};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, IDC_ARROW, LoadCursorW, RegisterClassExW, WINDOW_STYLE, WNDCLASSEXW,
    WS_EX_APPWINDOW, WS_EX_NOACTIVATE, WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
    WS_EX_TRANSPARENT, WS_MINIMIZEBOX, WS_POPUP, WS_SYSMENU,
};
use windows::core::{BOOL, Error, PCWSTR, Result, w};

use crate::window_procedure::window_procedure;

/// Crea una ventana oculta y sin tamaño. Con `owner`, queda siempre encima de ella
/// y deja pasar el mouse hacia ella.
pub(crate) fn create_window(owner: Option<HWND>) -> Result<HWND> {
    create_with_style(
        owner,
        WS_EX_NOREDIRECTIONBITMAP | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
        WS_POPUP,
        w!("Dokka"),
    )
}

/// Ventana del Launchpad: a pantalla completa, sí toma el foco para Esc.
pub(crate) fn create_launchpad_window() -> Result<HWND> {
    create_with_style(
        None,
        WS_EX_NOREDIRECTIONBITMAP | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
        WS_POPUP,
        w!("Dokka"),
    )
}

/// Ventana de Preferencias: normal y activable (sale en la barra de tareas y
/// toma el foco), sin estar siempre encima. El contenido lo dibuja el mismo
/// compositor en su propio árbol visual.
pub(crate) fn create_settings_window() -> Result<HWND> {
    create_with_style(
        None,
        WS_EX_NOREDIRECTIONBITMAP | WS_EX_APPWINDOW,
        // Sin borde ni barra de título: el dibujo propio hace de marco.
        // SYSMENU + MINIMIZEBOX dan menú de sistema, minimizar y animación
        // de la barra de tareas como una ventana normal.
        WS_POPUP | WS_SYSMENU | WS_MINIMIZEBOX,
        w!("Preferencias"),
    )
}

/// Ventana grande: a pantalla completa, sin foco y transparente al mouse.
pub(crate) fn create_preview_window() -> Result<HWND> {
    create_with_style(
        None,
        WS_EX_NOREDIRECTIONBITMAP
            | WS_EX_TOPMOST
            | WS_EX_TOOLWINDOW
            | WS_EX_NOACTIVATE
            | WS_EX_TRANSPARENT,
        WS_POPUP,
        w!("Dokka"),
    )
}

/// Barra superior: como el dock (sin activación, siempre encima, cristal),
/// en archivos propios. Sin dueña para que `WM_ACTIVATE` avise al perder el foco.
pub(crate) fn create_menubar_window() -> Result<HWND> {
    create_with_style(
        None,
        WS_EX_NOREDIRECTIONBITMAP | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
        WS_POPUP,
        w!("Dokka"),
    )
}

fn create_with_style(
    owner: Option<HWND>,
    extended: windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE,
    style: WINDOW_STYLE,
    title: PCWSTR,
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
            title,
            style,
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
