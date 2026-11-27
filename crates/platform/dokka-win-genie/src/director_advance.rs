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

//! Cola del director: cancelar vuelos rancios, avanzar y recordar salidas.

use std::time::Instant;

use crate::director::{GenieDirector, MAX_DEPARTURES};

impl GenieDirector {
    /// Ventana en vuelo y cuánto debe verse su miniatura del dock: nada
    /// mientras vuela, fundida al aterrizar (como Muelle).
    pub fn landing(&self, now: Instant) -> Option<(isize, f32)> {
        let flight = self.flight.as_ref()?;
        Some((flight.hwnd, flight.landing(now)))
    }

    /// Pone el vuelo en curso bajo el dock para pasar tras su cristal.
    /// Solo con el dock oculto: arriba del todo taparía la barrita.
    pub fn tuck_current_if_hidden(&self, hidden: bool) {
        if !hidden {
            return;
        }
        if let (Some(flight), Some(dock)) = (self.flight.as_ref(), self.dock_window) {
            flight.tuck_under(dock);
        }
    }

    /// Proporción de la ventana minimizada, si se recuerda.
    pub fn snapshot_aspect(&self, hwnd: isize) -> Option<f32> {
        self.departures
            .iter()
            .find(|(id, _)| *id == hwnd)
            .map(|(_, departure)| departure.aspect())
    }

    /// Cancela vuelos rancios: otra app ganó el foco o se restauró por fuera.
    pub fn cancel_if_stale(&mut self, foreground: isize, minimized: &[isize]) {
        let stale = match self.flight.as_ref() {
            Some(flight) => !flight.is_current(foreground),
            None => false,
        };
        if stale {
            let hwnd = self.flight.as_ref().map(|flight| flight.hwnd).unwrap_or(0);
            eprintln!("genio: vuelo {hwnd} cancelado: otra app gano el foco ({foreground})");
            self.flight = None;
        }
        self.departures.retain(|(id, _)| minimized.contains(id));
    }

    /// Avanza el vuelo; devuelve ventanas a restaurar ahora mismo.
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
                if let Some(departure) = flight.take_departure() {
                    self.departures.push((flight.hwnd, departure));
                    while self.departures.len() > MAX_DEPARTURES {
                        self.departures.remove(0);
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
}
