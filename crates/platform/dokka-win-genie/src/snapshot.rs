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

//! Foto reducida de una ventana minimizada para volar de vuelta.
//!
//! A mitad de resolucion en cada eje: un cuarto de memoria, y la suavidad no
//! se nota en medio segundo de vuelo.

use crate::GenieImage;
use crate::capture::halved;
use crate::geometry::{GenieEdge, PixelRect};

/// Lo que parecia una ventana minimizada, para restaurarla con genio.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Girada segun el borde del dock de entonces.
    pub(crate) image: GenieImage,
    /// Donde estaba la ventana, en pixeles de pantalla.
    pub(crate) window: PixelRect,
    pub(crate) edge: GenieEdge,
}

impl Snapshot {
    pub(crate) fn from_capture(image: GenieImage, window: PixelRect, edge: GenieEdge) -> Self {
        let reduced = halved(&image);
        Self {
            image: reduced,
            window,
            edge,
        }
    }

    /// Ancho entre alto en pantalla.
    pub fn aspect(&self) -> f32 {
        self.window.width() as f32 / self.window.height().max(1) as f32
    }
}
