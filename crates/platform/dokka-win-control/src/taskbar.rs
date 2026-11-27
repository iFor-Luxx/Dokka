use windows::Win32::Foundation::HWND;
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, GetWindowThreadProcessId, SHOW_WINDOW_CMD, SW_HIDE, SW_SHOWNOACTIVATE,
    ShowWindow,
};
use windows::core::{PCWSTR, w};

/// Muestra u oculta la barra de tareas de Windows, incluidas las de otros monitores.
/// No cambia ningún ajuste persistente: solo muestra u oculta las ventanas
/// `Shell_TrayWnd` y `Shell_SecondaryTrayWnd`.
pub fn set_taskbar_visible(visible: bool) {
    let command = if visible { SW_SHOWNOACTIVATE } else { SW_HIDE };
    set_matching_visible(w!("Shell_TrayWnd"), command);
    // SAFETY: buscar y mostrar u ocultar ventanas del shell con parámetros válidos.
    unsafe {
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

/// Muestra u oculta las ventanas de una clase salvo la de este proceso.
/// Dokka puede tener su propia `Shell_TrayWnd` por delante de la real para
/// recibir los iconos; esa no se toca: solo la barra de Windows.
fn set_matching_visible(class: PCWSTR, command: SHOW_WINDOW_CMD) {
    // SAFETY: buscar ventanas por clase y leer su proceso no cambia nada;
    // mostrar u ocultar una ventana ajena con un comando válido es seguro.
    unsafe {
        let own = GetCurrentProcessId();
        let mut after: Option<HWND> = None;
        while let Ok(found) = FindWindowExW(None, after, class, None) {
            if found.is_invalid() {
                break;
            }
            after = Some(found);
            let mut owner = 0;
            GetWindowThreadProcessId(found, Some(&mut owner));
            if owner == own {
                continue;
            }
            let _ = ShowWindow(found, command);
        }
    }
}
