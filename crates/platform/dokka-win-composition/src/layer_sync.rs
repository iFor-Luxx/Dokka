//! Reutiliza capas al encoger una escena: actualiza las necesarias y oculta el resto.
//!
//! Las capas sobrantes se ocultan con `SetIsVisible(false)` en su sitio del
//! orden de pintado; al volver a crecer hasta la reserva no se reconstruye.
//! Si la escena pide más capas de las que hay, `matches` ya devolvió `false`
//! y se reconstruye una sola vez con la reserva mayor.

use dokka_scene::{Divider, Dot, Icon, Label, Text};
use windows::UI::Composition::Compositor;
use windows::core::Result;

use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::label_layer::LabelLayer;
use crate::painter::Painter;
use crate::text_layer::TextLayer;

/// Actualiza los iconos pedidos y oculta la reserva sobrante.
pub(crate) fn sync_icons(
    layers: &mut [IconLayer],
    compositor: &Compositor,
    painter: &Painter,
    icons: &[Icon],
) -> Result<()> {
    for (index, layer) in layers.iter_mut().enumerate() {
        if let Some(icon) = icons.get(index) {
            layer.update(compositor, painter, icon)?;
        } else {
            layer.set_visible(false)?;
        }
    }
    Ok(())
}

/// Actualiza los puntos pedidos y oculta la reserva sobrante.
pub(crate) fn sync_dots(
    layers: &mut [DotLayer],
    compositor: &Compositor,
    dots: &[Dot],
) -> Result<()> {
    for (index, layer) in layers.iter_mut().enumerate() {
        if let Some(dot) = dots.get(index) {
            layer.update(compositor, dot)?;
        } else {
            layer.set_visible(false)?;
        }
    }
    Ok(())
}

/// Actualiza los textos pedidos y oculta la reserva sobrante.
pub(crate) fn sync_texts(
    layers: &mut [TextLayer],
    compositor: &Compositor,
    painter: &Painter,
    texts: &[Text],
) -> Result<()> {
    for (index, layer) in layers.iter_mut().enumerate() {
        if let Some(text) = texts.get(index) {
            layer.update(compositor, painter, text)?;
        } else {
            layer.set_visible(false)?;
        }
    }
    Ok(())
}

/// Actualiza las líneas pedidas y oculta la reserva sobrante.
pub(crate) fn sync_dividers(layers: &[DividerLayer], dividers: &[Divider]) -> Result<()> {
    for (index, layer) in layers.iter().enumerate() {
        if let Some(divider) = dividers.get(index) {
            layer.update(divider)?;
        } else {
            layer.set_visible(false)?;
        }
    }
    Ok(())
}

/// Actualiza las etiquetas pedidas y oculta la reserva sobrante.
pub(crate) fn sync_labels(
    layers: &mut [LabelLayer],
    compositor: &Compositor,
    painter: &Painter,
    labels: &[Label],
) -> Result<()> {
    for (index, layer) in layers.iter_mut().enumerate() {
        if let Some(label) = labels.get(index) {
            layer.update(compositor, painter, label)?;
        } else {
            layer.set_visible(false)?;
        }
    }
    Ok(())
}
