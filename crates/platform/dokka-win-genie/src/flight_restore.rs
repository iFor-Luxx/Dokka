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

//! Vuelo de vuelta: desde el hueco hasta su sitio original.

use std::time::Instant;

use dokka_animation::{GENIE_SECONDS, Genie};

use crate::departure::Departure;
use crate::flight::Flight;
use crate::geometry::{GenieEdge, PixelRect, Turn, to_local};
use crate::overlay::show_no_activate;
use crate::phase::Phase;
use crate::scene::Stage;
use crate::strips::{GenieCompositor, bind, clip_at_slot, launch};

impl Flight {
    /// Vuelo de vuelta hasta donde estaba. La ventana real la restaura quien
    /// llama al llegar a tamaño completo.
    pub(crate) fn restore(
        gfx: &GenieCompositor,
        hwnd: isize,
        departure: Departure,
        slot: PixelRect,
        screen: PixelRect,
        edge: GenieEdge,
        foreground: isize,
    ) -> Option<Self> {
        if departure.edge != edge {
            eprintln!("genio: sin vuelo de vuelta para {hwnd}: el dock cambio de borde");
            return None;
        }
        let turn = Turn::new(edge, screen);
        let build = || -> windows::core::Result<(Stage, Genie, Phase)> {
            let area = (departure.whole, departure.frame);
            let (stage, window) = Stage::new(gfx, hwnd, area, screen, turn)?;
            let genie = Genie {
                window,
                slot: turn.edges(to_local(slot, screen)),
            };
            clip_at_slot(gfx, &stage.root, &genie, turn)?;
            let drive = bind(gfx, &genie, &stage.pieces, true)?;
            launch(gfx, &genie, &drive, true, GENIE_SECONDS)?;
            gfx.live.commit()?;
            let phase = Phase::Flying {
                start: Instant::now(),
                _drive: drive,
            };
            Ok((stage, genie, phase))
        };
        let (stage, genie, phase) = build()
            .inspect_err(|error| {
                let code = error.code().0;
                eprintln!("genio: sin vuelo de vuelta para {hwnd}: (HRESULT {code:#X})");
            })
            .ok()?;
        show_no_activate(stage.overlay);
        Some(Self {
            hwnd,
            created_at: Instant::now(),
            seconds: GENIE_SECONDS,
            foreground,
            stage,
            screen,
            turn,
            genie,
            aimed: true,
            phase,
            restoring: true,
            restore_pending: true,
            departure: None,
        })
    }
}
