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

//! Avance del vuelo por fotogramas: espera el hueco, despega y se asienta.
//!
//! Solo el final es nuestro: las tiras las mueve el compositor.

use std::time::{Duration, Instant};

use dokka_animation::{
    GENIE_AIM_FRAMES, GENIE_FLIGHT_TIMEOUT_SECONDS, GENIE_SETTLE_SECONDS, flight_progress,
};
use windows::Win32::Foundation::HWND;

use crate::departure::Departure;
use crate::flight::Flight;
use crate::geometry::{PixelRect, to_local};
use crate::overlay::tuck_under;
use crate::phase::Phase;
use crate::strips::{GenieCompositor, bind, clip_at_slot, launch};

impl Flight {
    /// Da al vuelo su hueco del dock en píxeles de pantalla.
    pub(crate) fn aim(&mut self, gfx: &GenieCompositor, slot: PixelRect) {
        self.genie.slot = self.turn.edges(to_local(slot, self.screen));
        self.aimed = true;
        let _ = clip_at_slot(gfx, &self.stage.root, &self.genie, self.turn);
        let _ = gfx.live.commit();
        eprintln!(
            "genio: vuelo {} apuntado al hueco [{}, {}, {}, {}]",
            self.hwnd, slot.left, slot.top, slot.right, slot.bottom
        );
    }

    /// Pone el vuelo bajo el dock para pasar tras su cristal.
    pub(crate) fn tuck_under(&self, dock: HWND) {
        tuck_under(self.stage.overlay, dock);
    }

    /// Un restaurar tardío pertenece a su petición: si otra app ganó el foco,
    /// el vuelo está rancio. Minimizar siempre sigue su curso.
    pub(crate) fn is_current(&self, foreground: isize) -> bool {
        !self.restoring
            || foreground == 0
            || foreground == self.foreground
            || foreground == self.hwnd
    }

    /// `true`, una sola vez, cuando toca restaurar la ventana real.
    pub(crate) fn take_restore(&mut self, finished: bool) -> bool {
        let due = finished || matches!(self.phase, Phase::Settling { .. });
        due && std::mem::take(&mut self.restore_pending)
    }

    /// De dónde salió, para volver con el vuelo al revés. Solo si despegó.
    pub(crate) fn take_departure(&mut self) -> Option<Departure> {
        if matches!(self.phase, Phase::Waiting { .. }) {
            return None;
        }
        self.departure.take()
    }

    /// Opacidad de 0 a 1 de la miniatura del dock en este instante.
    pub(crate) fn landing(&self, now: Instant) -> f32 {
        let Phase::Flying { start, .. } = &self.phase else {
            return 0.0;
        };
        let progress = flight_progress(
            now.saturating_duration_since(*start).as_secs_f32(),
            self.seconds,
        );
        f32::from(dokka_animation::landing_opacity(progress, self.restoring)) / 255.0
    }

    /// Avanza el vuelo; `false` cuando terminó o ya no puede seguir.
    pub(crate) fn step(&mut self, gfx: &GenieCompositor, now: Instant) -> bool {
        let limit = Duration::from_secs_f32(GENIE_FLIGHT_TIMEOUT_SECONDS);
        if now.saturating_duration_since(self.created_at) >= limit {
            self.restore_pending = false;
            return false;
        }
        match &mut self.phase {
            Phase::Waiting { frames } => {
                *frames += 1;
                if !self.aimed {
                    return *frames <= GENIE_AIM_FRAMES;
                }
                self.take_off(gfx)
            }
            Phase::Flying { start, .. } => {
                let progress = flight_progress(
                    now.saturating_duration_since(*start).as_secs_f32(),
                    self.seconds,
                );
                if progress < 1.0 {
                    return true;
                }
                if !self.restoring {
                    return false;
                }
                self.phase = Phase::Settling {
                    until: now + Duration::from_secs_f32(GENIE_SETTLE_SECONDS),
                };
                true
            }
            Phase::Settling { until } => now < *until,
        }
    }

    /// Ata las tiras al embudo y las lanza.
    fn take_off(&mut self, gfx: &GenieCompositor) -> bool {
        let flight = bind(gfx, &self.genie, &self.stage.pieces, false).and_then(|drive| {
            launch(gfx, &self.genie, &drive, false, self.seconds)?;
            gfx.live.commit()?;
            Ok(drive)
        });
        match flight {
            Ok(drive) => {
                eprintln!(
                    "genio: vuelo {} despega tras {} ms",
                    self.hwnd,
                    self.created_at.elapsed().as_millis()
                );
                self.phase = Phase::Flying {
                    start: Instant::now(),
                    _drive: drive,
                };
                true
            }
            Err(error) => {
                eprintln!(
                    "genio: vuelo {} cancelado: no despego ({:#X})",
                    self.hwnd,
                    error.code().0
                );
                false
            }
        }
    }
}
