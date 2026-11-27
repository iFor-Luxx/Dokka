//! Hilo propio con la ventana oculta que recibe la bandeja.
//!
//! La ventana se llama `Shell_TrayWnd` y se mantiene por delante de la
//! barra real para que Windows le envíe primero los iconos. Al crearla se
//! avisa con `TaskbarCreated` para que las apps vuelvan a registrarse.
//! Todo lo dirigido a la barra se reenvía a la real; registrar en Dokka
//! nunca impide ese reenvío.

use std::ffi::c_void;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

use windows::Win32::Foundation::{
    ERROR_CLASS_ALREADY_EXISTS, GetLastError, HWND, LPARAM, LRESULT, WPARAM,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CREATESTRUCTW, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GWLP_USERDATA,
    GetMessageW, GetWindowLongPtrW, HWND_BROADCAST, HWND_TOPMOST, IsWindow, KillTimer, MSG,
    PostQuitMessage, RegisterClassW, RegisterWindowMessageW, SWP_NOACTIVATE, SWP_NOMOVE,
    SWP_NOSIZE, SendNotifyMessageW, SetTimer, SetWindowLongPtrW, SetWindowPos, TranslateMessage,
    WM_ACTIVATEAPP, WM_CLOSE, WM_COMMAND, WM_COPYDATA, WM_DESTROY, WM_NCCREATE, WM_TIMER, WM_USER,
    WNDCLASSW, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};
use windows::core::w;

use crate::forward;
use crate::intake;
use crate::state::Shared;

/// Temporizador que mantiene la ventana delante y poda iconos muertos.
const TIMER_ID: usize = 1;
/// Cada cuánto se reordena y se revisan las ventanas, en milisegundos.
const TIMER_MS: u32 = 200;

/// Arranca el hilo y devuelve su espera junto con la ventana creada.
pub(crate) fn spawn(shared: Arc<Mutex<Shared>>) -> Result<(JoinHandle<()>, HWND), String> {
    let (done, waiting) = mpsc::channel::<Result<usize, String>>();
    let thread = thread::spawn(move || run(shared, done));
    match waiting.recv() {
        // `HWND` no cruza hilos: viaja como número y se rehace aquí.
        Ok(Ok(raw)) => Ok((thread, HWND(raw as *mut c_void))),
        Ok(Err(reason)) => {
            let _ = thread.join();
            Err(reason)
        }
        Err(_) => {
            let _ = thread.join();
            Err("la bandeja no pudo arrancar".to_owned())
        }
    }
}

/// Cuerpo del hilo: crea la ventana, avisa y atiende mensajes.
fn run(shared: Arc<Mutex<Shared>>, done: mpsc::Sender<Result<usize, String>>) {
    let holder = Box::new(shared);
    let raw = Box::into_raw(holder);
    let window = create_window(raw as *const c_void);
    let Ok(window) = window else {
        // SAFETY: se recupera lo apartado para el procedimiento.
        let _ = unsafe { Box::from_raw(raw) };
        let _ = done.send(Err("Windows no dejó crear la bandeja".to_owned()));
        return;
    };
    // SAFETY: ventana propia del hilo; el aviso es lo mejor posible.
    unsafe {
        SetTimer(Some(window), TIMER_ID, TIMER_MS, None);
        let created = RegisterWindowMessageW(w!("TaskbarCreated"));
        let _ = SendNotifyMessageW(HWND_BROADCAST, created, WPARAM(0), LPARAM(0));
    }
    if done.send(Ok(window.0 as usize)).is_err() {
        // SAFETY: nadie espera; se desmonta la ventana en su propio hilo.
        unsafe {
            let _ = KillTimer(Some(window), TIMER_ID);
            let _ = DestroyWindow(window);
            let _ = Box::from_raw(raw);
        }
        return;
    }
    // SAFETY: `message` es escribible durante cada llamada.
    unsafe {
        let mut message = MSG::default();
        loop {
            let status = GetMessageW(&mut message, None, 0, 0);
            if status.0 == 0 || status.0 == -1 {
                break;
            }
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        let _ = Box::from_raw(raw);
    }
}

/// Ventana oculta con el nombre de la barra; sin mostrarse nunca.
fn create_window(param: *const c_void) -> Result<HWND, String> {
    // SAFETY: las cadenas son estáticas y el procedimiento cumple la firma.
    unsafe {
        let module = GetModuleHandleW(None).map_err(|_| "sin instancia".to_owned())?;
        let class = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: module.into(),
            lpszClassName: w!("Shell_TrayWnd"),
            ..Default::default()
        };
        if RegisterClassW(&class) == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS {
            return Err("clase sin registrar".to_owned());
        }
        CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            w!("Shell_TrayWnd"),
            w!(""),
            WS_POPUP,
            0,
            0,
            0,
            0,
            None,
            None,
            Some(module.into()),
            Some(param),
        )
        .map_err(|_| "ventana sin crear".to_owned())
    }
}

