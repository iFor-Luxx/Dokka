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

use dokka_animation::GENIE_SECONDS;
use dokka_model::DockPosition;
use windows::Win32::Foundation::HWND;

use crate::director_support::{dock_handle, ensure_compositor, rect_of};
use crate::flight::Flight;
use crate::geometry::{GenieEdge, PixelRect};
use crate::origin::is_iconic_now;
use crate::snapshot::Snapshot;
use crate::strips::GenieCompositor;
use crate::transitions::{animations_enabled, calm_window, restore_all_transitions};

/// Fotos guardadas para restaurar con genio.
pub(crate) const MAX_SNAPSHOTS: usize = 4;

/// Un vuelo y lo necesario para el siguiente. Solo un vuelo a la vez.
pub struct GenieDirector {
    pub(crate) enabled: bool,
    pub(crate) gfx: Option<GenieCompositor>,
    pub(crate) gfx_failed: bool,
    pub(crate) flight: Option<Flight>,
    pub(crate) snapshots: Vec<(isize, Snapshot)>,
    pub(crate) calmed: Vec<isize>,
    pub(crate) started: bool,
    pub(crate) dock_window: Option<HWND>,
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
        if enabled {
            // Como Muelle: crearlo ahora evita que la primera vez tarde.
            ensure_compositor(&mut self.gfx, &mut self.gfx_failed, 0);
        }
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
        self.calmed.retain(|known| windows.contains(known));
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
    /// `from` es la ventana entera y `frame` lo que se veía; `false` si esta
    /// apagado, ya vuela algo o falla Windows: minimizar normal.
    pub fn begin_minimize(
        &mut self,
        hwnd: isize,
        from: [i32; 4],
        frame: [i32; 4],
        screen: [i32; 4],
        position: DockPosition,
        foreground: isize,
    ) -> bool {
        if !self.enabled {
            eprintln!("genio: sin vuelo para {hwnd}: efecto apagado en Preferencias");
            return false;
        }
        if !animations_enabled() {
            eprintln!("genio: sin vuelo para {hwnd}: animaciones del sistema apagadas");
            return false;
        }
        if !self.started {
            eprintln!("genio: sin vuelo para {hwnd}: aun no arranco el seguimiento");
            return false;
        }
        if self.flight.is_some() {
            eprintln!("genio: sin vuelo para {hwnd}: ya vuela otra ventana");
            return false;
        }
        ensure_compositor(&mut self.gfx, &mut self.gfx_failed, hwnd);
        let Some(gfx) = self.gfx.as_ref() else {
            return false;
        };
        let edge = GenieEdge::from_position(position);
        let flight = Flight::start(
            gfx,
            hwnd,
            (
                PixelRect::from_win(rect_of(from)),
                PixelRect::from_win(rect_of(frame)),
            ),
            PixelRect::from_win(rect_of(screen)),
            GENIE_SECONDS,
            edge,
            foreground,
        );
        match flight {
            Some(flight) => {
                eprintln!(
                    "genio: vuelo {hwnd} empieza desde [{}, {}, {}, {}]",
                    from[0], from[1], from[2], from[3]
                );
                self.flight = Some(flight);
                true
            }
            None => false,
        }
    }

    /// Da al vuelo en curso su hueco del dock en pixeles de pantalla.
    pub fn aim_current(&mut self, slot: [i32; 4]) {
        let Some(flight) = self.flight.as_mut() else {
            eprintln!("genio: sin apuntar: no hay vuelo en curso");
            return;
        };
        // Sin compositor no hay vuelo; no puede pasar, pero falla solo.
        let Some(gfx) = self.gfx.as_ref() else {
            eprintln!(
                "genio: sin apuntar el vuelo {}: no hay compositor",
                flight.hwnd
            );
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
        if !self.enabled {
            eprintln!("genio: sin vuelta para {hwnd}: efecto apagado en Preferencias");
            return false;
        }
        if !animations_enabled() {
            eprintln!("genio: sin vuelta para {hwnd}: animaciones del sistema apagadas");
            return false;
        }
        if self.flight.is_some() {
            eprintln!("genio: sin vuelta para {hwnd}: ya vuela otra ventana");
            return false;
        }
        // La ventana ya visible no necesita vuelo: pasó algo por fuera.
        if !is_iconic_now(hwnd) {
            eprintln!("genio: sin vuelta para {hwnd}: ya no esta minimizada");
            return false;
        }
        let index = self.snapshots.iter().position(|(id, _)| *id == hwnd);
        let Some(index) = index else {
            eprintln!("genio: sin vuelta para {hwnd}: no hay foto guardada");
            return false;
        };
        ensure_compositor(&mut self.gfx, &mut self.gfx_failed, hwnd);
        let Some(gfx) = self.gfx.as_ref() else {
            return false;
        };
        let edge = GenieEdge::from_position(position);
        let flight = Flight::restore(
            gfx,
            hwnd,
            &self.snapshots[index].1,
            PixelRect::from_win(rect_of(slot)),
            PixelRect::from_win(rect_of(screen)),
            edge,
            foreground,
        );
        match flight {
            Some(flight) => {
                eprintln!("genio: vuelo {hwnd} de vuelta despega");
                self.snapshots.remove(index);
                self.flight = Some(flight);
                true
            }
            None => false,
        }
    }
}

impl Default for GenieDirector {
    fn default() -> Self {
        Self::new()
    }
}
