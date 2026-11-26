use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, GW_OWNER, GetWindow, HTTRANSPARENT, MA_NOACTIVATE, PostQuitMessage, WM_DESTROY,
    WM_MOUSEACTIVATE, WM_NCHITTEST,
};

pub(crate) unsafe extern "system" fn window_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // SAFETY: consulta de solo lectura sobre la ventana que recibe el mensaje.
    let is_owned = || unsafe { GetWindow(window, GW_OWNER) }.is_ok_and(|w| !w.is_invalid());
    match message {
        // Un clic en el dock no debe quitar el foco a la app que se está usando.
        WM_MOUSEACTIVATE => LRESULT(MA_NOACTIVATE as isize),
        // La ventana superpuesta es solo visual: el mouse llega a su dueña.
        WM_NCHITTEST if is_owned() => LRESULT(HTTRANSPARENT as isize),
        WM_DESTROY if !is_owned() => {
            // SAFETY: termina el bucle de mensajes de este hilo.
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        // SAFETY: Windows entrega argumentos válidos para el mensaje que se está atendiendo.
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}
