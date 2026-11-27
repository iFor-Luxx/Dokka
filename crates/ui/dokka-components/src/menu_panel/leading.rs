//! Icono de la izquierda de cada fila: su icono o la marca elegida.
//!
//! Una sola columna para que todos los textos queden alineados.
//! Las filas sin icono dejan el hueco.

use std::sync::Arc;

use dokka_scene::{Color, Drawing, Icon, IconImage, Rect, Shape};
use dokka_theme::{Theme, menu as measures};

use super::MenuRow;
use super::check::MenuChecks;

/// Iconos de la columna izquierda, en el orden de las filas:
/// la marca si está elegida, si no su icono. Al resaltar se tiñen
/// con el color del texto resaltado.
pub(super) fn leading_icons(
    theme: &Theme,
    checks: &MenuChecks,
    bounds: Rect,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Vec<Icon> {
    let mut top = bounds.y + measures::PADDING;
    let mut icons = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        if let MenuRow::Item(item) = row {
            let is_highlighted = highlighted == Some(index);
            if item.checked {
                icons.push(lead_icon(
                    theme,
                    if is_highlighted {
                        &checks.highlighted
                    } else {
                        &checks.normal
                    },
                    bounds,
                    top,
                ));
            } else if let Some(icon) = item.icon.as_ref() {
                let shown = if is_highlighted {
                    Arc::new(tinted(icon, theme.menu.highlighted_text))
                } else {
                    Arc::clone(icon)
                };
                icons.push(lead_icon(theme, &shown, bounds, top));
            }
            top += measures::ROW_HEIGHT;
        } else {
            top += measures::SEPARATOR_HEIGHT;
        }
    }
    icons
}

fn lead_icon(theme: &Theme, drawing: &Arc<Drawing>, bounds: Rect, top: f32) -> Icon {
    Icon {
        x: bounds.x + measures::PADDING + (measures::ICON_COLUMN - measures::ROW_ICON_SIDE) / 2.0,
        y: top + (measures::ROW_HEIGHT - measures::ROW_ICON_SIDE) / 2.0,
        side: measures::ROW_ICON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.menu.background,
        placeholder_radius: 0.0,
    }
}

/// Copia del dibujo con el color del texto resaltado.
/// Los iconos del menú son glifos Lucide de un solo color.
fn tinted(drawing: &Drawing, color: Color) -> Drawing {
    Drawing {
        shapes: drawing
            .shapes
            .iter()
            .map(|shape| match shape {
                Shape::Glyph {
                    glyph, x, y, size, ..
                } => Shape::Glyph {
                    glyph: *glyph,
                    x: *x,
                    y: *y,
                    size: *size,
                    color,
                },
                Shape::RoundedRect {
                    x,
                    y,
                    width,
                    height,
                    radius,
                    ..
                } => Shape::RoundedRect {
                    x: *x,
                    y: *y,
                    width: *width,
                    height: *height,
                    radius: *radius,
                    color,
                },
                Shape::Circle { x, y, radius, .. } => Shape::Circle {
                    x: *x,
                    y: *y,
                    radius: *radius,
                    color,
                },
                Shape::Line {
                    from, to, width, ..
                } => Shape::Line {
                    from: *from,
                    to: *to,
                    width: *width,
                    color,
                },
                Shape::Triangle { points, .. } => Shape::Triangle {
                    points: *points,
                    color,
                },
                Shape::Text {
                    text,
                    x,
                    y,
                    size,
                    is_bold,
                    ..
                } => Shape::Text {
                    text: text.clone(),
                    x: *x,
                    y: *y,
                    size: *size,
                    is_bold: *is_bold,
                    color,
                },
            })
            .collect(),
    }
}
