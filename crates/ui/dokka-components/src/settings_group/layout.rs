use dokka_scene::Rect;
use dokka_theme::settings as measures;

/// Alto de la tarjeta para estas filas.
pub fn group_height(rows: usize) -> f32 {
    rows as f32 * measures::CONTENT_ROW_HEIGHT
}

/// Límites de la tarjeta que empieza en `top` con estas filas.
pub fn group_panel_bounds(top: f32, rows: usize) -> Rect {
    Rect {
        x: measures::CONTENT_LEFT,
        y: top,
        width: measures::CONTENT_WIDTH,
        height: group_height(rows),
    }
}

/// Fila `index` dentro de la tarjeta que empieza en `top`.
pub fn row_bounds(top: f32, index: usize) -> Rect {
    Rect {
        x: measures::CONTENT_LEFT,
        y: top + index as f32 * measures::CONTENT_ROW_HEIGHT,
        width: measures::CONTENT_WIDTH,
        height: measures::CONTENT_ROW_HEIGHT,
    }
}

/// Línea fina entre la fila `index` y la siguiente, con aire a la izquierda.
pub fn divider_rect(top: f32, index: usize) -> Rect {
    let row = row_bounds(top, index);
    Rect {
        x: row.x + measures::CONTENT_PADDING,
        y: row.y + row.height - 1.0,
        width: row.width - measures::CONTENT_PADDING,
        height: 1.0,
    }
}
