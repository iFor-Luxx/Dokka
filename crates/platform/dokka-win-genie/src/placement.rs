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

//! Dónde cae en pantalla la imagen viva y qué trozo de ella lleva cada tira.

use crate::geometry::{GenieEdge, PixelRect, Turn};

/// Diferencia de tamaño que aún se explica por bordes invisibles.
const BORDER_SLACK: i32 = 64;

pub(crate) struct Placement {
    /// La imagen entera, en pantalla.
    image: PixelRect,
    /// Lo que cabe en el monitor: lo que vuela.
    pub(crate) visible: PixelRect,
    source: (i32, i32),
}

impl Placement {
    /// `whole` es la ventana con sus bordes; `frame`, lo que se veía (distinto
    /// solo si estaba maximizada); `source`, el tamaño de la imagen de Windows.
    pub(crate) fn new(
        whole: PixelRect,
        frame: PixelRect,
        source: (i32, i32),
        screen: PixelRect,
    ) -> Option<Self> {
        if source.0 <= 0 || source.1 <= 0 {
            return None;
        }
        // A tamaño exacto no hay estirado; si no cuadra (apps sin escala
        // propia), se ajusta a lo que se veía.
        let close = (source.0 - frame.width()).abs() <= BORDER_SLACK
            && (source.1 - frame.height()).abs() <= BORDER_SLACK;
        let (width, height) = if close {
            source
        } else {
            (frame.width(), frame.height())
        };
        let left = frame.left + (frame.width() - width) / 2;
        // Arriba no hay borde invisible, salvo maximizada, que desborda igual.
        let top = if whole == frame {
            frame.top
        } else {
            frame.top + (frame.height() - height) / 2
        };
        let image = PixelRect {
            left,
            top,
            right: left + width,
            bottom: top + height,
        };
        let visible = image.intersect(screen);
        (!visible.is_empty()).then_some(Self {
            image,
            visible,
            source,
        })
    }

    /// Trozo `[x, y, ancho, alto]` de lo visible que toca a las filas
    /// `top..top + rows` del marco del vuelo.
    pub(crate) fn strip(&self, turn: Turn, top: i32, rows: i32) -> [i32; 4] {
        let (width, height) = (self.visible.width(), self.visible.height());
        match turn.edge {
            GenieEdge::Bottom => [0, top, width, rows],
            GenieEdge::Left => [width - top - rows, 0, rows, height],
            GenieEdge::Right => [top, 0, rows, height],
        }
    }

    /// El mismo trozo en píxeles de la imagen de origen.
    pub(crate) fn in_source(&self, [x, y, width, height]: [i32; 4]) -> [i32; 4] {
        let x = x + self.visible.left - self.image.left;
        let y = y + self.visible.top - self.image.top;
        let scale = |value: i32, from: i32, to: i32| (value * to + from / 2) / from.max(1);
        let (wide, tall) = (self.image.width(), self.image.height());
        [
            scale(x, wide, self.source.0),
            scale(y, tall, self.source.1),
            scale(width, wide, self.source.0),
            scale(height, tall, self.source.1),
        ]
    }
}
