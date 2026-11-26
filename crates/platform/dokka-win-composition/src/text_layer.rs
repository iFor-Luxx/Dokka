use dokka_scene::{Text, TextAlign};
use windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::painter::Painter;

/// Estilo que obliga a repintar el texto: contenido, tamaño, color y alineación.
#[derive(Debug, Clone, PartialEq)]
struct TextStyle {
    content: String,
    size_bits: u32,
    is_bold: bool,
    color: dokka_scene::Color,
    align: TextAlign,
}

impl TextStyle {
    fn of(text: &Text) -> Self {
        Self {
            content: text.content.clone(),
            size_bits: text.size.to_bits(),
            is_bold: text.is_bold,
            color: text.color,
            align: text.align,
        }
    }
}

/// Texto ya pintado; solo se repinta si cambia.
pub(crate) struct TextLayer {
    visual: SpriteVisual,
    style: Option<TextStyle>,
    size: Vector2,
}

impl TextLayer {
    pub(crate) fn add(compositor: &Compositor, parent: &ContainerVisual) -> Result<Self> {
        let visual = compositor.CreateSpriteVisual()?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self {
            visual,
            style: None,
            size: Vector2::default(),
        })
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        text: &Text,
    ) -> Result<()> {
        self.set_visible(true)?;
        if self.style.as_ref() != Some(&TextStyle::of(text)) {
            let (surface, size) = painter.paint_text(text)?;
            self.visual
                .SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
            self.visual.SetSize(size)?;
            self.size = size;
            self.style = Some(TextStyle::of(text));
        }
        let left = match text.align {
            TextAlign::Center => text.x - self.size.X / 2.0,
            TextAlign::Left => text.x,
        };
        self.visual.SetOffset(Vector3 {
            X: left,
            Y: text.center_y - self.size.Y / 2.0,
            Z: 0.0,
        })?;
        Ok(())
    }

    /// Oculta o muestra sin destruir la capa, conservando el orden de pintado.
    pub(crate) fn set_visible(&self, visible: bool) -> Result<()> {
        self.visual.SetIsVisible(visible)
    }
}
