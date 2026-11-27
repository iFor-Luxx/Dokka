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

//! Tiras del genio. Cada una es una miniatura viva de su franja de la ventana,
//! deformada en perspectiva para que sus bordes sigan la curva del embudo. El
//! compositor las mueve solo; con pocas tiras va a la tasa del monitor.

use dokka_animation::Genie;
use dokka_win_thumbnail::{LiveCompositor, LiveImage};
use windows::UI::Composition::{
    CompositionPropertySet, Compositor, ContainerVisual, ExpressionAnimation, Visual,
};
use windows::core::{HSTRING, Result, h};

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

/// Una tira: el visual que se deforma, su primera fila y cuántas cubre.
pub(crate) struct Piece {
    pub(crate) visual: Visual,
    pub(crate) top: f32,
    pub(crate) rows: f32,
    pub(crate) _image: LiveImage,
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
    // Cuánto se dobló la altura `y` de pantalla.
    let curve = |y: &str| {
        format!(
            "((1 - Cos(3.14159265 * Clamp(({y} - ({top:.3})) / {span:.3}, 0, 1))) * 0.5 * d.bend)",
            top = window.top,
            span = (slot.top - window.top).max(1.0),
        )
    };
    let width = window.right - window.left;
    for piece in pieces {
        let y0 = window.top + piece.top;
        let above = format!("(({y0:.3}) + d.shift)");
        let below = format!(
            "(Min({:.3}, {:.3} - d.shift) + d.shift)",
            y0 + piece.rows,
            slot.bottom
        );
        // Windows limita el largo de cada fórmula: el doblez de arriba y el de
        // abajo se calculan aparte y la matriz solo los lee.
        let bent = compositor.CreatePropertySet()?;
        for (name, formula) in [(h!("top"), curve(&above)), (h!("low"), curve(&below))] {
            bent.InsertScalar(name, 0.0)?;
            let animation = expression(compositor, &formula, &drive)?;
            bent.StartAnimation(name, &animation)?;
        }
        let left = |c: &str| {
            format!(
                "(({:.3}) + ({:.3}) * {c})",
                window.left,
                slot.left - window.left
            )
        };
        let wide = |c: &str| {
            format!(
                "({width:.3} + ({:.3}) * {c})",
                (slot.right - slot.left) - width
            )
        };
        let (l0, l1, w0, w1) = (left("s.top"), left("s.low"), wide("s.top"), wide("s.low"));
        let k = format!("({w0} / {w1})");
        let rows = piece.rows;
        // Trapecio exacto: un rectángulo visto en perspectiva.
        let matrix = format!(
            "Matrix4x4({w0} / {width:.3}, 0, 0, 0, \
             ({k} * {l1} - {l0}) / {rows}, Max({k} * {below} - {above}, 0) / {rows}, 0, \
             ({k} - 1) / {rows}, \
             0, 0, 1, 0, \
             {l0}, {above}, 0, 1)"
        );
        let animation = expression(compositor, &matrix, &drive)?;
        animation.SetReferenceParameter(h!("s"), &bent)?;
        piece
            .visual
            .StartAnimation(h!("TransformMatrix"), &animation)?;
    }
    Ok(drive)
}

fn expression(
    compositor: &Compositor,
    formula: &str,
    drive: &CompositionPropertySet,
) -> Result<ExpressionAnimation> {
    let animation = compositor.CreateExpressionAnimationWithExpression(&HSTRING::from(formula))?;
    animation.SetReferenceParameter(h!("d"), drive)?;
    Ok(animation)
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

/// Nada se dibuja bajo el hueco: las tiras desaparecen al entrar en él.
pub(crate) fn clip_at_slot(
    gfx: &GenieCompositor,
    root: &ContainerVisual,
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
