use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, FindWindowW, SW_HIDE, SW_SHOWNOACTIVATE, ShowWindow,
};
use windows::core::w;

/// Muestra u oculta la barra de tareas de Windows, incluidas las de otros monitores.
/// No cambia ningún ajuste persistente: solo muestra u oculta las ventanas
/// `Shell_TrayWnd` y `Shell_SecondaryTrayWnd`.
pub fn set_taskbar_visible(visible: bool) {
    let command = if visible { SW_SHOWNOACTIVATE } else { SW_HIDE };
    // SAFETY: buscar y mostrar u ocultar ventanas del shell con parámetros válidos.
    unsafe {
        if let Ok(primary) = FindWindowW(w!("Shell_TrayWnd"), None)
            && !primary.is_invalid()
        {
            let _ = ShowWindow(primary, command);
        }
        let mut after: Option<HWND> = None;
        while let Ok(found) = FindWindowExW(None, after, w!("Shell_SecondaryTrayWnd"), None) {
            if found.is_invalid() {
                break;
            }
            let _ = ShowWindow(found, command);
            after = Some(found);
        }
    }
}
