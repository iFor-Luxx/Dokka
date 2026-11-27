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

//! Director de vuelos: un vuelo a la vez, fotos para volver y transiciones.
//!
//! Si algo falla devuelve `None` o `false` y la ventana sigue su curso normal.
//! Apaga la animacion propia de las ventanas seguidas y la devuelve al apagar
//! la opcion o al cerrar.

use std::time::Instant;

use dokka_animation::GENIE_SECONDS;
use dokka_model::DockPosition;
use windows::Win32::Foundation::HWND;

use crate::flight::Flight;
use crate::geometry::{GenieEdge, PixelRect};
use crate::snapshot::Snapshot;
use crate::strips::GenieCompositor;
use crate::transitions::{animations_enabled, calm_window, restore_all_transitions};

/// Fotos guardadas para restaurar con genio.
const MAX_SNAPSHOTS: usize = 4;

/// Un vuelo y lo necesario para el siguiente. Solo un vuelo a la vez.
pub struct GenieDirector {
    enabled: bool,
    gfx: Option<GenieCompositor>,
    gfx_failed: bool,
    flight: Option<Flight>,
    snapshots: Vec<(isize, Snapshot)>,
    calmed: Vec<isize>,
    started: bool,
    dock_window: Option<HWND>,
}

impl GenieDirector {
    /// Director apagado hasta que el shell lo enciende con la preferencia.
    pub fn new() -> Self {
        Self {
            enabled: false,
            gfx: None,
            gfx_failed: false,
            flight: None,
            snapshots: Vec::new(),
            calmed: Vec::new(),
            started: false,
            dock_window: None,
        }
    }

    /// Ventana del dock para pasar el vuelo tras su cristal.
    pub fn set_dock_window(&mut self, raw: isize) {
        self.dock_window = Some(dock_handle(raw));
    }

