use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, GW_OWNER, GWL_EXSTYLE, GetWindow, GetWindowLongW, HTTRANSPARENT, MA_NOACTIVATE,
    PostMessageW, PostQuitMessage, SW_HIDE, ShowWindow, WA_INACTIVE, WM_ACTIVATE, WM_CLOSE,
    WM_DESTROY, WM_MOUSEACTIVATE, WM_NCHITTEST, WS_EX_NOACTIVATE, WS_EX_TRANSPARENT,
};

/// Aviso propio de que la ventana dejó de estar activa o se pulsó Esc.
/// `WM_ACTIVATE` llega directo al procedimiento y no pasa por la cola de `run_loop`,
/// así que se reenvía como mensaje publicado.
pub(crate) const DISMISS_MESSAGE: u32 = 0x8001;

/// Aviso propio de que Preferencias se cerró con la X, Alt+F4 o la barra de
/// tareas. `WM_CLOSE` tampoco pasa por la cola, así que se reenvía igual.
pub(crate) const SETTINGS_DISMISS_MESSAGE: u32 = 0x8002;

/// Aviso propio de que Preferencias ganó o perdió el foco. `WM_ACTIVATE`
/// llega directo al procedimiento y no pasa por la cola de `run_loop`, así
/// que se reenvía como mensaje publicado con el `wParam` original.
pub(crate) const SETTINGS_FOCUS_MESSAGE: u32 = 0x8003;

pub(crate) unsafe extern "system" fn window_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // SAFETY: consulta de solo lectura sobre la ventana que recibe el mensaje.
    let is_owned = || unsafe { GetWindow(window, GW_OWNER) }.is_ok_and(|w| !w.is_invalid());
    match message {
        // El dock no quita el foco a la app en uso; el Launchpad sí lo toma.
        WM_MOUSEACTIVATE if !is_owned() && has_no_activate(window) => {
            LRESULT(MA_NOACTIVATE as isize)
        }
        // La ventana superpuesta es solo visual: el mouse llega a su dueña.
        // La vista grande también deja pasar el mouse hacia la ventana real.
        WM_NCHITTEST if is_owned() || has_transparent(window) => LRESULT(HTTRANSPARENT as isize),
        // El foco ganado o perdido se reenvía a la cola para que lo lea
        // `run_loop`; el aviso de pérdida sigue llegando como `DISMISS_MESSAGE`.
        WM_ACTIVATE if !is_owned() => {
            // SAFETY: reenvía los avisos a la cola del hilo para que los lea `run_loop`.
            let _ = unsafe {
                PostMessageW(Some(window), SETTINGS_FOCUS_MESSAGE, wparam, LPARAM(0))
            };
            if wparam.0 & 0xFFFF == WA_INACTIVE as usize {
                // SAFETY: reenvía el aviso a la cola del hilo para que lo lea `run_loop`.
                let _ = unsafe {
                    PostMessageW(Some(window), DISMISS_MESSAGE, WPARAM(0), LPARAM(0))
                };
            }
            // SAFETY: Windows entrega argumentos válidos para el mensaje que se está atendiendo.
            unsafe { DefWindowProcW(window, message, wparam, lparam) }
        }
        WM_DESTROY if !is_owned() => {
            // SAFETY: termina el bucle de mensajes de este hilo.
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        // Cerrar con la X, Alt+F4 o la barra de tareas solo oculta: destruir una
        // ventana sin dueña terminaría la app. Vale para las tres ventanas.
        // Preferencias además avisa para que el shell cierre la isla.
        WM_CLOSE if !is_owned() => {
            // SAFETY: la ventana que recibe el mensaje sigue viva aquí.
            unsafe {
                let _ = ShowWindow(window, SW_HIDE);
                let _ = PostMessageW(Some(window), SETTINGS_DISMISS_MESSAGE, WPARAM(0), LPARAM(0));
            }
            LRESULT(0)
        }
        // SAFETY: Windows entrega argumentos válidos para el mensaje que se está atendiendo.
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}

/// `true` si la ventana se creó para no activarse, como el dock.
fn has_no_activate(window: HWND) -> bool {
    // SAFETY: consulta de solo lectura del estilo extendido propio.
    let style = unsafe { GetWindowLongW(window, GWL_EXSTYLE) } as u32;
    style & WS_EX_NOACTIVATE.0 != 0
}

/// `true` si la ventana deja pasar el mouse, como la vista grande.
fn has_transparent(window: HWND) -> bool {
    // SAFETY: consulta de solo lectura del estilo extendido propio.
    let style = unsafe { GetWindowLongW(window, GWL_EXSTYLE) } as u32;
    style & WS_EX_TRANSPARENT.0 != 0
}
