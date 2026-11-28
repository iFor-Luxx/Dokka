//! Bucle de mensajes de la ventana del instalador.
//!
//! Como `run_loop` pero para una sola ventana: traduce el ratón con captura
//! (como Preferencias, para arrastrar desde el fondo), la X como `Dismissed`
//! y agrupa los avisos de actualización en un `Refresh`.

use std::time::Instant;

use windows::Win32::Graphics::Dwm::DwmFlush;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, KillTimer, MSG, PM_REMOVE, PeekMessageW, SW_SHOWNORMAL,
    SetTimer, ShowWindow, TranslateMessage, WM_QUIT, WM_TIMER,
};

use crate::read_pointer::read_settings_pointer;
use crate::window_procedure::SETTINGS_DISMISS_MESSAGE;
use crate::{InstallerEvent, SurfaceError};

use super::installer_window::InstallerWindow;

/// Espera para agrupar avisos en una sola actualización.
const SETTLE_MILLISECONDS: u32 = 200;
const TICK_MILLISECONDS: u32 = 1000;
/// Evita saltos grandes si un fotograma se retrasa.
const LONGEST_FRAME_SECONDS: f32 = 0.05;

impl InstallerWindow {
    /// Muestra la ventana y atiende eventos hasta que se cierre.
    ///
    /// `handle` devuelve `true` mientras quiera recibir fotogramas.
    /// `progress_message` en la cola del hilo se entrega como `Refresh`.
    pub fn run(
        &mut self,
        progress_message: u32,
        mut handle: impl FnMut(&mut Self, InstallerEvent) -> bool,
    ) -> Result<(), SurfaceError> {
        let mut message = MSG::default();
        // SAFETY: temporizador del hilo, sin callback.
        let tick = unsafe { SetTimer(None, 0, TICK_MILLISECONDS, None) };
        let mut pending: Option<usize> = None;
        let mut is_pressed = false;
        let mut last_frame: Option<Instant> = None;
        // SAFETY: la ventana pertenece a este hilo y se activa al mostrarse.
        let _ = unsafe { ShowWindow(self.window(), SW_SHOWNORMAL) };
        let mut is_animating = true;

        loop {
            if is_animating {
                // SAFETY: `message` es escribible durante la llamada.
                while unsafe { PeekMessageW(&mut message, None, 0, 0, PM_REMOVE) }.as_bool() {
                    if message.message == WM_QUIT {
                        return Ok(());
                    }
                    if let Some(event) = self.process(
                        &message,
                        progress_message,
                        tick,
                        &mut pending,
                        &mut is_pressed,
                    ) {
                        handle(self, event);
                    }
                }
                let now = Instant::now();
                let seconds = last_frame
                    .map_or(0.0, |last| now.duration_since(last).as_secs_f32())
                    .min(LONGEST_FRAME_SECONDS);
                last_frame = Some(now);
                is_animating = handle(self, InstallerEvent::Frame { seconds });
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
                if let Some(event) = self.process(
                    &message,
                    progress_message,
                    tick,
                    &mut pending,
                    &mut is_pressed,
                ) {
                    is_animating = handle(self, event);
                }
            }
        }
    }

    /// Pide terminar `run`.
    pub fn close(&self) {
        // SAFETY: encola el fin del bucle de este hilo.
        unsafe { windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0) };
    }

    fn process(
        &mut self,
        message: &MSG,
        progress_message: u32,
        tick: usize,
        pending: &mut Option<usize>,
        is_pressed: &mut bool,
    ) -> Option<InstallerEvent> {
        if message.message == progress_message {
            if pending.is_none() {
                // SAFETY: temporizador del hilo, sin callback.
                let timer = unsafe { SetTimer(None, 0, SETTLE_MILLISECONDS, None) };
                *pending = Some(timer);
            }
            return None;
        }
        if message.message == WM_TIMER && message.wParam.0 == tick {
            return Some(InstallerEvent::Tick);
        }
        if message.message == WM_TIMER && *pending == Some(message.wParam.0) {
            // SAFETY: cancela el temporizador creado arriba.
            let _ = unsafe { KillTimer(None, message.wParam.0) };
            *pending = None;
            return Some(InstallerEvent::Refresh);
        }
        if message.hwnd == self.window() && message.message == SETTINGS_DISMISS_MESSAGE {
            return Some(InstallerEvent::Dismissed);
        }
        if message.hwnd == self.window()
            && let Some(event) = read_settings_pointer(message, self.scale(), is_pressed)
        {
            return map_pointer(event, self.margin());
        }
        // SAFETY: `message` acaba de ser rellenado por Windows.
        unsafe {
            let _ = TranslateMessage(message);
            DispatchMessageW(message);
        }
        None
    }
}

/// Traduce el puntero de Preferencias al del instalador, con el margen
/// quitado: la isla trabaja en coordenadas de contenido.
fn map_pointer(event: crate::SurfaceEvent, margin: f32) -> Option<InstallerEvent> {
    match event {
        crate::SurfaceEvent::SettingsMoved { x, y } => Some(InstallerEvent::PointerMoved {
            x: x - margin,
            y: y - margin,
        }),
        crate::SurfaceEvent::SettingsLeft => Some(InstallerEvent::PointerLeft),
        crate::SurfaceEvent::SettingsPressed { x, y } => Some(InstallerEvent::Pressed {
            x: x - margin,
            y: y - margin,
        }),
        crate::SurfaceEvent::SettingsClicked { x, y } => Some(InstallerEvent::Clicked {
            x: x - margin,
            y: y - margin,
        }),
        _ => None,
    }
}
