use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::Direct2D::{
    D2D1_DRAW_TEXT_OPTIONS_NONE, ID2D1DeviceContext, ID2D1SolidColorBrush,
};
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_MEASURING_MODE_NATURAL,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWriteCreateFactory,
    IDWriteFactory,
};
use windows::core::{Result, w};

/// Caja generosa alrededor del centro para que nada se parta en líneas.
const BOX_HALF_SIDE: f32 = 200.0;

/// Texto de una línea centrado en un punto.
#[derive(Debug, Clone, Copy)]
pub struct CenteredText<'a> {
    pub text: &'a str,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub is_bold: bool,
}

#[derive(Debug)]
pub struct TextPainter {
    factory: IDWriteFactory,
}

impl TextPainter {
    pub fn new() -> Result<Self> {
        // SAFETY: crea la fábrica compartida de DirectWrite.
        let factory = unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)? };
        Ok(Self { factory })
    }

    /// Debe llamarse entre el inicio y el fin del dibujo de `context`.
    pub fn draw(
        &self,
        context: &ID2D1DeviceContext,
        brush: &ID2D1SolidColorBrush,
        text: CenteredText<'_>,
    ) -> Result<()> {
        let weight = if text.is_bold {
            DWRITE_FONT_WEIGHT_SEMI_BOLD
        } else {
            DWRITE_FONT_WEIGHT_NORMAL
        };
        let characters: Vec<u16> = text.text.encode_utf16().collect();
        let area = D2D_RECT_F {
            left: text.x - BOX_HALF_SIDE,
            top: text.y - BOX_HALF_SIDE,
            right: text.x + BOX_HALF_SIDE,
            bottom: text.y + BOX_HALF_SIDE,
        };
        // SAFETY: las cadenas y el rectángulo viven durante las llamadas.
        unsafe {
            let format = self.factory.CreateTextFormat(
                w!("Segoe UI"),
                None,
                weight,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                text.size,
                w!("es-ES"),
            )?;
            format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)?;
            format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;
            context.DrawText(
                &characters,
                &format,
                &area,
                brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
        Ok(())
    }
}
