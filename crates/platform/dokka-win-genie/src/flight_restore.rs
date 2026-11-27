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

//! Vuelo de vuelta: desde la foto guardada hasta su sitio original.

use std::time::Instant;

use dokka_animation::Genie;

use crate::flight::Flight;
use crate::geometry::{PixelRect, to_local};
use crate::overlay::{create_overlay, destroy, show_no_activate};
use crate::phase::Phase;
use crate::snapshot::Snapshot;
use crate::strips::{GenieCompositor, bind, clip_at_slot, cut, launch};

impl Flight {
    /// Vuelo de vuelta desde la foto reducida hasta su hueco original.
    /// La ventana real la restaura quien llama al llegar a tamaño completo.
    pub(crate) fn restore(
        gfx: &GenieCompositor,
        hwnd: isize,
        snapshot: &Snapshot,
        slot: PixelRect,
        screen: PixelRect,
        edge: crate::geometry::GenieEdge,
        foreground: isize,
    ) -> Option<Self> {
        let source = snapshot.window.intersect(screen);
        if source.is_empty() {
            eprintln!("genio: sin vuelo de vuelta para {hwnd}: su foto quedo fuera del monitor");
            return None;
        }
        if snapshot.edge != edge {
            eprintln!("genio: sin vuelo de vuelta para {hwnd}: el dock cambio de borde");
            return None;
        }
        let turn = crate::geometry::Turn::new(edge, screen);
        let (overlay, target, root) = create_overlay(&gfx.compositor, screen).or_else(|| {
            eprintln!("genio: sin vuelo de vuelta para {hwnd}: no se creo el overlay");
            None
        })?;
        let genie = Genie {
            window: turn.edges(to_local(source, screen)),
            slot: turn.edges(to_local(slot, screen)),
        };
        let height = (genie.window.bottom - genie.window.top) as u32;
        let image = &snapshot.image;
        let size = (image.width as f32, image.height as f32);
        let drive = turn
            .apply(&root)
            .and_then(|()| clip_at_slot(gfx, &root, &genie, turn))
            .and_then(|()| cut(gfx, &root, image, height))
            .and_then(|pieces| bind(gfx, &genie, &pieces, size, true))
            .and_then(|drive| {
                launch(gfx, &genie, &drive, true, dokka_animation::GENIE_SECONDS).map(|()| drive)
            });
        let Ok(drive) = drive.inspect_err(|error| {
            let code = error.code().0;
            eprintln!(
                "genio: sin vuelo de vuelta para {hwnd}: fallo el compositor (HRESULT {code:#X})"
            );
        }) else {
            destroy(overlay);
            return None;
        };
        let flight = Self {
            hwnd,
            created_at: Instant::now(),
            seconds: dokka_animation::GENIE_SECONDS,
            foreground,
            overlay,
            stand: None,
            _target: target,
            root,
            screen,
            turn,
            source,
            genie,
            aimed: true,
            phase: Phase::Flying {
                start: Instant::now(),
                _drive: drive,
                stand_in: None,
            },
            restoring: true,
            restore_pending: true,
            captured: None,
        };
        show_no_activate(flight.overlay);
        Some(flight)
    }
}
