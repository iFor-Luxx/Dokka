use dokka_scene::Rect;
use dokka_theme::settings as measures;

/// Zona del segmentado, a la derecha de su fila con el mismo aire que el
/// texto a la izquierda. El dibujo y el golpe usan este rectángulo.
pub fn segmented_touch(row: Rect) -> Rect {
    Rect {
        x: row.x + row.width - measures::CONTENT_PADDING - measures::SEGMENTED_WIDTH,
        y: row.y + (row.height - measures::SEGMENTED_HEIGHT) / 2.0,
        width: measures::SEGMENTED_WIDTH,
        height: measures::SEGMENTED_HEIGHT,
    }
}

/// Tercio del segmento `index` dentro de la cápsula, de igual ancho.
pub fn segment_bounds(touch: Rect, index: usize, count: usize) -> Rect {
    if count == 0 {
        return Rect {
            x: touch.x,
            y: touch.y,
            width: 0.0,
            height: touch.height,
        };
    }
    let width = touch.width / count as f32;
    Rect {
        x: touch.x + width * index as f32,
        y: touch.y,
        width,
        height: touch.height,
    }
}
