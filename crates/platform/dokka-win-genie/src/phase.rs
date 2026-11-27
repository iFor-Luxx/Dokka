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

//! Fases del vuelo: foto fija esperando la copia, tiras volando y reposo.

use std::sync::mpsc::Receiver;
use std::time::Instant;

use windows::UI::Composition::CompositionPropertySet;

use crate::GenieImage;

/// Copia de pantalla y su versión reducida para volver.
pub(crate) struct Captured {
    pub(crate) image: GenieImage,
    pub(crate) restore: GenieImage,
}

pub(crate) enum Phase {
    /// La miniatura fija está en pantalla mientras un hilo copia la pantalla.
    Standing {
        thumbnail: isize,
        frames: u32,
        shot: Receiver<Option<Captured>>,
        started: Instant,
    },
    Flying {
        start: Instant,
        /// Se guarda: las tiras lo leen en sus animaciones.
        _drive: CompositionPropertySet,
        /// La foto fija de un minimizar y los fotogramas que lleva sobre las tiras.
        stand_in: Option<(isize, u32)>,
    },
    /// Un restaurar llegó a tamaño completo y espera a la ventana real.
    Settling { until: Instant },
}
