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

//! Lo que se ve durante un vuelo: la ventana propia y las tiras vivas, puestas
//! donde estaba la ventana y giradas según el borde del dock.

use dokka_animation::{GENIE_OVERLAP_ROWS, GENIE_STRIPS, GenieEdges};
use dokka_win_thumbnail::{LiveImage, LiveTarget, source_size};
use windows::UI::Composition::{ContainerVisual, Visual};
use windows::Win32::Foundation::HWND;
use windows::core::{Error, Interface, Result};
use windows_numerics::{Matrix4x4, Vector2};

use crate::geometry::{PixelRect, Turn, to_local};
use crate::overlay::{create_window, destroy};
use crate::placement::Placement;
use crate::strips::{GenieCompositor, Piece};

pub(crate) struct Stage {
    pub(crate) overlay: HWND,
    pub(crate) root: ContainerVisual,
    _target: Option<LiveTarget>,
    pub(crate) pieces: Vec<Piece>,
}

impl Stage {
    /// Prepara el vuelo de `hwnd` con las tiras quietas donde estaba. La
    /// ventana propia queda oculta: la enseña quien llama.
    pub(crate) fn new(
        gfx: &GenieCompositor,
        hwnd: isize,
        (whole, frame): (PixelRect, PixelRect),
        screen: PixelRect,
        turn: Turn,
    ) -> Result<(Self, GenieEdges)> {
        let overlay = create_window(screen).ok_or_else(Error::from_thread)?;
        // Desde aquí `Drop` limpia si algo falla.
        let mut stage = Self {
            overlay,
            root: gfx.compositor.CreateContainerVisual()?,
            _target: None,
            pieces: Vec::new(),
        };
        // Tamaño fijo: este destino no tiene padre del que tomarlo y, sin él, el
        // recorte del hueco puede taparlo todo.
        stage.root.SetSize(Vector2 {
            X: screen.width() as f32,
            Y: screen.height() as f32,
        })?;
        let host = overlay.0 as isize;
        let target = gfx.live.target_for(host)?;
        target.target().SetRoot(&stage.root)?;
        stage._target = Some(target);
        turn.apply(&stage.root)?;
        let source = source_size(host, hwnd)?;
        let placement =
            Placement::new(whole, frame, source, screen).ok_or_else(Error::from_thread)?;
        let window = turn.edges(to_local(placement.visible, screen));
        let height = (window.bottom - window.top) as i32;
        let count = height.min(GENIE_STRIPS);
        for index in 0..count {
            let top = index * height / count;
            let rows = (index + 1) * height / count - top;
            // Una fila de más tapa la costura con la tira siguiente.
            let extent = (rows + GENIE_OVERLAP_ROWS).min(height - top);
            let area = placement.strip(turn, top, extent);
            let image = LiveImage::new(
                &gfx.live,
                host,
                hwnd,
                placement.in_source(area),
                (area[2], area[3]),
            )?;
            let visual = upright(gfx, &image, turn, area)?;
            visual.SetTransformMatrix(Matrix4x4::translation(
                window.left,
                window.top + top as f32,
                0.0,
            ))?;
            stage.root.Children()?.InsertAtTop(&visual)?;
            stage.pieces.push(Piece {
                visual,
                top: top as f32,
                rows: rows as f32,
                _image: image,
            });
        }
        Ok((stage, window))
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        // Las miniaturas se dan de baja antes de perder su ventana.
        self.pieces.clear();
        destroy(self.overlay);
    }
}

/// El visual que se deformará, ya en el marco del vuelo. Con el dock a un lado
/// la franja se gira dentro de un contenedor.
fn upright(
    gfx: &GenieCompositor,
    image: &LiveImage,
    turn: Turn,
    [_, _, width, height]: [i32; 4],
) -> Result<Visual> {
    let Some(matrix) = turn.part_matrix(width as f32, height as f32) else {
        return Ok(image.visual().clone());
    };
    image.visual().SetTransformMatrix(matrix)?;
    let holder = gfx.compositor.CreateContainerVisual()?;
    holder.Children()?.InsertAtTop(image.visual())?;
    holder.cast()
}
