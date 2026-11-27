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

//! Avance del vuelo por fotogramas: espera la copia, despega y se asienta.
//!
//! El reloj arranca al despegar para que la copia de pantalla no coma
//! animacion. Solo el final es nuestro: las tiras las mueve el compositor.

use std::time::Instant;

use dokka_animation::{
    GENIE_AIM_FRAMES, GENIE_CAPTURE_TIMEOUT_SECONDS, GENIE_FLIGHT_TIMEOUT_SECONDS,
    GENIE_HANDOVER_FRAMES, GENIE_SETTLE_SECONDS, flight_progress,
};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::DwmUnregisterThumbnail;

use crate::GenieCompositor;
use crate::flight::Flight;
use crate::geometry::{PixelRect, to_local};
use crate::overlay::destroy;
use crate::overlay::tuck_under;
use crate::phase::{Captured, Phase};
use crate::snapshot::Snapshot;
use crate::strips::{bind, clip_at_slot, cut, launch};

impl Flight {
    /// Da al vuelo su hueco del dock en pixeles de pantalla.
    pub(crate) fn aim(&mut self, gfx: &GenieCompositor, slot: PixelRect) {
        self.genie.slot = self.turn.edges(to_local(slot, self.screen));
        self.aimed = true;
        let _ = clip_at_slot(gfx, &self.root, &self.genie, self.turn);
        eprintln!(
            "genio: vuelo {} apuntado al hueco [{}, {}, {}, {}]",
            self.hwnd, slot.left, slot.top, slot.right, slot.bottom
        );
    }

    /// Pone el vuelo bajo el dock para pasar tras su cristal.
    pub(crate) fn tuck_under(&self, dock: HWND) {
        tuck_under(self.overlay, self.stand, dock);
    }

    /// Un restaurar tardio pertenece a su peticion: si otra app gano el foco,
    /// el vuelo esta rancio. Minimizar siempre sigue su curso.
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

    /// Foto para restaurar despues con el vuelo al reves.
    pub(crate) fn take_snapshot_for_restore(&mut self) -> Option<Snapshot> {
        let edge = self.turn.edge;
        self.captured
            .take()
            .map(|(image, window)| Snapshot::from_capture(image, window, edge))
    }

    /// Avanza el vuelo; `false` cuando termino o ya no puede seguir.
    pub(crate) fn step(&mut self, gfx: &GenieCompositor, now: Instant) -> bool {
        let elapsed = now.saturating_duration_since(self.created_at);
        if elapsed >= flight_timeout()
            || (matches!(self.phase, Phase::Standing { .. }) && elapsed >= capture_timeout())
        {
            self.restore_pending = false;
            return false;
        }
        match &mut self.phase {
            Phase::Standing {
                thumbnail,
                frames,
                shot,
                started,
            } => {
                if now.saturating_duration_since(*started) >= capture_timeout() {
                    return false;
                }
                *frames += 1;
                if !self.aimed {
                    return *frames <= GENIE_AIM_FRAMES;
                }
                let image = match try_shot(shot) {
                    Ok(Some(image)) => image,
                    Err(std::sync::mpsc::TryRecvError::Empty) => return true,
                    Ok(None) | Err(std::sync::mpsc::TryRecvError::Disconnected) => return false,
                };
                let thumbnail = *thumbnail;
                let Ok(drive) = self.take_off(gfx, image) else {
                    eprintln!(
                        "genio: vuelo {} cancelado: no despego el compositor",
                        self.hwnd
                    );
                    return false;
                };
                eprintln!(
                    "genio: vuelo {} despega tras {} ms",
                    self.hwnd,
                    self.created_at.elapsed().as_millis()
                );
                self.phase = Phase::Flying {
                    start: Instant::now(),
                    _drive: drive,
                    stand_in: Some((thumbnail, 0)),
                };
                true
            }
            Phase::Flying {
                start, stand_in, ..
            } => {
                if let Some((thumbnail, frames)) = stand_in {
                    *frames += 1;
                    if *frames > GENIE_HANDOVER_FRAMES {
                        // SAFETY: el registro se libera una sola vez.
                        unsafe {
                            let _ = DwmUnregisterThumbnail(*thumbnail);
                        }
                        *stand_in = None;
                    }
                }
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
                    until: now + std::time::Duration::from_secs_f32(GENIE_SETTLE_SECONDS),
                };
                true
            }
            Phase::Settling { until } => now < *until,
        }
    }

    /// Corta la copia en tiras, las ata al embudo y las lanza.
    fn take_off(
        &mut self,
        gfx: &GenieCompositor,
        captured: Captured,
    ) -> windows::core::Result<windows::UI::Composition::CompositionPropertySet> {
        let image = captured.image;
        let restore = captured.restore;
        let size = (image.width as f32, image.height as f32);
        // Cada paso dice cuál falló: el compositor rechaza en silencio.
        let step = |name: &'static str| {
            move |error: &windows::core::Error| {
                eprintln!(
                    "genio: fallo al {name}: {} ({:#X})",
                    error.message(),
                    error.code().0
                );
            }
        };
        let pieces =
            cut(gfx, &self.root, &image, image.height).inspect_err(step("cortar las tiras"))?;
        let drive = bind(gfx, &self.genie, &pieces, size, false)
            .inspect_err(step("dar forma al embudo"))?;
        launch(gfx, &self.genie, &drive, false, self.seconds).inspect_err(step("lanzar"))?;
        self.captured = Some((restore, self.source));
        Ok(drive)
    }
}

impl Drop for Flight {
    fn drop(&mut self) {
        // SAFETY: los handles se crearon en `start` o `restore` y se liberan
        // una sola vez.
        unsafe {
            if let Phase::Standing { thumbnail, .. }
            | Phase::Flying {
                stand_in: Some((thumbnail, _)),
                ..
            } = &self.phase
            {
                let _ = DwmUnregisterThumbnail(*thumbnail);
            }
            if let Some(stand) = self.stand {
                destroy(stand);
            }
            destroy(self.overlay);
        }
    }
}

/// Tiempo limite del vuelo, como duracion.
pub(crate) fn flight_timeout() -> std::time::Duration {
    std::time::Duration::from_secs_f32(GENIE_FLIGHT_TIMEOUT_SECONDS)
}

/// Tiempo limite de la copia de pantalla, como duracion.
pub(crate) fn capture_timeout() -> std::time::Duration {
    std::time::Duration::from_secs_f32(GENIE_CAPTURE_TIMEOUT_SECONDS)
}

/// Resultado de intentar recibir la copia del hilo.
pub(crate) fn try_shot(
    shot: &std::sync::mpsc::Receiver<Option<Captured>>,
) -> Result<Option<Captured>, std::sync::mpsc::TryRecvError> {
    shot.try_recv()
}
