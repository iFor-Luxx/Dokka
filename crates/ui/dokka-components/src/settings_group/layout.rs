use dokka_scene::Rect;
use dokka_theme::settings as measures;

/// Alto de la tarjeta: suma de sus filas.
pub fn group_height(heights: &[f32]) -> f32 {
    heights.iter().sum()
}

/// Borde superior de la fila `index` con estas alturas.
fn row_top(top: f32, heights: &[f32], index: usize) -> f32 {
    top + heights.iter().take(index).sum::<f32>()
}

/// Límites de la tarjeta que empieza en `top` con esta altura total.
pub fn group_panel_bounds(top: f32, total: f32) -> Rect {
    group_panel_bounds_for(top, total, measures::CONTENT_WIDTH)
}

/// Límites de la tarjeta para este ancho de contenido.
pub fn group_panel_bounds_for(top: f32, total: f32, width: f32) -> Rect {
    Rect {
        x: measures::CONTENT_LEFT,
        y: top,
        width,
        height: total,
    }
}

/// Fila `index` dentro de la tarjeta que empieza en `top`.
pub fn row_bounds(top: f32, heights: &[f32], index: usize) -> Rect {
    row_bounds_for(top, heights, index, measures::CONTENT_WIDTH)
}

/// Fila para este ancho de contenido: los controles, pegados al margen
/// derecho, se adaptan solos al ensancharse.
pub fn row_bounds_for(top: f32, heights: &[f32], index: usize, width: f32) -> Rect {
    let height = heights.get(index).copied().unwrap_or(0.0);
    Rect {
        x: measures::CONTENT_LEFT,
        y: row_top(top, heights, index),
        width,
        height,
    }
}

/// Línea fina entre la fila `index` y la siguiente: empieza en la columna
/// del texto y acaba en el margen derecho, nunca de borde a borde.
pub fn divider_rect(top: f32, heights: &[f32], index: usize) -> Rect {
    divider_rect_for(top, heights, index, measures::CONTENT_WIDTH)
}

/// Línea para este ancho de contenido.
pub fn divider_rect_for(top: f32, heights: &[f32], index: usize, width: f32) -> Rect {
    let row = row_bounds_for(top, heights, index, width);
    Rect {
        x: row.x + measures::CONTENT_PADDING,
        y: row.y + row.height - 1.0,
        width: row.width - measures::CONTENT_PADDING * 2.0,
        height: 1.0,
    }
}
