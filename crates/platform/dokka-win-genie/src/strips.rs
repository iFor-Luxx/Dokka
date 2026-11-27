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

//! Tiras de composicion: la imagen viva se corta en bandas finas que siguen el
//! embudo. El compositor mueve las tiras en su propio hilo a la tasa del
//! monitor; el hilo de interfaz solo avanza el estado y pide restaurar.

use dokka_animation::{GENIE_OVERLAP_ROWS, Genie, GenieEdges, strip_count};
use dokka_win_thumbnail::LiveCompositor;
use windows::UI::Composition::{
    CompositionPropertySet, CompositionStretch, CompositionVisualSurface, Compositor,
    ContainerVisual, SpriteVisual,
};
use windows::core::{HSTRING, Result, h};
use windows_numerics::{Matrix3x2, Vector2, Vector3};

use crate::geometry::Turn;
use crate::overlay::flight_duration;

/// Compositor propio de los vuelos, capaz de mostrar imágenes vivas.
pub(crate) struct GenieCompositor {
    pub(crate) live: LiveCompositor,
    pub(crate) compositor: Compositor,
}

impl GenieCompositor {
    pub(crate) fn new() -> Result<Self> {
        let live = LiveCompositor::new()?;
        let compositor = live.compositor().clone();
        Ok(Self { live, compositor })
    }
}

/// Una tira: su visual, su primera fila y cuántas cubre.
pub(crate) struct Piece {
    pub visual: SpriteVisual,
    pub top: f32,
    pub rows: f32,
}

/// Corta la imagen en tiras y las deja quietas sobre `window`.
pub(crate) fn cut(
    gfx: &GenieCompositor,
    root: &ContainerVisual,
    surface: &CompositionVisualSurface,
    window: &GenieEdges,
) -> Result<Vec<Piece>> {
    let width = window.right - window.left;
    let height = (window.bottom - window.top) as u32;
    let count = strip_count(height);
    // Filas enteras: las fracciones dejan costuras entre tiras.
    let row = |index: u32| (index as f32 * height as f32 / count as f32).floor();
    let compositor = &gfx.compositor;
    let children = root.Children()?;
    (0..count)
        .map(|index| {
            let (top, rows) = (row(index), row(index + 1) - row(index));
            let brush = compositor.CreateSurfaceBrushWithSurface(surface)?;
            brush.SetStretch(CompositionStretch::None)?;
            brush.SetHorizontalAlignmentRatio(0.0)?;
            brush.SetVerticalAlignmentRatio(0.0)?;
            brush.SetTransformMatrix(Matrix3x2::translation(0.0, -top))?;
            let visual = compositor.CreateSpriteVisual()?;
            visual.SetSize(Vector2 {
                X: width,
                Y: rows + GENIE_OVERLAP_ROWS,
            })?;
            visual.SetOffset(Vector3 {
                X: window.left,
                Y: window.top + top,
                Z: 0.0,
            })?;
            visual.SetBrush(&brush)?;
            children.InsertAtTop(&visual)?;
            Ok(Piece { visual, top, rows })
        })
        .collect()
}

/// Ata cada tira a dos valores compartidos (`bend`, `shift`) con expresiones
/// que doblan los bordes hacia el hueco y deslizan la imagen dentro.
pub(crate) fn bind(
    gfx: &GenieCompositor,
    genie: &Genie,
    pieces: &[Piece],
    reverse: bool,
) -> Result<CompositionPropertySet> {
    let compositor = &gfx.compositor;
    let (window, slot) = (genie.window, genie.slot);
    let travel = (slot.bottom - window.top).max(0.0);
    let at = |progress: f32| if reverse { 1.0 - progress } else { progress };
    let drive = compositor.CreatePropertySet()?;
    drive.InsertScalar(h!("bend"), Genie::bend(at(0.0)))?;
    drive.InsertScalar(h!("shift"), Genie::slide(at(0.0)) * travel)?;
    let curve = format!(
        "((1 - Cos(3.14159265 * Clamp(((y0 + Min(y1, {bottom:.3} - d.shift)) * 0.5 + d.shift - ({top:.3})) / {span:.3}, 0, 1))) * 0.5 * d.bend)",
        bottom = slot.bottom,
        top = window.top,
        span = (slot.top - window.top).max(1.0),
    );
    let offset = format!(
        "Vector3(({left:.3}) + ({shrink:.3}) * {curve}, y0 + d.shift, 0)",
        left = window.left,
        shrink = slot.left - window.left,
    );
    let scale = format!(
        "Vector3((({width:.3}) + ({narrow:.3}) * {curve}) / {source:.3}, Max(Min(y1, {bottom:.3} - d.shift) - y0, 0) / rows, 1)",
        width = window.right - window.left,
        narrow = (slot.right - slot.left) - (window.right - window.left),
        source = window.right - window.left,
        bottom = slot.bottom,
    );
    let offset = compositor.CreateExpressionAnimationWithExpression(&HSTRING::from(offset))?;
    let scale = compositor.CreateExpressionAnimationWithExpression(&HSTRING::from(scale))?;
    offset.SetReferenceParameter(h!("d"), &drive)?;
    scale.SetReferenceParameter(h!("d"), &drive)?;
    for piece in pieces {
        let y0 = window.top + piece.top;
        let y1 = y0 + piece.rows;
        offset.SetScalarParameter(h!("y0"), y0)?;
        offset.SetScalarParameter(h!("y1"), y1)?;
        scale.SetScalarParameter(h!("y0"), y0)?;
        scale.SetScalarParameter(h!("y1"), y1)?;
        scale.SetScalarParameter(h!("rows"), piece.rows)?;
        piece.visual.StartAnimation(h!("Offset"), &offset)?;
        piece.visual.StartAnimation(h!("Scale"), &scale)?;
    }
    Ok(drive)
}

/// Arranca el vuelo: el compositor anima `bend` y `shift` a la tasa del monitor.
pub(crate) fn launch(
    gfx: &GenieCompositor,
    genie: &Genie,
    drive: &CompositionPropertySet,
    reverse: bool,
    seconds: f32,
) -> Result<()> {
    let compositor = &gfx.compositor;
    let travel = (genie.slot.bottom - genie.window.top).max(0.0);
    let at = |progress: f32| if reverse { 1.0 - progress } else { progress };
    let linear = compositor.CreateLinearEasingFunction()?;
    let duration = flight_duration(seconds);
    let animate = |name: &HSTRING, curve: fn(f32) -> f32, amount: f32| -> Result<()> {
        let animation = compositor.CreateScalarKeyFrameAnimation()?;
        animation.SetDuration(duration)?;
        for step in 0..=30 {
            let progress = step as f32 / 30.0;
            animation.InsertKeyFrameWithEasingFunction(
                progress,
                curve(at(progress)) * amount,
                &linear,
            )?;
        }
        drive.StartAnimation(name, &animation)?;
        Ok(())
    };
    animate(h!("bend"), Genie::bend, 1.0)?;
    animate(h!("shift"), Genie::slide, travel)?;
    Ok(())
}

/// Nada se dibuja bajo el hueco: las tiras desaparecen al entrar en el.
pub(crate) fn clip_at_slot(
    gfx: &GenieCompositor,
    root: &windows::UI::Composition::ContainerVisual,
    genie: &Genie,
    turn: Turn,
) -> Result<()> {
    let below = turn.size().1 - genie.slot.bottom;
    let clip = gfx
        .compositor
        .CreateInsetClipWithInsets(0.0, 0.0, 0.0, below.max(0.0))?;
    root.SetClip(&clip)?;
    Ok(())
}
