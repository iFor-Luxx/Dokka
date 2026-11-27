use std::time::Instant;

use windows::Win32::Graphics::Dwm::DwmFlush;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, KillTimer, MSG, PM_REMOVE, PeekMessageW, SW_SHOWNOACTIVATE,
    SetTimer, ShowWindow, TranslateMessage, WA_INACTIVE, WM_QUIT, WM_TIMER,
};

use crate::menubar_loop::read_menubar;
use crate::read_pointer::{read_pointer, read_settings_pointer};
use crate::window_procedure::{
    DISMISS_MESSAGE, MENUBAR_FULLSCREEN_MESSAGE, SETTINGS_DISMISS_MESSAGE, SETTINGS_FOCUS_MESSAGE,
};
use crate::{Surface, SurfaceError, SurfaceEvent};

/// Espera para agrupar ráfagas de avisos en una sola actualización.
const SETTLE_MILLISECONDS: u32 = 200;
const TICK_MILLISECONDS: u32 = 1000;
/// Evita saltos grandes si un fotograma se retrasa.
const LONGEST_FRAME_SECONDS: f32 = 0.05;

#[derive(Default)]
struct LoopState {
    tick_timer: usize,
    pending_timer: Option<usize>,
    is_pressed: bool,
    is_launchpad_pressed: bool,
    is_menubar_pressed: bool,
    is_settings_pressed: bool,
}

impl Surface {
    /// Muestra la ventana y atiende eventos hasta que se cierre.
    ///
    /// `handle` devuelve `true` mientras quiera recibir fotogramas. En reposo el hilo duerme.
    /// `refresh_message` en la cola del hilo se entrega como `Refresh`, tras una breve espera.
    pub fn run(
        &mut self,
        refresh_message: u32,
        mut handle: impl FnMut(&mut Self, SurfaceEvent) -> bool,
    ) -> Result<(), SurfaceError> {
        let mut message = MSG::default();
        let mut state = LoopState {
            // SAFETY: temporizador del hilo, sin callback.
            tick_timer: unsafe { SetTimer(None, 0, TICK_MILLISECONDS, None) },
            ..Default::default()
        };
        let mut last_frame: Option<Instant> = None;
        // SAFETY: las ventanas pertenecen a este hilo.
        let _ = unsafe { ShowWindow(self.window, SW_SHOWNOACTIVATE) };
        if self.menubar_visible && !self.menubar_fullscreen {
            // SAFETY: la ventana es propia y de este hilo.
            let _ = unsafe { ShowWindow(self.menubar, SW_SHOWNOACTIVATE) };
        }
        let mut is_animating = false;

        loop {
            if is_animating {
                // SAFETY: `message` es escribible durante la llamada.
                while unsafe { PeekMessageW(&mut message, None, 0, 0, PM_REMOVE) }.as_bool() {
                    if message.message == WM_QUIT {
                        return Ok(());
                    }
                    if let Some(event) = self.process(&message, refresh_message, &mut state) {
                        // Lo que decide si se sigue animando es el fotograma de abajo.
                        handle(self, event);
                    }
                }
                let now = Instant::now();
                let seconds = last_frame
                    .map_or(0.0, |last| now.duration_since(last).as_secs_f32())
                    .min(LONGEST_FRAME_SECONDS);
                last_frame = Some(now);
                is_animating = handle(self, SurfaceEvent::Frame { seconds });
                // Espera al siguiente refresco de pantalla.
                // SAFETY: no recibe argumentos.
                if unsafe { DwmFlush() }.is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            } else {
                last_frame = None;
                // SAFETY: `message` es escribible durante la llamada.
                match unsafe { GetMessageW(&mut message, None, 0, 0) }.0 {
                    0 => return Ok(()),
                    -1 => return Err(windows::core::Error::from_thread().into()),
                    _ => {}
                }
                if let Some(event) = self.process(&message, refresh_message, &mut state) {
                    is_animating = handle(self, event);
                }
            }
        }
    }

