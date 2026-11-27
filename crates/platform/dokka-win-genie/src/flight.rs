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

//! Un vuelo en curso. Al minimizar, las tiras aparecen donde estaba la ventana
//! con su imagen viva y despegan al llegar el hueco.

use std::time::Instant;

use dokka_animation::Genie;

use crate::departure::Departure;
use crate::geometry::{GenieEdge, PixelRect, Turn};
use crate::overlay::show_no_activate;
use crate::phase::Phase;
use crate::scene::Stage;
use crate::strips::GenieCompositor;

/// Un vuelo en curso hacia el dock o de vuelta.
pub(crate) struct Flight {
    pub hwnd: isize,
    pub(crate) created_at: Instant,
    pub(crate) seconds: f32,
    pub(crate) foreground: isize,
    pub(crate) stage: Stage,
    pub(crate) screen: PixelRect,
    pub(crate) turn: Turn,
    pub(crate) genie: Genie,
    pub(crate) aimed: bool,
    pub(crate) phase: Phase,
    pub(crate) restoring: bool,
    pub(crate) restore_pending: bool,
    pub(crate) departure: Option<Departure>,
}

impl Flight {
    /// Tiras quietas donde estaba la ventana; despegarán con `aim`.
    /// `from` es la ventana entera en píxeles de pantalla (con bordes
    /// invisibles); `frame`, lo que se veía. `screen` es el monitor.
    pub(crate) fn start(
        gfx: &GenieCompositor,
        hwnd: isize,
        (from, frame): (PixelRect, PixelRect),
        screen: PixelRect,
        seconds: f32,
        edge: GenieEdge,
        foreground: isize,
    ) -> Option<Self> {
        let turn = Turn::new(edge, screen);
        let (stage, window) = Stage::new(gfx, hwnd, (from, frame), screen, turn)
            .and_then(|built| gfx.live.commit().map(|()| built))
            .inspect_err(|error| {
                let code = error.code().0;
                eprintln!("genio: sin vuelo para {hwnd}: sin imagen viva (HRESULT {code:#X})");
            })
            .ok()?;
        show_no_activate(stage.overlay);
        Some(Self {
            hwnd,
            created_at: Instant::now(),
            seconds,
            foreground,
            stage,
            screen,
            turn,
            genie: Genie {
                window,
                slot: window,
            },
            aimed: false,
            phase: Phase::Waiting { frames: 0 },
            restoring: false,
            restore_pending: false,
            departure: Some(Departure {
                whole: from,
                frame,
                edge,
            }),
        })
    }
}
