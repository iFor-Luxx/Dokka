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

//! De dónde salió una ventana minimizada, para volar de vuelta. La imagen no se
//! guarda: Windows la conserva viva.

use crate::geometry::{GenieEdge, PixelRect};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Departure {
    /// La ventana entera, con sus bordes invisibles.
    pub(crate) whole: PixelRect,
    /// Lo que se veía.
    pub(crate) frame: PixelRect,
    /// Borde del dock de entonces.
    pub(crate) edge: GenieEdge,
}

impl Departure {
    /// Ancho entre alto en pantalla.
    pub(crate) fn aspect(&self) -> f32 {
        self.frame.width() as f32 / self.frame.height().max(1) as f32
    }
}
