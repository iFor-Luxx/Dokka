use dokka_scene::{Color, Label, Text, TextAlign};
use windows::UI::Composition::{
    CompositionRoundedRectangleGeometry, CompositionSpriteShape, Compositor, ContainerVisual,
    ShapeVisual, SpriteVisual,
};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::label_pointer::LabelPointer;
use crate::painter::Painter;
use crate::windows_color::windows_color;

/// Etiqueta flotante: píldora de cristal con punta redondeada,
/// como el panel (desenfoque + tinte + borde). La punta vive en
/// `LabelPointer` y es de la misma pieza, sin costura con la píldora.
/// El radio de la píldora es la mitad de su alto. Las capas se reutilizan
/// al moverse entre iconos: solo se repinta el texto si cambia.
pub(crate) struct LabelLayer {
    glass: SpriteVisual,
    glass_outline: CompositionRoundedRectangleGeometry,
    tint: ShapeVisual,
    tint_shape: CompositionSpriteShape,
    tint_outline: CompositionRoundedRectangleGeometry,
    pointer: LabelPointer,
    text_visual: SpriteVisual,
    chrome: Option<LabelChromeStyle>,
    text_style: Option<LabelTextStyle>,
    text_size: Vector2,
}

/// Lo que fija el aspecto al crearse: `update` no lo reconstruye.
#[derive(Debug, Clone, Copy, PartialEq)]
struct LabelChromeStyle {
    tint: Color,
    border: Color,
    border_width_bits: u32,
    pointer_height_bits: u32,
    pointer_radius_bits: u32,
}

impl LabelChromeStyle {
    fn of(label: &Label) -> Self {
        Self {
            tint: label.tint,
            border: label.border,
            border_width_bits: label.border_width.to_bits(),
            pointer_height_bits: label.pointer_height.to_bits(),
            pointer_radius_bits: label.pointer_radius.to_bits(),
        }
    }
}

/// Estilo que obliga a repintar el texto: contenido, tamaño y color.
#[derive(Debug, Clone, PartialEq)]
struct LabelTextStyle {
    content: String,
    size_bits: u32,
    color: Color,
}

impl LabelTextStyle {
    fn of(label: &Label) -> Self {
        Self {
            content: label.text.clone(),
            size_bits: label.font_size.to_bits(),
            color: label.text_color,
        }
    }
}

impl LabelLayer {
    pub(crate) fn add(compositor: &Compositor, parent: &ContainerVisual) -> Result<Self> {
        // Cada capa se inserta arriba: el orden de creación es el de pintado.
        let glass_outline = compositor.CreateRoundedRectangleGeometry()?;
        let glass = compositor.CreateSpriteVisual()?;
        glass.SetBrush(&compositor.CreateHostBackdropBrush()?)?;
        glass.SetClip(&compositor.CreateGeometricClipWithGeometry(&glass_outline)?)?;

        let tint_outline = compositor.CreateRoundedRectangleGeometry()?;
        let tint_shape = compositor.CreateSpriteShapeWithGeometry(&tint_outline)?;
        let tint = compositor.CreateShapeVisual()?;
        tint.Shapes()?.Append(&tint_shape)?;

        let children = parent.Children()?;
        children.InsertAtTop(&glass)?;
        children.InsertAtTop(&tint)?;
        let pointer = LabelPointer::add(compositor, parent)?;
        let text_visual = compositor.CreateSpriteVisual()?;
        children.InsertAtTop(&text_visual)?;
        Ok(Self {
            glass,
            glass_outline,
            tint,
            tint_shape,
            tint_outline,
            pointer,
            text_visual,
            chrome: None,
            text_style: None,
            text_size: Vector2::default(),
        })
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        label: &Label,
    ) -> Result<()> {
        self.set_visible(true)?;
        let (text_width, text_height) = painter.measure_label(label)?;
        let width = (text_width + label.padding_x * 2.0).ceil();
        let body = (text_height + label.padding_y * 2.0).ceil();
        let body_bottom = label.bottom - label.pointer_height;
        let body_top = body_bottom - body;
        // No dejar que se corte por el borde izquierdo.
        let left = (label.center_x - width / 2.0).max(0.0);
        let center_x = left + width / 2.0;
        let middle_y = body_top + body / 2.0;

        // Píldora: radio igual a la mitad del alto.
        let radius = body / 2.0;
        let inset = label.border_width / 2.0;
        let size = Vector2 { X: width, Y: body };
        let offset = Vector3 {
            X: left,
            Y: body_top,
            Z: 0.0,
        };
        self.glass.SetOffset(offset)?;
        self.glass.SetSize(size)?;
        self.glass_outline.SetSize(size)?;
        self.glass_outline.SetCornerRadius(radius_pair(radius))?;
        self.tint.SetOffset(offset)?;
        self.tint.SetSize(size)?;
        // El trazo se centra en el contorno: se mete medio grosor.
        self.tint_outline.SetSize(Vector2 {
            X: width - label.border_width,
            Y: body - label.border_width,
        })?;
        self.tint_outline
            .SetOffset(Vector2 { X: inset, Y: inset })?;
        self.tint_outline
            .SetCornerRadius(radius_pair(radius - inset))?;

        self.pointer.update(label, center_x, body_bottom)?;
        self.update_chrome(compositor, label)?;
        self.update_text(compositor, painter, label, center_x, middle_y)
    }

    fn update_chrome(&mut self, compositor: &Compositor, label: &Label) -> Result<()> {
        let chrome = LabelChromeStyle::of(label);
        if self.chrome != Some(chrome) {
            // Solo cuando cambia el estilo, no en cada movimiento.
            let fill = compositor.CreateColorBrushWithColor(windows_color(label.tint))?;
            let stroke = compositor.CreateColorBrushWithColor(windows_color(label.border))?;
            self.tint_shape.SetFillBrush(&fill)?;
            self.tint_shape.SetStrokeBrush(&stroke)?;
            self.tint_shape.SetStrokeThickness(label.border_width)?;
            self.pointer
                .set_chrome(&fill, &stroke, label.border_width)?;
            self.chrome = Some(chrome);
        }
        Ok(())
    }

    fn update_text(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        label: &Label,
        center_x: f32,
        middle_y: f32,
    ) -> Result<()> {
        let style = LabelTextStyle::of(label);
        if self.text_style.as_ref() != Some(&style) {
            let text = Text {
                x: center_x,
                center_y: middle_y,
                content: label.text.clone(),
                size: label.font_size,
                is_bold: false,
                color: label.text_color,
                align: TextAlign::Center,
            };
            let (surface, size) = painter.paint_text(&text)?;
            self.text_visual
                .SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
            self.text_visual.SetSize(size)?;
            self.text_size = size;
            self.text_style = Some(style);
        }
        self.text_visual.SetOffset(Vector3 {
            X: center_x - self.text_size.X / 2.0,
            Y: middle_y - self.text_size.Y / 2.0,
            Z: 0.0,
        })?;
        Ok(())
    }

    /// Oculta o muestra sin destruir la capa, conservando el orden de pintado.
    pub(crate) fn set_visible(&self, visible: bool) -> Result<()> {
        self.glass.SetIsVisible(visible)?;
        self.tint.SetIsVisible(visible)?;
        self.text_visual.SetIsVisible(visible)?;
        self.pointer.set_visible(visible)
    }
}

fn radius_pair(radius: f32) -> Vector2 {
    Vector2 {
        X: radius,
        Y: radius,
    }
}