/// Procedimiento de la ventana; nunca entra en pánico: todo es `Option`.
pub(crate) unsafe extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_NCCREATE {
        // SAFETY: Windows entrega el `CREATESTRUCT` al crear la ventana.
        unsafe {
            if let Some(data) = (lparam.0 as *const CREATESTRUCTW).as_ref() {
                let _ = SetWindowLongPtrW(window, GWLP_USERDATA, data.lpCreateParams as isize);
            }
        }
        return LRESULT(1);
    }
    // SAFETY: lectura del puntero guardado al crear la ventana.
    let shared = unsafe {
        let slot = GetWindowLongPtrW(window, GWLP_USERDATA);
        if slot == 0 {
            None
        } else {
            (slot as *const Arc<Mutex<Shared>>).as_ref()
        }
    };
    let Some(shared) = shared else {
        // SAFETY: sin estado propio: comportamiento por defecto.
        return unsafe { DefWindowProcW(window, message, wparam, lparam) };
    };
    match message {
        WM_CLOSE => {
            // SAFETY: la ventana es del hilo; al destruirla sale del bucle.
            unsafe {
                let _ = DestroyWindow(window);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            // SAFETY: temporizador y bucle del propio hilo.
            unsafe {
                let _ = KillTimer(Some(window), TIMER_ID);
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        WM_TIMER if wparam.0 == TIMER_ID => {
            on_timer(window, shared);
            LRESULT(0)
        }
        WM_COPYDATA => {
            intake::note_shell_icon(shared, lparam);
            if let Some(done) = forward::forward(message, wparam, lparam) {
                done
            } else if intake::is_shell_notify(lparam) {
                LRESULT(1)
            } else {
                // SAFETY: sin barra real: acuse si era icono, defecto si no.
                unsafe { DefWindowProcW(window, message, wparam, lparam) }
            }
        }
        WM_ACTIVATEAPP | WM_COMMAND => {
            if let Some(done) = forward::forward(message, wparam, lparam) {
                done
            } else {
                // SAFETY: sin barra real: comportamiento por defecto.
                unsafe { DefWindowProcW(window, message, wparam, lparam) }
            }
        }
        other if other >= WM_USER => {
            // SAFETY: la cadena es estática; el id es el mismo en el proceso.
            let created = unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) };
            if other == created {
                // Nuestro propio aviso: las apps ya se están registrando.
                LRESULT(0)
            } else if let Some(done) = forward::forward(message, wparam, lparam) {
                done
            } else {
                // SAFETY: sin barra real: comportamiento por defecto.
                unsafe { DefWindowProcW(window, message, wparam, lparam) }
            }
        }
        // SAFETY: Windows entrega argumentos válidos para cada mensaje.
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}

/// Mantiene la ventana delante sin activar y poda los iconos muertos.
fn on_timer(window: HWND, shared: &Arc<Mutex<Shared>>) {
    // SAFETY: ventana propia; no se mueve ni se activa.
    unsafe {
        let _ = SetWindowPos(
            window,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );
    }
    let watched = shared.lock().ok().map(|state| state.snapshot_hwnds());
    let Some(watched) = watched else {
        return;
    };
    let mut dead = Vec::new();
    for (key, hwnd_raw) in watched {
        let probe = HWND(hwnd_raw as usize as *mut c_void);
        // SAFETY: `IsWindow` admite cualquier valor.
        let alive = unsafe { IsWindow(Some(probe)).as_bool() };
        if !alive {
            dead.push(key);
        }
    }
    if dead.is_empty() {
        return;
    }
    if let Ok(mut state) = shared.lock() {
        state.remove_dead(&dead);
    }
}
