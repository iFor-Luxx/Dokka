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

//! Tiras de composicion: la foto se corta en bandas finas que siguen el
//! embudo. El compositor mueve las tiras en su propio hilo a la tasa del
//! monitor; el hilo de interfaz solo avanza el estado y pide restaurar.

use dokka_animation::{GENIE_MAX_STRIPS, GENIE_OVERLAP_ROWS, GENIE_STRIP_ROWS, Genie};
use windows::Foundation::Size;
use windows::Graphics::DirectX::{DirectXAlphaMode, DirectXPixelFormat};
use windows::UI::Composition::{
    CompositionDrawingSurface, CompositionGraphicsDevice, CompositionPropertySet, Compositor,
    SpriteVisual,
};
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_SINGLETHREADED, D3D11_SDK_VERSION,
    D3D11CreateDevice, ID3D11Device,
};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::System::WinRT::Composition::{
    ICompositionDrawingSurfaceInterop, ICompositorInterop,
};
use windows::core::{HSTRING, Interface, Result, h};
use windows_numerics::{Matrix3x2, Vector2};

use crate::GenieImage;
use crate::geometry::Turn;
use crate::overlay::flight_duration;

/// Compositor propio del vuelo con su dispositivo grafico.
pub(crate) struct GenieCompositor {
    pub(crate) compositor: Compositor,
    graphics: CompositionGraphicsDevice,
    _d3d: ID3D11Device,
}

impl GenieCompositor {
    pub(crate) fn new() -> Result<Self> {
        let compositor = Compositor::new()?;
        // SAFETY: D3D solo escribe en su ranura de salida; el dispositivo se
        // envuelve al momento en una interfaz COM propia.
        let (d3d, graphics) = unsafe {
            let mut d3d = None;
            // WARP como el vuelo original: siempre disponible y sin perdidas.
            let _ = D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_WARP,
                Default::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_SINGLETHREADED,
                None,
                D3D11_SDK_VERSION,
                Some(&mut d3d),
                None,
                None,
            )
            .or_else(|_| {
                D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_HARDWARE,
                    Default::default(),
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_SINGLETHREADED,
                    None,
                    D3D11_SDK_VERSION,
                    Some(&mut d3d),
                    None,
                    None,
                )
            });
            let d3d: ID3D11Device = d3d.ok_or_else(windows::core::Error::from_thread)?;
            let dxgi: IDXGIDevice = d3d.cast()?;
            let d2d = windows::Win32::Graphics::Direct2D::D2D1CreateDevice(&dxgi, None)?;
            let graphics = compositor
                .cast::<ICompositorInterop>()?
                .CreateGraphicsDevice(&d2d)?;
            (d3d, graphics)
        };
        Ok(Self {
            compositor,
            graphics,
            _d3d: d3d,
        })
    }

    /// Superficie con la imagen a su propio tamano para compartirla.
    pub(crate) fn image_surface(&self, image: &GenieImage) -> Result<CompositionDrawingSurface> {
        let surface = self.graphics.CreateDrawingSurface(
            Size {
                Width: image.width as f32,
                Height: image.height as f32,
            },
            DirectXPixelFormat::B8G8R8A8UIntNormalized,
            DirectXAlphaMode::Premultiplied,
        )?;
        let interop: ICompositionDrawingSurfaceInterop = surface.cast()?;
        // SAFETY: cada `BeginDraw` se cierra con `EndDraw`, tambien si falla.
        unsafe {
            let mut offset = POINT::default();
            let context: windows::Win32::Graphics::Direct2D::ID2D1DeviceContext =
                interop.BeginDraw(None, &mut offset)?;
            let painted = crate::paint::paint_image(&context, offset, image);
            interop.EndDraw()?;
            painted?;
        }
        Ok(surface)
    }
}

/// Una tira: su visual y su primera fila en pixeles de la foto.
pub(crate) struct Piece {
    pub visual: SpriteVisual,
    pub top: f32,
    pub rows: f32,
}

/// Corta la imagen en tiras de la altura de la ventana en pantalla.
pub(crate) fn cut(
    gfx: &GenieCompositor,
    root: &windows::UI::Composition::ContainerVisual,
    image: &GenieImage,
    window_height: u32,
) -> Result<Vec<Piece>> {
    let surface = gfx.image_surface(image)?;
    let count = (window_height / GENIE_STRIP_ROWS).clamp(1, GENIE_MAX_STRIPS);
    let exact = image.height == window_height;
    let row = |index: u32| {
        let row = index as f32 * image.height as f32 / count as f32;
        if exact { row.floor() } else { row }
    };
    let compositor = &gfx.compositor;
    let children = root.Children()?;
    (0..count)
        .map(|index| {
            let (top, rows) = (row(index), row(index + 1) - row(index));
            let brush = compositor.CreateSurfaceBrushWithSurface(&surface)?;
            brush.SetStretch(windows::UI::Composition::CompositionStretch::None)?;
            brush.SetHorizontalAlignmentRatio(0.0)?;
            brush.SetVerticalAlignmentRatio(0.0)?;
            brush.SetTransformMatrix(Matrix3x2::translation(0.0, -top))?;
            let visual = compositor.CreateSpriteVisual()?;
            visual.SetSize(Vector2 {
                X: image.width as f32,
                Y: rows + GENIE_OVERLAP_ROWS,
            })?;
            visual.SetBrush(&brush)?;
            children.InsertAtTop(&visual)?;
            Ok(Piece { visual, top, rows })
        })
        .collect()
}

/// Ata cada tira a dos valores compartidos (`bend`, `shift`) con expresiones
/// que reescriben `Genie::frame` para el compositor.
pub(crate) fn bind(
    gfx: &GenieCompositor,
    genie: &Genie,
    pieces: &[Piece],
    image: (f32, f32),
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
        source = image.0,
        bottom = slot.bottom,
    );
    let offset = compositor.CreateExpressionAnimationWithExpression(&HSTRING::from(offset))?;
    let scale = compositor.CreateExpressionAnimationWithExpression(&HSTRING::from(scale))?;
    offset.SetReferenceParameter(h!("d"), &drive)?;
    scale.SetReferenceParameter(h!("d"), &drive)?;
    let stretch = (window.bottom - window.top) / image.1;
    for piece in pieces {
        let y0 = window.top + piece.top * stretch;
        let y1 = y0 + piece.rows * stretch;
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
