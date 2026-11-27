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

//! Embudo del efecto genio: matematica pura, sin Windows ni compositor.
//!
//! La ventana se corta en tiras horizontales. Dos cosas pasan a la vez:
//! los bordes se doblan hacia el hueco (`bend`) y la imagen se desliza
//! hacia dentro (`slide`). El ancho depende solo de la altura en pantalla,
//! como liquido por un embudo. `t` va de 0 (ventana intacta) a 1 (dentro).

use std::f32::consts::PI;

/// Rectangulo por sus bordes, en las mismas unidades en ambos extremos.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edges {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

/// Donde se dibuja una tira de la imagen en un fotograma.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Strip {
    /// Filas de la imagen original, como fraccion de su altura.
    pub source_top: f32,
    pub source_bottom: f32,
    /// `None` cuando la tira ya entro en el hueco.
    pub target: Option<Edges>,
}

/// Ventana que vuela hacia su hueco del dock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Genie {
    pub window: Edges,
    pub slot: Edges,
}

/// Los bordes terminan de doblarse a mitad del vuelo.
const BEND_END: f32 = 0.5;
/// La imagen empieza a deslizarse al 15% del vuelo.
const SLIDE_START: f32 = 0.15;

/// Duracion del vuelo en segundos, igual al minimizar que al restaurar.
pub const GENIE_SECONDS: f32 = 0.35;
/// El ultimo fotograma se queda quieto para que la ventana real aparezca debajo.
pub const GENIE_SETTLE_SECONDS: f32 = 0.12;
/// La miniatura del dock aparece durante el tramo final del vuelo.
pub const GENIE_LANDING_START: f32 = 0.8;
/// Filas de pantalla por tira: el borde del embudo avanza de a pocos pixeles.
pub const GENIE_STRIP_ROWS: u32 = 3;
/// Tope de tiras por vuelo.
pub const GENIE_MAX_STRIPS: u32 = 400;
/// Filas de mas que dibuja cada tira para no dejar costuras.
pub const GENIE_OVERLAP_ROWS: f32 = 1.0;
/// Fotogramas que se espera el hueco antes de rendirse.
pub const GENIE_AIM_FRAMES: u32 = 2;
/// Fotogramas que conviven la foto fija y las tiras antes de soltar la foto.
pub const GENIE_HANDOVER_FRAMES: u32 = 2;
/// Se abandona una copia de pantalla que tarda mas que esto.
pub const GENIE_CAPTURE_TIMEOUT_SECONDS: f32 = 0.75;
/// Se abandona un vuelo que tarda mas que esto.
pub const GENIE_FLIGHT_TIMEOUT_SECONDS: f32 = 2.0;

fn smoothstep(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

impl Genie {
    /// Cuanto se doblaron los bordes hacia el hueco con el progreso `t`.
    #[must_use]
    pub fn bend(t: f32) -> f32 {
        smoothstep(t / BEND_END)
    }

    /// Cuanto se deslizo la imagen hacia el hueco con el progreso `t`.
    #[must_use]
    pub fn slide(t: f32) -> f32 {
        let x = ((t - SLIDE_START) / (1.0 - SLIDE_START)).clamp(0.0, 1.0);
        x * x
    }

    /// Bordes izquierdo y derecho de la imagen a la altura `y`.
    fn sides(&self, y: f32, bend: f32) -> (f32, f32) {
        let span = self.slot.top - self.window.top;
        let along = if span > 0.0 {
            ((y - self.window.top) / span).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let curve = (1.0 - (PI * along).cos()) / 2.0 * bend;
        (
            self.window.left + (self.slot.left - self.window.left) * curve,
            self.window.right + (self.slot.right - self.window.right) * curve,
        )
    }

    /// Coloca `strips.len()` tiras iguales para el progreso `t`.
    /// Reusa el bufer: no reserva memoria por fotograma.
    pub fn frame(&self, t: f32, strips: &mut [Strip]) {
        let count = strips.len().max(1) as f32;
        let height = self.window.bottom - self.window.top;
        let bend = Self::bend(t);
        let shift = Self::slide(t) * (self.slot.bottom - self.window.top).max(0.0);
        for (index, strip) in strips.iter_mut().enumerate() {
            let source_top = index as f32 / count;
            let source_bottom = (index + 1) as f32 / count;
            let top = self.window.top + source_top * height + shift;
            let bottom = (self.window.top + source_bottom * height + shift).min(self.slot.bottom);
            strip.source_top = source_top;
            strip.source_bottom = source_bottom;
            strip.target = (top < self.slot.bottom).then(|| {
                let (left, right) = self.sides((top + bottom) / 2.0, bend);
                Edges {
                    left,
                    top,
                    right,
                    bottom,
                }
            });
        }
    }
}

/// Cuantas tiras lleva una ventana de esta altura en pixeles.
pub fn strip_count(window_height: u32) -> u32 {
    (window_height / GENIE_STRIP_ROWS).clamp(1, GENIE_MAX_STRIPS)
}

/// Progreso lineal de 0 a 1 con segundos transcurridos y duracion.
pub fn flight_progress(elapsed_seconds: f32, total_seconds: f32) -> f32 {
    if total_seconds <= 0.0 {
        return 1.0;
    }
    (elapsed_seconds / total_seconds).clamp(0.0, 1.0)
}

/// Opacidad de 0 a 255 de la miniatura del dock: aparece al aterrizar
/// y se va al despegar. Misma curva que el vuelo original.
pub fn landing_opacity(progress: f32, restoring: bool) -> u8 {
    let t = if restoring { 1.0 - progress } else { progress };
    (((t - GENIE_LANDING_START) / (1.0 - GENIE_LANDING_START)).clamp(0.0, 1.0) * 255.0) as u8
}
