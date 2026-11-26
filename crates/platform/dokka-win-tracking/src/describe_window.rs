use dokka_model::{AppId, AppInfo, AppWindow, OpenWindow, WindowId};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextW, IsIconic};

use crate::find_executable::find_executable;

/// Convierte una ventana de Windows a los tipos de Dokka.
/// Devuelve `None` si no se puede saber a qué app pertenece.
pub(crate) fn describe_window(window: HWND) -> Option<OpenWindow> {
    let executable = find_executable(window)?;
    let name = executable.file_stem()?.to_string_lossy().into_owned();

    let mut title = [0_u16; 512];
    // SAFETY: Windows no escribe más allá del tamaño del búfer entregado.
    let (length, is_minimized) = unsafe {
        (
            GetWindowTextW(window, &mut title),
            IsIconic(window).as_bool(),
        )
    };

    Some(OpenWindow {
        app: AppInfo {
            // Las rutas de Windows no distinguen mayúsculas.
            id: AppId::new(executable.to_string_lossy().to_lowercase()),
            name,
            executable,
        },
        window: AppWindow {
            id: WindowId::new(window.0 as u64),
            title: String::from_utf16_lossy(&title[..length.max(0) as usize]),
            is_minimized,
        },
    })
}
