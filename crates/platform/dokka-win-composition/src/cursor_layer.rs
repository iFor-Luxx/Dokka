use dokka_scene::TextCursor;
use windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::painter::Painter;
use crate::windows_color::windows_color;

/// Cursor tras un texto: mide ESE texto con su mismo estilo y coloca ahí el
/// cursor, sin salirse del interior de la casilla. Solo mide cuando cambian
/// el contenido, el tamaño o el peso; la posición se recalcula siempre.
pub(crate) struct CursorLayer {
    visual: SpriteVisual,
    style: Option<CursorStyle>,
    measured: f32,
}

/// Lo que obliga a medir de nuevo y a cambiar el pincel.
#[derive(Debug, Clone, PartialEq)]
struct CursorStyle {
    content: String,
    size_bits: u32,
    is_bold: bool,
    color: dokka_scene::Color,
}

impl CursorStyle {
    fn of(cursor: &TextCursor) -> Self {
        Self {
            content: cursor.content.clone(),
            size_bits: cursor.size.to_bits(),
            is_bold: cursor.is_bold,
            color: cursor.color,
        }
    }
}

impl CursorLayer {
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        cursor: &TextCursor,
    ) -> Result<Self> {
        let visual = compositor.CreateSpriteVisual()?;
        visual.SetBrush(&compositor.CreateColorBrushWithColor(windows_color(cursor.color))?)?;
        parent.Children()?.InsertAtTop(&visual)?;
        Ok(Self {
            visual,
            style: None,
            measured: 0.0,
        })
    }

    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        cursor: &TextCursor,
    ) -> Result<()> {
        if self.style.as_ref() != Some(&CursorStyle::of(cursor)) {
            let needs_measure = match &self.style {
                None => true,
                Some(style) => {
                    style.content != cursor.content
                        || style.size_bits != cursor.size.to_bits()
                        || style.is_bold != cursor.is_bold
                }
            };
            if needs_measure {
                self.measured = painter.measure_cursor(cursor)?;
            }
            let needs_brush = match &self.style {
                None => true,
                Some(style) => style.color != cursor.color,
            };
            if needs_brush {
                self.visual.SetBrush(
                    &compositor.CreateColorBrushWithColor(windows_color(cursor.color))?,
                )?;
            }
            self.style = Some(CursorStyle::of(cursor));
        }
        let limit = (cursor.max_x - cursor.width).max(cursor.text_x);
        let x = (cursor.text_x + self.measured + cursor.gap)
            .min(limit)
            .max(cursor.text_x);
        self.visual.SetOffset(Vector3 {
            X: x,
            Y: cursor.center_y - cursor.height / 2.0,
            Z: 0.0,
        })?;
        self.visual.SetSize(Vector2 {
            X: cursor.width,
            Y: cursor.height,
        })?;
        self.set_visible(cursor.visible)
    }

    /// Oculta o muestra sin destruir la capa, conservando el orden de pintado.
    pub(crate) fn set_visible(&self, visible: bool) -> Result<()> {
        self.visual.SetIsVisible(visible)
    }
}
