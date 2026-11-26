use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, MA_NOACTIVATE, PostQuitMessage, WM_DESTROY, WM_MOUSEACTIVATE,
};

pub(crate) unsafe extern "system" fn window_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        // Un clic en el dock no debe quitar el foco a la app que se está usando.
        WM_MOUSEACTIVATE => LRESULT(MA_NOACTIVATE as isize),
        WM_DESTROY => {
            // SAFETY: termina el bucle de mensajes de este hilo.
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        // SAFETY: Windows entrega argumentos válidos para el mensaje que se está atendiendo.
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}