    fn process(
        &mut self,
        message: &MSG,
        refresh_message: u32,
        state: &mut LoopState,
    ) -> Option<SurfaceEvent> {
        if message.hwnd.is_invalid() && message.message == refresh_message {
            // Un aviso urgente (`wParam` a 1) se entrega ya, sin agrupar.
            if message.wParam.0 == 1 {
                return Some(SurfaceEvent::Refresh);
            }
            // No se reinicia el temporizador: avisos continuos no deben aplazarlo siempre.
            if state.pending_timer.is_none() {
                // SAFETY: temporizador del hilo, sin callback.
                let timer = unsafe { SetTimer(None, 0, SETTLE_MILLISECONDS, None) };
                state.pending_timer = Some(timer);
            }
            return None;
        }
        if message.message == WM_TIMER && message.wParam.0 == state.tick_timer {
            self.refresh_menubar_fullscreen();
            return Some(SurfaceEvent::Tick);
        }
        if message.message == WM_TIMER && state.pending_timer == Some(message.wParam.0) {
            // SAFETY: cancela el temporizador creado arriba.
            let _ = unsafe { KillTimer(None, message.wParam.0) };
            state.pending_timer = None;
            return Some(SurfaceEvent::Refresh);
        }
        if message.message == WM_TIMER
            && let Some(event) = self.watch_event(message.wParam.0)
        {
            return Some(event);
        }
        if message.hwnd == self.window && message.message == DISMISS_MESSAGE {
            return Some(SurfaceEvent::Dismissed);
        }
        if message.hwnd == self.menubar_window() && message.message == DISMISS_MESSAGE {
            return Some(SurfaceEvent::MenubarDismissed);
        }
        if message.hwnd == self.menubar_window() && message.message == MENUBAR_FULLSCREEN_MESSAGE {
            self.refresh_menubar_fullscreen();
            return None;
        }
        if message.hwnd == self.launchpad_window() && message.message == DISMISS_MESSAGE {
            return Some(SurfaceEvent::LaunchpadDismissed);
        }
        if message.hwnd == self.settings_window() && message.message == SETTINGS_DISMISS_MESSAGE {
            return Some(SurfaceEvent::SettingsDismissed);
        }
        // El `wParam` original dice si sigue activa (`WA_INACTIVE` es 0).
        if message.hwnd == self.settings_window() && message.message == SETTINGS_FOCUS_MESSAGE {
            if message.wParam.0 & 0xFFFF == WA_INACTIVE as usize {
                return Some(SurfaceEvent::SettingsBlurred);
            }
            return Some(SurfaceEvent::SettingsFocused);
        }
        if message.hwnd == self.window
            && let Some(event) = read_pointer(message, self.scale, &mut state.is_pressed)
        {
            return Some(event);
        }
        if message.hwnd == self.menubar_window()
            && let Some(event) =
                read_menubar(message, self.menubar_scale(), &mut state.is_menubar_pressed)
        {
            return Some(event);
        }
        if message.hwnd == self.launchpad_window()
            && let Some(event) = read_pointer(message, self.scale, &mut state.is_launchpad_pressed)
        {
            return Some(launchpad_event(event));
        }
        if message.hwnd == self.settings_window()
            && let Some(event) = read_settings_pointer(
                message,
                self.settings_scale(),
                &mut state.is_settings_pressed,
            )
        {
            return Some(event);
        }
        // SAFETY: `message` acaba de ser rellenado por Windows.
        unsafe {
            let _ = TranslateMessage(message);
            DispatchMessageW(message);
        }
        None
    }
}

/// Traduce un evento del dock a su equivalente del Launchpad.
/// Esc llega como tecla propia para limpiar la búsqueda; la pérdida de foco
/// sigue llegando como `LaunchpadDismissed`. El botón derecho también cierra.
fn launchpad_event(event: SurfaceEvent) -> SurfaceEvent {
    match event {
        SurfaceEvent::PointerMoved { x, y } => SurfaceEvent::LaunchpadMoved { x, y },
        SurfaceEvent::PointerLeft => SurfaceEvent::LaunchpadLeft,
        SurfaceEvent::Clicked { x, y, .. } => SurfaceEvent::LaunchpadClicked { x, y },
        SurfaceEvent::RightClicked { x, y } => SurfaceEvent::LaunchpadClicked { x, y },
        SurfaceEvent::Dismissed => SurfaceEvent::LaunchpadEscape,
        other => other,
    }
}