    /// Enciende o apaga el efecto. Al apagar devuelve transiciones y olvida fotos.
    pub fn set_enabled(&mut self, enabled: bool) {
        if self.enabled == enabled {
            return;
        }
        self.enabled = enabled;
        if !enabled {
            self.flight = None;
            self.snapshots.clear();
            self.restore_calmed();
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// `true` mientras hay un vuelo dibujandose.
    pub fn is_flying(&self) -> bool {
        self.flight.is_some()
    }

    /// Marca que ya paso el primer arranque: las minimizadas de antes no vuelan.
    pub fn mark_started(&mut self) {
        self.started = true;
    }

    /// Apaga la animacion propia de estas ventanas para sustituirla.
    /// Solo con el efecto encendido y animaciones del sistema activas.
    pub fn calm_windows(&mut self, windows: &[isize]) {
        if !self.enabled || !animations_enabled() {
            return;
        }
        for &window in windows {
            if !self.calmed.contains(&window) {
                calm_window(window);
                self.calmed.push(window);
            }
        }
    }

    /// Devuelve la animacion propia a las ventanas apagadas.
    pub fn restore_calmed(&mut self) {
        restore_all_transitions(&self.calmed);
        self.calmed.clear();
    }

    /// Empieza a minimizar: foto fija donde estaba. El hueco llega con `aim`.
    /// `None` si esta apagado, ya vuela algo o falla Windows: minimizar normal.
    pub fn begin_minimize(
        &mut self,
        hwnd: isize,
        from: [i32; 4],
        screen: [i32; 4],
        position: DockPosition,
        foreground: isize,
    ) -> bool {
        if !self.enabled || !animations_enabled() || !self.started {
            return false;
        }
        if self.flight.is_some() {
            return false;
        }
        let Some(gfx) = self.compositor() else {
            return false;
        };
        let edge = GenieEdge::from_position(position);
        let flight = Flight::start(
            gfx,
            hwnd,
            PixelRect::from_win(rect_of(from)),
            PixelRect::from_win(rect_of(screen)),
            GENIE_SECONDS,
            edge,
            foreground,
        );
        match flight {
            Some(flight) => {
                if let Some(dock) = self.dock_window {
                    flight.tuck_under(dock);
                }
                self.flight = Some(flight);
                true
            }
            None => false,
        }
    }

    /// Da al vuelo en curso su hueco del dock en pixeles de pantalla.
    pub fn aim_current(&mut self, slot: [i32; 4]) {
        let Some(flight) = self.flight.as_mut() else {
            return;
        };
        // Sin compositor no hay vuelo; no puede pasar, pero falla solo.
        let Some(gfx) = self.gfx.as_ref() else {
            return;
        };
        flight.aim(gfx, PixelRect::from_win(rect_of(slot)));
    }

    /// Pide restaurar desde el dock: vuela al reves desde su foto.
    /// `false` si no hay foto o falla: restaurar normal.
    pub fn begin_restore(
        &mut self,
        hwnd: isize,
        slot: [i32; 4],
        screen: [i32; 4],
        position: DockPosition,
        foreground: isize,
    ) -> bool {
        if !self.enabled || !animations_enabled() {
            return false;
        }
        if self.flight.is_some() {
            return false;
        }
        let index = self.snapshots.iter().position(|(id, _)| *id == hwnd);
        let Some(index) = index else {
            return false;
        };
        let (_, snapshot) = self.snapshots.remove(index);
        let Some(gfx) = self.compositor() else {
            return false;
        };
        let edge = GenieEdge::from_position(position);
        let flight = Flight::restore(
            gfx,
            hwnd,
            &snapshot,
            PixelRect::from_win(rect_of(slot)),
            PixelRect::from_win(rect_of(screen)),
            edge,
            foreground,
        );
        match flight {
            Some(flight) => {
                if let Some(dock) = self.dock_window {
                    flight.tuck_under(dock);
                }
                self.flight = Some(flight);
                true
            }
            None => false,
        }
    }

    /// Cancela vuelos rancios: otra app gano el foco o se restauro por fuera.
    /// Las minimizadas por fuera no vuelan; sus fotos huerfanas se olvidan.
    pub fn cancel_if_stale(&mut self, foreground: isize, minimized: &[isize]) {
        let stale = match self.flight.as_ref() {
            Some(flight) => !flight.is_current(foreground),
            None => false,
        };
        if stale {
            self.flight = None;
        }
        self.snapshots.retain(|(id, _)| minimized.contains(id));
    }

    /// Avanza el vuelo; devuelve ventanas a restaurar ahora mismo.
    /// Al terminar un minimizar guarda su foto (hasta 4).
    pub fn advance(&mut self, now: Instant) -> Vec<isize> {
        let mut restore = Vec::new();
        let Some(gfx) = self.gfx.as_ref() else {
            return restore;
        };
        let mut finished = false;
        if let Some(flight) = self.flight.as_mut() {
            let going = flight.step(gfx, now);
            if flight.take_restore(!going) {
                restore.push(flight.hwnd);
            }
            if !going {
                if let Some(snapshot) = flight.take_snapshot_for_restore() {
                    self.snapshots.push((flight.hwnd, snapshot));
                    while self.snapshots.len() > MAX_SNAPSHOTS {
                        self.snapshots.remove(0);
                    }
                }
                finished = true;
            }
        }
        if finished {
            self.flight = None;
        }
        restore
    }

    fn compositor(&mut self) -> Option<&GenieCompositor> {
        if self.gfx.is_none() && !self.gfx_failed {
            match GenieCompositor::new() {
                Ok(gfx) => self.gfx = Some(gfx),
                Err(error) => {
                    self.gfx_failed = true;
                    eprintln!("Dokka no pudo preparar el genio: {error}");
                }
            }
        }
        self.gfx.as_ref()
    }
}

impl Default for GenieDirector {
    fn default() -> Self {
        Self::new()
    }
}

fn rect_of(area: [i32; 4]) -> windows::Win32::Foundation::RECT {
    windows::Win32::Foundation::RECT {
        left: area[0],
        top: area[1],
        right: area[2],
        bottom: area[3],
    }
}

/// Deja pasar el tipo del handle sin exponer Windows fuera de `platform`.
pub(crate) fn dock_handle(raw: isize) -> HWND {
    HWND(raw as *mut std::ffi::c_void)
}
