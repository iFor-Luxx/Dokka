// Adaptado de https://github.com/crafter-station/Muelle (commit 37e843d).
// MIT License
//
// Copyright (c) 2026 Muelle contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Puente del efecto genio: detecta minimizar y restaurar y pide los vuelos.
//!
//! Usa lo que ya hay (`ChangeWatcher` avisa y `Refresh` sincroniza): compara
//! minimizadas antes y despues para saber que empezo o termino por fuera. El
//! destino lo da el dock en sus coordenadas y aqui pasa a pixeles de pantalla.
//! Si algo falla, la ventana sigue su curso normal.
//!
//! Recibe las piezas por separado para no pedir prestado el `DockSession`
//! entero mientras se pide prestado su campo `genie`.

use std::time::Instant;

use dokka_dock::Dock;
use dokka_model::{DockApp, DockPosition, WindowId};
use dokka_win_control::{foreground_window, window_rect};
use dokka_win_genie::{GenieDirector, dock_slot_to_screen, monitor_rect_for_window};
use dokka_win_surface::Surface;

/// Estado del genio entre eventos. Solo el shell lo posee.
pub(crate) struct GenieBridge {
    director: GenieDirector,
    last_minimized: Vec<isize>,
    started: bool,
    settings_window: Option<u64>,
}

impl GenieBridge {
    pub(crate) fn new() -> Self {
        Self {
            director: GenieDirector::new(),
            last_minimized: Vec::new(),
            started: false,
            settings_window: None,
        }
    }

    /// Enciende o apaga el efecto con la preferencia del dock.
    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.director.set_enabled(enabled);
    }

    /// Recuerda la ventana del dock para pasar el vuelo tras su cristal.
    pub(crate) fn set_dock_window(&mut self, raw: isize) {
        self.director.set_dock_window(raw);
    }

    /// Recuerda la ventana propia de Preferencias para no animarla.
    pub(crate) fn set_settings_window(&mut self, raw: u64) {
        self.settings_window = Some(raw);
    }

    /// `true` mientras hay un vuelo dibujandose.
    pub(crate) fn is_flying(&self) -> bool {
        self.director.is_flying()
    }

    fn is_settings(&self, raw: u64) -> bool {
        self.settings_window.is_some_and(|known| known == raw)
    }

    /// Calma las ventanas abiertas para sustituir su animacion propia.
    fn calm_open(&mut self, apps: &[DockApp]) {
        let mut windows: Vec<isize> = Vec::new();
        for app in apps {
            for window in &app.windows {
                if self.is_settings(window.id.value()) {
                    continue;
                }
                windows.push(window.id.value() as isize);
            }
        }
        self.director.calm_windows(&windows);
    }

    /// Minimizada ahora segun las apps ya sincronizadas.
    fn current_minimized(apps: &[DockApp]) -> Vec<isize> {
        let mut minimized: Vec<isize> = Vec::new();
        for app in apps {
            for window in &app.windows {
                if window.is_minimized {
                    minimized.push(window.id.value() as isize);
                }
            }
        }
        minimized
    }

    /// Tras sincronizar: detecta nuevas minimizadas y restaura por fuera.
    pub(crate) fn on_refresh(
        &mut self,
        dock: &Dock,
        apps: &[DockApp],
        position: DockPosition,
        surface: &Surface,
    ) {
        let minimized = Self::current_minimized(apps);
        if !self.started {
            self.last_minimized = minimized;
            self.started = true;
            self.director.mark_started();
            self.calm_open(apps);
            return;
        }
        self.calm_open(apps);
        let foreground = foreground_window().map_or(0, |id| id.value() as isize);
        self.director.cancel_if_stale(foreground, &minimized);
        for &hwnd in &minimized {
            if self.last_minimized.contains(&hwnd) {
                continue;
            }
            self.begin_minimize(dock, apps, position, surface, hwnd, foreground);
        }
        self.last_minimized = minimized;
    }

    /// Empieza el vuelo de una ventana recien minimizada.
    fn begin_minimize(
        &mut self,
        dock: &Dock,
        apps: &[DockApp],
        position: DockPosition,
        surface: &Surface,
        hwnd: isize,
        foreground: isize,
    ) {
        let id = WindowId::new(hwnd as u64);
        if self.is_settings(hwnd as u64) {
            return;
        }
        let Some(source) = window_rect(id) else {
            return;
        };
        let screen = monitor_rect_for_window(hwnd);
        if !self
            .director
            .begin_minimize(hwnd, source, screen, position, foreground)
        {
            return;
        }
        let Some(slot) = dock.genie_target(apps, id) else {
            return;
        };
        let dock_rect = window_rect(WindowId::new(surface.handle().raw().0 as u64));
        let Some(dock_rect) = dock_rect else {
            return;
        };
        let slot_screen = dock_slot_to_screen(
            dock_rect,
            [slot.x, slot.y, slot.width, slot.height],
            surface.scale(),
        );
        self.director.aim_current(slot_screen);
    }

    /// Clic en la miniatura: vuela al reves desde su foto.
    pub(crate) fn on_restore_click(
        &mut self,
        dock: &Dock,
        apps: &[DockApp],
        position: DockPosition,
        surface: &Surface,
        window: WindowId,
    ) -> bool {
        let hwnd = window.value() as isize;
        let Some(slot) = dock.genie_target(apps, window) else {
            return false;
        };
        let screen = monitor_rect_for_window(hwnd);
        let dock_rect = window_rect(WindowId::new(surface.handle().raw().0 as u64));
        let Some(dock_rect) = dock_rect else {
            return false;
        };
        let slot_screen = dock_slot_to_screen(
            dock_rect,
            [slot.x, slot.y, slot.width, slot.height],
            surface.scale(),
        );
        let foreground = foreground_window().map_or(0, |id| id.value() as isize);
        self.director
            .begin_restore(hwnd, slot_screen, screen, position, foreground)
    }

    /// Avanza el vuelo; devuelve ventanas a restaurar ahora mismo.
    pub(crate) fn on_frame(&mut self, now: Instant) -> Vec<WindowId> {
        self.director
            .advance(now)
            .into_iter()
            .map(|hwnd| WindowId::new(hwnd as u64))
            .collect()
    }

    /// Al cerrar: devuelve transiciones y olvida vuelos.
    pub(crate) fn shutdown(&mut self) {
        self.director.restore_calmed();
    }

    /// Devuelve transiciones sin tocar vuelos (al apagar la opcion).
    pub(crate) fn restore_transitions(&mut self) {
        self.director.restore_calmed();
    }
}

/// Pasa al puente el estado ya sincronizado de la sesión.
pub(crate) fn refresh(session: &mut crate::dock_session::DockSession, surface: &Surface) {
    let position = session.preferences.dock.position;
    let (genie, dock, state) = (&mut session.genie, &session.dock, &session.state);
    genie.on_refresh(dock, state.apps(), position, surface);
}
