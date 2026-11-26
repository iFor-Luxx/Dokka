use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::Direct2D::{
    D2D1_DRAW_TEXT_OPTIONS_NONE, ID2D1DeviceContext, ID2D1SolidColorBrush,
};
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_MEASURING_MODE_NATURAL,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
    DWRITE_TEXT_METRICS, DWriteCreateFactory, IDWriteFactory, IDWriteFactory3, IDWriteFactory5,
    IDWriteFontCollection, IDWriteInMemoryFontFileLoader, IDWriteTextFormat,
};
use windows::core::{HSTRING, Interface, Result, w};

/// Caja generosa alrededor del centro para que nada se parta en líneas.
const BOX_HALF_SIDE: f32 = 600.0;

/// Familia de la fuente de iconos dentro del TTF de Lucide.
const ICON_FAMILY: &str = "lucide";

/// Texto de una línea colocado en un punto.
#[derive(Debug, Clone, Copy)]
pub struct CenteredText<'a> {
    pub text: &'a str,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub is_bold: bool,
    /// `true` alinea a la izquierda en `x`; `false` centra en `x`.
    pub align_left: bool,
}

impl<'a> CenteredText<'a> {
    /// Texto centrado en el punto, como antes.
    pub fn centered(text: &'a str, x: f32, y: f32, size: f32, is_bold: bool) -> Self {
        Self {
            text,
            x,
            y,
            size,
            is_bold,
            align_left: false,
        }
    }
}

#[derive(Debug)]
pub struct TextPainter {
    factory: IDWriteFactory,
    family: HSTRING,
    icon_family: HSTRING,
    icon_collection: IDWriteFontCollection,
    /// Mantiene viva la fuente en memoria mientras se usa.
    _icon_loader: IDWriteInMemoryFontFileLoader,
}

impl TextPainter {
    /// `family` es el nombre de la fuente; lo decide quien usa el pintor.
    /// `icon_bytes` son los bytes `&'static` de la fuente de iconos.
    pub fn new(family: &str, icon_bytes: &'static [u8]) -> Result<Self> {
        // SAFETY: crea la fábrica compartida de DirectWrite.
        let factory: IDWriteFactory = unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)? };
        let (loader, collection) = Self::load_icons(&factory, icon_bytes)?;
        Ok(Self {
            factory,
            family: HSTRING::from(family),
            icon_family: HSTRING::from(ICON_FAMILY),
            icon_collection: collection,
            _icon_loader: loader,
        })
    }

    /// Ancho y alto que ocupa el texto en una línea.
    pub fn measure(&self, text: &str, size: f32, is_bold: bool) -> Result<(f32, f32)> {
        let characters: Vec<u16> = text.encode_utf16().collect();
        let format = self.format(size, is_bold)?;
        let mut metrics = DWRITE_TEXT_METRICS::default();
        // SAFETY: las cadenas viven durante la llamada y `metrics` es escribible.
        unsafe {
            let layout = self.factory.CreateTextLayout(
                &characters,
                &format,
                BOX_HALF_SIDE * 2.0,
                BOX_HALF_SIDE,
            )?;
            layout.GetMetrics(&mut metrics)?;
        }
        Ok((metrics.widthIncludingTrailingWhitespace, metrics.height))
    }

    /// Debe llamarse entre el inicio y el fin del dibujo de `context`.
    pub fn draw(
        &self,
        context: &ID2D1DeviceContext,
        brush: &ID2D1SolidColorBrush,
        text: CenteredText<'_>,
    ) -> Result<()> {
        let characters: Vec<u16> = text.text.encode_utf16().collect();
        let format = self.format(text.size, text.is_bold)?;
        let area = if text.align_left {
            D2D_RECT_F {
                left: text.x,
                top: text.y - BOX_HALF_SIDE,
                right: text.x + BOX_HALF_SIDE * 2.0,
                bottom: text.y + BOX_HALF_SIDE,
            }
        } else {
            D2D_RECT_F {
                left: text.x - BOX_HALF_SIDE,
                top: text.y - BOX_HALF_SIDE,
                right: text.x + BOX_HALF_SIDE,
                bottom: text.y + BOX_HALF_SIDE,
            }
        };
        // SAFETY: las cadenas y el rectángulo viven durante las llamadas.
        unsafe {
            let align = if text.align_left {
                DWRITE_TEXT_ALIGNMENT_LEADING
            } else {
                DWRITE_TEXT_ALIGNMENT_CENTER
            };
            format.SetTextAlignment(align)?;
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

    /// Dibuja un glifo de la fuente de iconos, centrado en (`x`, `y`).
    /// Debe llamarse entre el inicio y el fin del dibujo de `context`.
    pub fn draw_glyph(
        &self,
        context: &ID2D1DeviceContext,
        brush: &ID2D1SolidColorBrush,
        glyph: char,
        x: f32,
        y: f32,
        size: f32,
    ) -> Result<()> {
        let mut encoded = [0u16; 2];
        let characters = glyph.encode_utf16(&mut encoded);
        let format = self.icon_format(size)?;
        let area = D2D_RECT_F {
            left: x - BOX_HALF_SIDE,
            top: y - BOX_HALF_SIDE,
            right: x + BOX_HALF_SIDE,
            bottom: y + BOX_HALF_SIDE,
        };
        // SAFETY: el formato, las cadenas y el rectángulo viven durante las llamadas.
        unsafe {
            format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)?;
            format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;
            context.DrawText(
                characters,
                &format,
                &area,
                brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
        Ok(())
    }

    /// Carga la fuente de iconos en memoria una sola vez y devuelve
    /// el cargador (que debe seguir vivo) y su colección propia.
    /// Requiere Windows 10 1703+ (`IDWriteFactory5`).
    fn load_icons(
        factory: &IDWriteFactory,
        bytes: &'static [u8],
    ) -> Result<(IDWriteInMemoryFontFileLoader, IDWriteFontCollection)> {
        // SAFETY: `factory` es la fábrica compartida y los objetos COM
        // viven en este hilo; `bytes` es `&'static` y sigue vivo.
        unsafe {
            let factory5: IDWriteFactory5 = factory.cast()?;
            let loader = factory5.CreateInMemoryFontFileLoader()?;
            factory.RegisterFontFileLoader(&loader)?;
            let font_file = loader.CreateInMemoryFontFileReference(
                factory,
                bytes.as_ptr().cast(),
                bytes.len() as u32,
                None,
            )?;
            let builder = factory5.CreateFontSetBuilder()?;
            builder.AddFontFile(&font_file)?;
            let font_set = builder.CreateFontSet()?;
            let factory3: IDWriteFactory3 = factory.cast()?;
            let collection = factory3.CreateFontCollectionFromFontSet(&font_set)?;
            let icons: IDWriteFontCollection = collection.cast()?;
            Ok((loader, icons))
        }
    }

    fn format(&self, size: f32, is_bold: bool) -> Result<IDWriteTextFormat> {
        let weight = if is_bold {
            DWRITE_FONT_WEIGHT_SEMI_BOLD
        } else {
            DWRITE_FONT_WEIGHT_NORMAL
        };
        // SAFETY: las cadenas son estáticas.
        unsafe {
            self.factory.CreateTextFormat(
                &self.family,
                None,
                weight,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                size,
                w!("es-ES"),
            )
        }
    }

    fn icon_format(&self, size: f32) -> Result<IDWriteTextFormat> {
        // SAFETY: la familia y la colección viven con el pintor.
        unsafe {
            self.factory.CreateTextFormat(
                &self.icon_family,
                Some(&self.icon_collection),
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                size,
                w!("es-ES"),
            )
        }
    }
}
