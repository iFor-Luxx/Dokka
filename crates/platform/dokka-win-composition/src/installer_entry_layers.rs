//! Capas de la entrada inicial: fondo y montañas que entran desenfocadas.
//!
//! `fondo.jpg` queda abajo del todo; `izquierda.png` y `derecha.png` entran a
//! la vez como un solo bloque cada una, con desenfoque gaussiano animado de
//! `blur_max` a 0 (lineal) mientras viajan (frenada suave). `playa.jpg`
//! (el `photo` del dibujador) funde encima al llegar y el contenido sube en
//! bloque. Todo lo anima el compositor; en reposo no queda ningún efecto
//! activo: las capas se ocultan y se quitan del árbol. Si alguna imagen falla,
//! se devuelve `None` y la app muestra `playa.jpg` quieta.

use std::sync::Arc;

use dokka_scene::IconImage;
use windows::UI::Composition::{CompositionEffectBrush, Compositor, ContainerVisual, SpriteVisual};
use windows::core::{HSTRING, Result};
use windows_numerics::Vector3;

use crate::blur_effect::BLUR_ANIMATABLE;
use crate::installer_background::decode_background;
use crate::installer_entry::{EntryTiming, InstallerEntryData, cubic_ease_timing, span};
use crate::installer_entry_paint::{EntryRect, blurred_visual, still_visual};
use crate::painter::Painter;

/// Visuales de la entrada, aún sin colocar hasta el primer dibujo.
pub(crate) struct InstallerEntryLayers {
    fondo: SpriteVisual,
    izquierda: SpriteVisual,
    derecha: SpriteVisual,
    blur_left: CompositionEffectBrush,
    blur_right: CompositionEffectBrush,
    home_x: f32,
    home_y: f32,
    placed: bool,
}

impl InstallerEntryLayers {
    /// Decodifica fondo y montañas y crea sus visuales con el mismo rectángulo
    /// que la foto. `None` si alguna imagen no se puede leer o el efecto no
    /// se puede crear: la app sigue con la foto quieta.
    pub(crate) fn build(
        compositor: &Compositor,
        painter: &Painter,
        data: &InstallerEntryData<'_>,
        width: f32,
        height: f32,
        margin: f32,
        radius: f32,
    ) -> Result<Option<Self>> {
        if width <= 0.0 || height <= 0.0 {
            return Ok(None);
        }
        let fondo_bitmap = match decode_background(data.fondo) {
            Some(bitmap) => bitmap,
            None => return Ok(None),
        };
        let left_bitmap = match decode_background(data.izquierda) {
            Some(bitmap) => bitmap,
            None => return Ok(None),
        };
        let right_bitmap = match decode_background(data.derecha) {
            Some(bitmap) => bitmap,
            None => return Ok(None),
        };
        let fondo_brush = compositor.CreateSurfaceBrushWithSurface(
            &painter.paint(&IconImage::Pixels(Arc::new(fondo_bitmap)), width)?,
        )?;
        let fondo = compositor.CreateSpriteVisual()?;
        still_visual(
            compositor,
            &fondo,
            &fondo_brush,
            &EntryRect {
                width,
                height,
                x: margin,
                y: margin,
                radius,
            },
        )?;
        let (izquierda, blur_left) = blurred_visual(
            compositor,
            painter,
            left_bitmap,
            data.blur_max,
            &EntryRect {
                width,
                height,
                x: margin - data.travel,
                y: margin,
                radius,
            },
        )?;
        let (derecha, blur_right) = blurred_visual(
            compositor,
            painter,
            right_bitmap,
            data.blur_max,
            &EntryRect {
                width,
                height,
                x: margin + data.travel,
                y: margin,
                radius,
            },
        )?;
        Ok(Some(Self {
            fondo,
            izquierda,
            derecha,
            blur_left,
            blur_right,
            home_x: margin,
            home_y: margin,
            placed: false,
        }))
    }

    /// Cuelga fondo y montañas bajo la foto (cada `InsertAtBottom` queda
    /// debajo): fondo, izquierda, derecha y encima la foto.
    pub(crate) fn place(&mut self, parent: &ContainerVisual) -> Result<()> {
        if self.placed {
            return Ok(());
        }
        let children = parent.Children()?;
        children.InsertAtBottom(&self.derecha)?;
        children.InsertAtBottom(&self.izquierda)?;
        children.InsertAtBottom(&self.fondo)?;
        self.placed = true;
        Ok(())
    }

