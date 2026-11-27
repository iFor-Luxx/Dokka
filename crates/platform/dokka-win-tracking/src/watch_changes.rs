use std::sync::atomic::{AtomicBool, Ordering};

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Accessibility::{HWINEVENTHOOK, SetWinEventHook, UnhookWinEvent};
use windows::Win32::UI::WindowsAndMessaging::{
    EVENT_OBJECT_CLOAKED, EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE, EVENT_OBJECT_LOCATIONCHANGE,
    EVENT_OBJECT_UNCLOAKED, EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_MINIMIZEEND,
    EVENT_SYSTEM_MINIMIZESTART, GetForegroundWindow, OBJID_WINDOW, PostThreadMessageW,
    WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WM_APP,
};

use crate::occlusion::foreground_occludes_primary;

/// Mensaje que recibe el hilo observador cuando alguna ventana cambia.
pub const WINDOWS_CHANGED_MESSAGE: u32 = WM_APP + 1;

/// Último estado ocupado conocido para filtrar el movimiento.
static LAST_OCCLUDED: AtomicBool = AtomicBool::new(false);

/// Mantiene activos los avisos de Windows; al soltarlo se cancelan.
/// El hilo que lo crea debe atender su cola de mensajes.
#[derive(Debug)]
pub struct ChangeWatcher {
    hooks: Vec<HWINEVENTHOOK>,
}

impl ChangeWatcher {
    pub fn start() -> Self {
        // Destruir, mostrar y ocultar son consecutivos; ocultar por Windows va aparte.
        // El primer plano no llegaba como aviso: sin él la barra no sabe qué app mostrar.
        // El movimiento solo avisa si cambia lo ocupado: maximizar o restaurar
        // no manda otro aviso y el movimiento inundaría sin este filtro.
        LAST_OCCLUDED.store(foreground_occludes_primary(), Ordering::Relaxed);
        let ranges = [
            (EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE),
            (EVENT_OBJECT_CLOAKED, EVENT_OBJECT_UNCLOAKED),
            (EVENT_SYSTEM_MINIMIZESTART, EVENT_SYSTEM_MINIMIZEEND),
            (EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND),
            (EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_LOCATIONCHANGE),
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
    event: u32,
    window: HWND,
    object: i32,
    _child: i32,
    _thread: u32,
    _time: u32,
) {
    // Los eventos de controles internos no interesan, solo los de ventanas.
    if object != OBJID_WINDOW.0 {
        return;
    }
    if event == EVENT_OBJECT_LOCATIONCHANGE {
        // Solo la ventana al frente puede ocupar la pantalla; el resto se ignora.
        // SAFETY: consulta sin argumentos; solo se compara el handle.
        let foreground = unsafe { GetForegroundWindow() };
        if window != foreground {
            return;
        }
        let occluded = foreground_occludes_primary();
        if occluded == LAST_OCCLUDED.swap(occluded, Ordering::Relaxed) {
            return;
        }
    } else {
        LAST_OCCLUDED.store(foreground_occludes_primary(), Ordering::Relaxed);
    }
    // Al empezar a minimizar no se puede esperar a agrupar avisos: `wParam` a 1.
    // Vía de Muelle (MIT): el hwnd viaja en `lParam` para actuar de inmediato,
    // sin deducirlo comparando listas después.
    let urgent = usize::from(event == EVENT_SYSTEM_MINIMIZESTART);
    // SAFETY: publica un mensaje sin datos propios en la cola del hilo actual.
    let _ = unsafe {
        PostThreadMessageW(
            GetCurrentThreadId(),
            WINDOWS_CHANGED_MESSAGE,
            WPARAM(urgent),
            LPARAM(window.0 as isize),
        )
    };
}
