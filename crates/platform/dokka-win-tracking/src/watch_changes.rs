use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Accessibility::{HWINEVENTHOOK, SetWinEventHook, UnhookWinEvent};
use windows::Win32::UI::WindowsAndMessaging::{
    EVENT_OBJECT_CLOAKED, EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE, EVENT_OBJECT_UNCLOAKED,
    OBJID_WINDOW, PostThreadMessageW, WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WM_APP,
};

/// Mensaje que recibe el hilo observador cuando alguna ventana cambia.
pub const WINDOWS_CHANGED_MESSAGE: u32 = WM_APP + 1;

/// Mantiene activos los avisos de Windows; al soltarlo se cancelan.
/// El hilo que lo crea debe atender su cola de mensajes.
#[derive(Debug)]
pub struct ChangeWatcher {
    hooks: Vec<HWINEVENTHOOK>,
}

impl ChangeWatcher {
    pub fn start() -> Self {
        // Destruir, mostrar y ocultar son consecutivos; ocultar por Windows va aparte.
        let ranges = [
            (EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE),
            (EVENT_OBJECT_CLOAKED, EVENT_OBJECT_UNCLOAKED),
        ];
        let hooks = ranges
            .into_iter()
            .map(|(first, last)| {
                // SAFETY: fuera de contexto, el callback se ejecuta en este mismo hilo.
                unsafe {
                    SetWinEventHook(
                        first,
                        last,
                        None,
                        Some(notify),
                        0,
                        0,
                        WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
                    )
                }
            })
            .filter(|hook| !hook.is_invalid())
            .collect();
        Self { hooks }
    }
}

impl Drop for ChangeWatcher {
    fn drop(&mut self) {
        for &hook in &self.hooks {
            // SAFETY: cada hook se creó en `start` y se retira una sola vez.
            let _ = unsafe { UnhookWinEvent(hook) };
        }
    }
}

unsafe extern "system" fn notify(
    _hook: HWINEVENTHOOK,
    _event: u32,
    _window: HWND,
    object: i32,
    _child: i32,
    _thread: u32,
    _time: u32,
) {
    // Los eventos de controles internos no interesan, solo los de ventanas.
    if object == OBJID_WINDOW.0 {
        // SAFETY: publica un mensaje sin datos en la cola del hilo actual.
        let _ = unsafe {
            PostThreadMessageW(
                GetCurrentThreadId(),
                WINDOWS_CHANGED_MESSAGE,
                WPARAM(0),
                LPARAM(0),
            )
        };
    }
}