    /// Lanza el viaje, el desenfoque, el fundido de la foto y la subida del
    /// contenido en bloque. Todo con animaciones del compositor.
    pub(crate) fn start(
        &self,
        compositor: &Compositor,
        photo: &SpriteVisual,
        content: &ContainerVisual,
        timing: &EntryTiming,
    ) -> Result<()> {
        let ease = cubic_ease_timing(compositor, timing)?;
        start_travel(
            compositor,
            &self.izquierda,
            (self.home_x - timing.travel, self.home_y),
            (self.home_x, self.home_y),
            timing.mountains_seconds,
            &ease,
        )?;
        start_travel(
            compositor,
            &self.derecha,
            (self.home_x + timing.travel, self.home_y),
            (self.home_x, self.home_y),
            timing.mountains_seconds,
            &ease,
        )?;
        start_blur(
            compositor,
            &self.blur_left,
            timing.blur_max,
            timing.mountains_seconds,
        )?;
        start_blur(
            compositor,
            &self.blur_right,
            timing.blur_max,
            timing.mountains_seconds,
        )?;
        photo.SetOpacity(0.0)?;
        let photo_in = compositor.CreateScalarKeyFrameAnimation()?;
        photo_in.InsertKeyFrame(0.0, 0.0)?;
        photo_in.InsertKeyFrame(1.0, 1.0)?;
        photo_in.SetDelayTime(span(timing.mountains_seconds))?;
        photo_in.SetDuration(span(timing.photo_fade_seconds))?;
        photo.StartAnimation(&HSTRING::from("Opacity"), &photo_in)?;
        content.SetOpacity(0.0)?;
        content.SetOffset(Vector3 {
            X: 0.0,
            Y: timing.rise,
            Z: 0.0,
        })?;
        let content_in = compositor.CreateScalarKeyFrameAnimation()?;
        content_in.InsertKeyFrame(0.0, 0.0)?;
        content_in.InsertKeyFrame(1.0, 1.0)?;
        content_in.SetDelayTime(span(timing.content_delay_seconds))?;
        content_in.SetDuration(span(timing.content_seconds))?;
        content.StartAnimation(&HSTRING::from("Opacity"), &content_in)?;
        let rise = compositor.CreateVector3KeyFrameAnimation()?;
        rise.InsertKeyFrame(
            0.0,
            Vector3 {
                X: 0.0,
                Y: timing.rise,
                Z: 0.0,
            },
        )?;
        rise.InsertKeyFrameWithEasingFunction(
            1.0,
            Vector3 {
                X: 0.0,
                Y: 0.0,
                Z: 0.0,
            },
            &ease,
        )?;
        rise.SetDelayTime(span(timing.content_delay_seconds))?;
        rise.SetDuration(span(timing.content_seconds))?;
        content.StartAnimation(&HSTRING::from("Offset"), &rise)
    }

    /// Detiene las animaciones, oculta y quita las capas y deja el estado
    /// final (foto y contenido visibles, sin efectos activos).
    pub(crate) fn finish(
        &self,
        parent: &ContainerVisual,
        photo: &SpriteVisual,
        content: &ContainerVisual,
    ) -> Result<()> {
        self.izquierda.StopAnimation(&HSTRING::from("Offset"))?;
        self.blur_left
            .StopAnimation(&HSTRING::from(BLUR_ANIMATABLE))?;
        self.derecha.StopAnimation(&HSTRING::from("Offset"))?;
        self.blur_right
            .StopAnimation(&HSTRING::from(BLUR_ANIMATABLE))?;
        photo.StopAnimation(&HSTRING::from("Opacity"))?;
        content.StopAnimation(&HSTRING::from("Opacity"))?;
        content.StopAnimation(&HSTRING::from("Offset"))?;
        self.fondo.SetIsVisible(false)?;
        self.izquierda.SetIsVisible(false)?;
        self.derecha.SetIsVisible(false)?;
        let children = parent.Children()?;
        children.Remove(&self.fondo)?;
        children.Remove(&self.izquierda)?;
        children.Remove(&self.derecha)?;
        photo.SetOpacity(1.0)?;
        content.SetOpacity(1.0)?;
        content.SetOffset(Vector3 {
            X: 0.0,
            Y: 0.0,
            Z: 0.0,
        })
    }
}

/// Desplazamiento de una montaña hasta casa con la frenada suave del tema.
fn start_travel(
    compositor: &Compositor,
    visual: &SpriteVisual,
    from: (f32, f32),
    to: (f32, f32),
    seconds: f32,
    ease: &windows::UI::Composition::CubicBezierEasingFunction,
) -> Result<()> {
    let shift = compositor.CreateVector3KeyFrameAnimation()?;
    shift.InsertKeyFrame(
        0.0,
        Vector3 {
            X: from.0,
            Y: from.1,
            Z: 0.0,
        },
    )?;
    shift.InsertKeyFrameWithEasingFunction(
        1.0,
        Vector3 {
            X: to.0,
            Y: to.1,
            Z: 0.0,
        },
        ease,
    )?;
    shift.SetDuration(span(seconds))?;
    visual.StartAnimation(&HSTRING::from("Offset"), &shift)
}

/// El desenfoque baja lineal hasta 0 durante el viaje.
fn start_blur(
    compositor: &Compositor,
    brush: &CompositionEffectBrush,
    blur_max: f32,
    seconds: f32,
) -> Result<()> {
    let blur = compositor.CreateScalarKeyFrameAnimation()?;
    blur.InsertKeyFrame(0.0, blur_max)?;
    blur.InsertKeyFrame(1.0, 0.0)?;
    blur.SetDuration(span(seconds))?;
    brush.StartAnimation(&HSTRING::from(BLUR_ANIMATABLE), &blur)
}
