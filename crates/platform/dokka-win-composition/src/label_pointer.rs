use std::f32::consts::SQRT_2;

use dokka_scene::Label;
use windows::UI::Composition::{
    CompositionColorBrush, CompositionRoundedRectangleGeometry, CompositionSpriteShape, Compositor,
    ContainerVisual, ShapeVisual, SpriteVisual, Visual,
};
use windows::core::{Interface, Result};
use windows_numerics::{Vector2, Vector3};

/// Punta de cristal bajo la píldora de la etiqueta, de la misma pieza:
/// un cuadrado girado 45° y recortado para que solo se vea la mitad
/// inferior, que forma el triángulo de vértice redondeado.
/// Tapa la franja del borde inferior de la píldora para que no se
/// acumulen dos tintes ni se vea línea entre ambas piezas.
pub(crate) struct LabelPointer {
    container: ContainerVisual,
    clip: CompositionRoundedRectangleGeometry,
    glass: SpriteVisual,
    tint: ShapeVisual,
    shape: CompositionSpriteShape,
    outline: CompositionRoundedRectangleGeometry,
}

impl LabelPointer {
    pub(crate) fn add(compositor: &Compositor, parent: &ContainerVisual) -> Result<Self> {
        let clip = compositor.CreateRoundedRectangleGeometry()?;
        let container = compositor.CreateContainerVisual()?;
        container.SetClip(&compositor.CreateGeometricClipWithGeometry(&clip)?)?;
        let glass = compositor.CreateSpriteVisual()?;
        glass.SetBrush(&compositor.CreateHostBackdropBrush()?)?;
        let outline = compositor.CreateRoundedRectangleGeometry()?;
        let shape = compositor.CreateSpriteShapeWithGeometry(&outline)?;
        let tint = compositor.CreateShapeVisual()?;
        tint.Shapes()?.Append(&shape)?;
        parent.Children()?.InsertAtTop(&container)?;
        container.Children()?.InsertAtTop(&glass)?;
        container.Children()?.InsertAtTop(&tint)?;
        Ok(Self {
            container,
            clip,
            glass,
            tint,
            shape,
            outline,
        })
    }

    /// Mueve y redimensiona la punta sin reconstruir capas.
    pub(crate) fn update(&mut self, label: &Label, center_x: f32, body_bottom: f32) -> Result<()> {
        let height = label.pointer_height;
        let border = label.border_width;
        // Ancho visible en el borde = 2 × altura; se deja margen para el trazo.
        let container_width = height * 2.0 + border * 2.0;
        let container_height = height + border * 2.0;
        self.container.SetOffset(Vector3 {
            X: center_x - height - border,
            Y: body_bottom - border,
            Z: 0.0,
        })?;
        self.container.SetSize(Vector2 {
            X: container_width,
            Y: container_height,
        })?;
        // El recorte no deforma el triángulo: es el rectángulo del contenedor.
        self.clip.SetSize(Vector2 {
            X: container_width,
            Y: container_height,
        })?;
        self.clip.SetCornerRadius(Vector2 { X: 0.0, Y: 0.0 })?;

        // Cuadrado de lado `s`: al girarlo 45°, la diagonal mide 2 × altura.
        let side = height * SQRT_2;
        let square = Vector2 { X: side, Y: side };
        // El centro del cuadrado queda sobre el borde inferior de la píldora.
        let square_offset = Vector3 {
            X: center_x - side / 2.0 - (center_x - height - border),
            Y: body_bottom - side / 2.0 - (body_bottom - border),
            Z: 0.0,
        };
        self.glass.SetSize(square)?;
        self.glass.SetOffset(square_offset)?;
        diamond(&self.glass.cast()?, side)?;
        // El tinte lleva el mismo tamaño que el cristal: sin tamaño no pinta.
        self.tint.SetSize(square)?;
        self.tint.SetOffset(square_offset)?;
        diamond(&self.tint.cast()?, side)?;
        self.outline.SetSize(Vector2 {
            X: side - border,
            Y: side - border,
        })?;
        self.outline.SetOffset(Vector2 {
            X: border / 2.0,
            Y: border / 2.0,
        })?;
        // Vértice redondeado: el menú sigue con radio cero.
        self.outline.SetCornerRadius(Vector2 {
            X: label.pointer_radius,
            Y: label.pointer_radius,
        })
    }

    /// Tinte y borde de la punta.
    pub(crate) fn set_chrome(
        &self,
        fill: &CompositionColorBrush,
        stroke: &CompositionColorBrush,
        width: f32,
    ) -> Result<()> {
        self.shape.SetFillBrush(fill)?;
        self.shape.SetStrokeBrush(stroke)?;
        self.shape.SetStrokeThickness(width)
    }
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
