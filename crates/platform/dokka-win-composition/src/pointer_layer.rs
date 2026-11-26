use std::f32::consts::SQRT_2;

use dokka_scene::{Panel, Pointer};
use windows::UI::Composition::{Compositor, ContainerVisual, Visual};
use windows::core::{Interface, Result};
use windows_numerics::{Vector2, Vector3};

use crate::windows_color::windows_color;

/// Punta de cristal bajo el menú: un cuadrado girado 45° dentro de un
/// contenedor recortado para que solo se vea la mitad inferior, que forma
/// el triángulo. El recorte empieza `border_width` por encima del borde
/// inferior del panel y la capa se pinta encima del panel, de modo que su
/// cristal tape ese tramo de borde y de tinte y no se acumulen dos tintes.
/// Los dos lados inclinados llevan el mismo borde que el panel.
/// Ancho visible en el borde del panel = 2 × altura, vértice a `altura` por debajo.
pub(crate) struct PointerLayer {
    container: ContainerVisual,
}

impl PointerLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        panel: &Panel,
        pointer: &Pointer,
    ) -> Result<Self> {
        let container = compositor.CreateContainerVisual()?;
        // El cristal tapa la franja del borde inferior: el recorte incluye ese tramo.
        update_container(compositor, &container, panel, pointer)?;
        // Cuadrado de lado `s`: al girarlo 45°, la diagonal mide 2 × altura.
        let side = pointer.height * SQRT_2;
        let square = Vector2 { X: side, Y: side };
        let (glass_x, glass_y) = square_offset(panel, pointer, side);
        let glass = compositor.CreateSpriteVisual()?;
        glass.SetBrush(&compositor.CreateHostBackdropBrush()?)?;
        glass.SetSize(square)?;
        glass.SetOffset(Vector3 {
            X: glass_x,
            Y: glass_y,
            Z: 0.0,
        })?;
        diamond(&glass.cast()?, side)?;
        container.Children()?.InsertAtTop(&glass)?;

        // El trazo se centra en el contorno: se mete medio grosor para que no se corte.
        let border = panel.border_width;
        let inset = border / 2.0;
        let tint_outline = compositor.CreateRoundedRectangleGeometry()?;
        tint_outline.SetCornerRadius(Vector2 { X: 0.0, Y: 0.0 })?;
        tint_outline.SetSize(Vector2 {
            X: side - border,
            Y: side - border,
        })?;
        tint_outline.SetOffset(Vector2 { X: inset, Y: inset })?;
        let shape = compositor.CreateSpriteShapeWithGeometry(&tint_outline)?;
        shape.SetFillBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.tint))?)?;
        shape
            .SetStrokeBrush(&compositor.CreateColorBrushWithColor(windows_color(panel.border))?)?;
        shape.SetStrokeThickness(border)?;
        let tint = compositor.CreateShapeVisual()?;
        tint.SetSize(square)?;
        tint.SetOffset(Vector3 {
            X: glass_x,
            Y: glass_y,
            Z: 0.0,
        })?;
        diamond(&tint.cast()?, side)?;
        tint.Shapes()?.Append(&shape)?;
        container.Children()?.InsertAtTop(&tint)?;

        parent.Children()?.InsertAtTop(&container)?;
        Ok(Self { container })
    }

    /// Mueve la punta sin reconstruir capas.
    pub(crate) fn update(&self, panel: &Panel, pointer: &Pointer) -> Result<()> {
        let (x, y) = container_offset(panel, pointer);
        self.container.SetOffset(Vector3 { X: x, Y: y, Z: 0.0 })
    }
}

/// Estilo que obliga a recrear la punta: su altura (tinte y borde salen del panel).
pub(crate) fn pointer_style_of(pointer: Pointer) -> f32 {
    pointer.height
}

fn container_offset(panel: &Panel, pointer: &Pointer) -> (f32, f32) {
    let bottom = panel.bounds.y + panel.bounds.height;
    let border = panel.border_width;
    (pointer.center_x - pointer.height - border, bottom - border)
}

fn update_container(
    compositor: &Compositor,
    container: &ContainerVisual,
    panel: &Panel,
    pointer: &Pointer,
) -> Result<()> {
    let border = panel.border_width;
    // Ancho visible en el borde = 2 × altura; se deja margen para el trazo.
    let width = pointer.height * 2.0 + border * 2.0;
    // Franja del borde que se tapa más el triángulo y margen para el trazo.
    let height = pointer.height + border * 2.0;
    let (x, y) = container_offset(panel, pointer);
    container.SetOffset(Vector3 { X: x, Y: y, Z: 0.0 })?;
    container.SetSize(Vector2 {
        X: width,
        Y: height,
    })?;
    // El recorte no deforma el triángulo: es el rectángulo del contenedor.
    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetCornerRadius(Vector2 { X: 0.0, Y: 0.0 })?;
    outline.SetSize(Vector2 {
        X: width,
        Y: height,
    })?;
    container.SetClip(&compositor.CreateGeometricClipWithGeometry(&outline)?)
}

fn square_offset(panel: &Panel, pointer: &Pointer, side: f32) -> (f32, f32) {
    let bottom = panel.bounds.y + panel.bounds.height;
    let (container_x, container_y) = container_offset(panel, pointer);
    // El centro del cuadrado queda sobre el borde inferior del panel.
    let square_x = pointer.center_x - side / 2.0;
    let square_y = bottom - side / 2.0;
    (square_x - container_x, square_y - container_y)
}

fn diamond(visual: &Visual, side: f32) -> Result<()> {
    visual.SetCenterPoint(Vector3 {
        X: side / 2.0,
        Y: side / 2.0,
        Z: 0.0,
    })?;
    visual.SetRotationAxis(Vector3 {
        X: 0.0,
        Y: 0.0,
        Z: 1.0,
    })?;
    visual.SetRotationAngleInDegrees(45.0)
}
