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

//! Lo que se ve durante un vuelo: la ventana propia, la imagen viva de la
//! ventana (recortada y girada según el dock) y sus tiras.

use dokka_animation::GenieEdges;
use dokka_win_thumbnail::{LiveImage, LiveTarget};
use windows::UI::Composition::{CompositionVisualSurface, ContainerVisual};
use windows::Win32::Foundation::HWND;
use windows::core::{Interface, Result};
use windows_numerics::Vector2;

use crate::geometry::{PixelRect, Turn, to_local};
use crate::overlay::{create_window, destroy};
use crate::strips::{GenieCompositor, Piece, cut};

pub(crate) struct Stage {
    pub(crate) overlay: HWND,
    pub(crate) root: ContainerVisual,
    _target: Option<LiveTarget>,
    image: Option<LiveImage>,
}

impl Stage {
    /// Prepara el vuelo de `hwnd`, con las tiras puestas donde estaba. La
    /// ventana propia queda oculta: la enseña quien llama.
    pub(crate) fn new(
        gfx: &GenieCompositor,
        hwnd: isize,
        (whole, visible): (PixelRect, PixelRect),
        screen: PixelRect,
        turn: Turn,
    ) -> Result<(Self, GenieEdges, Vec<Piece>)> {
        let overlay = create_window(screen).ok_or_else(windows::core::Error::from_thread)?;
        // Desde aquí `Drop` limpia si algo falla.
        let mut stage = Self {
            overlay,
            root: gfx.compositor.CreateContainerVisual()?,
            _target: None,
            image: None,
        };
        stage
            .root
            .SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;
        let target = gfx.live.target_for(overlay.0 as isize)?;
        target.target().SetRoot(&stage.root)?;
        stage._target = Some(target);
        turn.apply(&stage.root)?;
        let image = LiveImage::new(
            &gfx.live,
            overlay.0 as isize,
            hwnd,
            (whole.width(), whole.height()),
        )?;
        let window = turn.edges(to_local(visible, screen));
        let surface = surface_of(gfx, &image, whole, visible, turn)?;
        stage.image = Some(image);
        let pieces = cut(gfx, &stage.root, &surface, &window)?;
        Ok((stage, window, pieces))
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        // La miniatura se da de baja antes de perder su ventana.
        self.image = None;
        destroy(self.overlay);
    }
}

/// Superficie compartida por todas las tiras: la parte visible de la imagen
/// viva, ya girada como el marco del vuelo.
fn surface_of(
    gfx: &GenieCompositor,
    image: &LiveImage,
    whole: PixelRect,
    visible: PixelRect,
    turn: Turn,
) -> Result<CompositionVisualSurface> {
    let (width, height) = (visible.width() as f32, visible.height() as f32);
    let size = if turn.edge.is_vertical() {
        Vector2 {
            X: height,
            Y: width,
        }
    } else {
        Vector2 {
            X: width,
            Y: height,
        }
    };
    let compositor = &gfx.compositor;
    let crop = compositor.CreateContainerVisual()?;
    crop.SetOffset(windows_numerics::Vector3 {
        X: (whole.left - visible.left) as f32,
        Y: (whole.top - visible.top) as f32,
        Z: 0.0,
    })?;
    crop.Children()?.InsertAtTop(image.visual())?;
    let turned = compositor.CreateContainerVisual()?;
    turned.SetTransformMatrix(turn.source_matrix(width, height))?;
    turned.Children()?.InsertAtTop(&crop)?;
    let frame = compositor.CreateContainerVisual()?;
    frame.SetSize(size)?;
    frame.Children()?.InsertAtTop(&turned)?;
    let surface = compositor.CreateVisualSurface()?;
    surface.SetSourceVisual(&frame.cast::<windows::UI::Composition::Visual>()?)?;
    surface.SetSourceSize(size)?;
    Ok(surface)
}
