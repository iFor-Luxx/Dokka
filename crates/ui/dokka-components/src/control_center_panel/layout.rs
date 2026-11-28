//! Dónde cae cada control visible, única fuente para tamaño, dibujo,
//! golpe y arrastre.
//!
//! Rejilla de cuatro columnas: se recorren los controles visibles en el
//! orden de fábrica y cada uno va al primer hueco libre (por filas, de
//! izquierda a derecha) donde quepa. Anchos por alto en celdas: Red y
//! Preferencias 2x1, Música 2x2, Pantalla y Sonido 4x1, el resto 1x1. Una
//! fila con deslizador mide su alto; las demás, `CELL`. Hueco `GAP` y aire
//! `PADDING` como siempre. Con nada oculto cada pieza cae donde caía.

use dokka_model::{ControlItem, HiddenControls};
use dokka_scene::Rect;
use dokka_theme::control_center as measures;

/// Ancho por alto en celdas de cada control.
pub(crate) fn cell_span(item: ControlItem) -> (usize, usize) {
    match item {
        ControlItem::Network | ControlItem::Settings => (2, 1),
        ControlItem::Music => (2, 2),
        ControlItem::Brightness | ControlItem::Sound => (4, 1),
        _ => (1, 1),
    }
}

/// Alto en píxeles de una fila con deslizador a todo lo ancho.
pub(crate) fn slider_height(item: ControlItem) -> Option<f32> {
    match item {
        ControlItem::Brightness => Some(measures::BRIGHTNESS_HEIGHT),
        ControlItem::Sound => Some(measures::SOUND_HEIGHT),
        _ => None,
    }
}

/// Rectángulo de cada control visible y alto total del panel.
#[derive(Debug, Clone)]
pub(crate) struct ControlLayout {
    rects: [Option<Rect>; 13],
    height: f32,
}

impl ControlLayout {
    /// Rectángulo de este control, o nada si está oculto (o sin DDC).
    pub(crate) fn rect(&self, item: ControlItem) -> Option<Rect> {
        self.rects[item as usize]
    }

    /// Alto total con el aire exterior.
    pub(crate) fn height(&self) -> f32 {
        self.height
    }
}

/// Reparte los controles visibles sobre la rejilla de cuatro columnas.
pub(crate) fn arrange(
    bounds: Rect,
    hidden: HiddenControls,
    with_brightness: bool,
) -> ControlLayout {
    let mut visible = Vec::with_capacity(13);
    for item in ControlItem::FACTORY_ORDER {
        if hidden.contains(item) {
            continue;
        }
        if item == ControlItem::Brightness && !with_brightness {
            continue;
        }
        visible.push(item);
    }
    let mut grid: Vec<[bool; 4]> = Vec::new();
    let mut row_heights: Vec<f32> = Vec::new();
    let mut placed: Vec<(ControlItem, usize, usize)> = Vec::with_capacity(visible.len());
    for item in visible {
        let (span_w, span_h) = cell_span(item);
        let mut row = 0;
        let (placed_row, placed_column) = loop {
            while grid.len() < row + span_h {
                grid.push([false; 4]);
                row_heights.push(measures::CELL);
            }
            let mut at = None;
            for column in 0..=(4 - span_w) {
                if fits(&grid, row, column, span_w, span_h) {
                    at = Some((row, column));
                    break;
                }
            }
            if let Some(found) = at {
                break found;
            }
            row += 1;
        };
        let (row, column) = (placed_row, placed_column);
        if let Some(height) = slider_height(item) {
            row_heights[row] = height;
        }
        for delta_row in 0..span_h {
            for delta_column in 0..span_w {
                grid[row + delta_row][column + delta_column] = true;
            }
        }
        placed.push((item, row, column));
    }
    let mut rows_top = Vec::with_capacity(row_heights.len());
    let mut cursor = bounds.y + measures::PADDING;
    for height in &row_heights {
        rows_top.push(cursor);
        cursor += height + measures::GAP;
    }
    let height = if row_heights.is_empty() {
        measures::PADDING * 2.0
    } else {
        cursor - measures::GAP + measures::PADDING - bounds.y
    };
    let mut rects: [Option<Rect>; 13] = [None; 13];
    for (item, row, column) in placed {
        let (span_w, span_h) = cell_span(item);
        let width = span_w as f32 * measures::CELL + (span_w as f32 - 1.0) * measures::GAP;
        let item_height = slider_height(item)
            .unwrap_or(span_h as f32 * measures::CELL + (span_h as f32 - 1.0) * measures::GAP);
        rects[item as usize] = Some(Rect {
            x: bounds.x + measures::PADDING + column as f32 * (measures::CELL + measures::GAP),
            y: rows_top[row],
            width,
            height: item_height,
        });
    }
    ControlLayout { rects, height }
}

/// `true` si el hueco de este tamaño cabe libre aquí (lo que aún no
/// existe cuenta como libre).
fn fits(grid: &[[bool; 4]], row: usize, column: usize, span_w: usize, span_h: usize) -> bool {
    for delta_row in 0..span_h {
        for delta_column in 0..span_w {
            if let Some(occupied) = grid
                .get(row + delta_row)
                .map(|cells| cells[column + delta_column])
                && occupied
            {
                return false;
            }
        }
    }
    true
}

/// Círculo con icono dentro de una cápsula, con margen igual arriba,
/// abajo e izquierda.
pub(crate) fn capsule_circle(capsule: Rect) -> Rect {
    Rect {
        x: capsule.x + measures::CAPSULE_MARGIN,
        y: capsule.y + measures::CAPSULE_MARGIN,
        width: measures::CAPSULE_CIRCLE_SIDE,
        height: measures::CAPSULE_CIRCLE_SIDE,
    }
}

/// La `x` del título y el subtítulo de una cápsula.
pub(crate) fn capsule_text_x(capsule: Rect) -> f32 {
    capsule_circle(capsule).x + measures::CAPSULE_CIRCLE_SIDE + measures::ICON_TEXT_GAP
}

/// Carátula marcadora dentro de la música, arriba a la izquierda.
pub(crate) fn music_art(card: Rect) -> Rect {
    Rect {
        x: card.x + measures::PADDING,
        y: card.y + measures::PADDING,
        width: measures::MUSIC_ART_SIDE,
        height: measures::MUSIC_ART_SIDE,
    }
}

/// Botón de música `index` (0 anterior, 1 pausa, 2 siguiente).
pub(crate) fn music_button(card: Rect, index: usize) -> Rect {
    let total = measures::MUSIC_BUTTON_SIDE * 3.0 + measures::MUSIC_BUTTON_GAP * 2.0;
    Rect {
        x: card.x
            + (card.width - total) / 2.0
            + (measures::MUSIC_BUTTON_SIDE + measures::MUSIC_BUTTON_GAP) * index as f32,
        y: card.y + card.height - measures::PADDING - measures::MUSIC_BUTTON_SIDE,
        width: measures::MUSIC_BUTTON_SIDE,
        height: measures::MUSIC_BUTTON_SIDE,
    }
}

/// Centro vertical de la pista del deslizador, en la parte baja.
fn track_center_y(card: Rect) -> f32 {
    card.y + card.height - measures::SLIDER_TRACK_BOTTOM
}

/// Icono pequeño del extremo izquierdo del deslizador.
pub(crate) fn sound_left_icon(card: Rect) -> Rect {
    Rect {
        x: card.x + measures::PADDING,
        y: track_center_y(card) - measures::END_ICON_SIDE / 2.0,
        width: measures::END_ICON_SIDE,
        height: measures::END_ICON_SIDE,
    }
}

/// Caja del silencio: icono pequeño a la derecha, con aire para pulsarlo.
/// Se conserva para el golpe; lo visible va en `sound_mute_icon`.
/// También vale para el icono derecho del brillo: misma geometría.
pub(crate) fn sound_mute_box(card: Rect) -> Rect {
    Rect {
        x: card.x + card.width - measures::PADDING - measures::MUTE_HIT_SIDE,
        y: track_center_y(card) - measures::MUTE_HIT_SIDE / 2.0,
        width: measures::MUTE_HIT_SIDE,
        height: measures::MUTE_HIT_SIDE,
    }
}

/// Icono visible del silencio: termina a la misma distancia del borde que
/// empieza el izquierdo (`SOUND_ICON_MARGIN`), centrado en la pista.
/// También vale para el icono derecho del brillo: misma geometría.
pub(crate) fn sound_mute_icon(card: Rect) -> Rect {
    Rect {
        x: card.x + card.width - measures::SOUND_ICON_MARGIN - measures::END_ICON_SIDE,
        y: track_center_y(card) - measures::END_ICON_SIDE / 2.0,
        width: measures::END_ICON_SIDE,
        height: measures::END_ICON_SIDE,
    }
}

/// Zona del deslizador entre los dos iconos visibles, con el mismo aire a
/// cada lado para que la barra quede centrada y simétrica. Es la zona que
/// recibe el componente `slider` (Sonido y Pantalla comparten geometría).
/// La altura da aire vertical para no exigir puntería fina.
pub(crate) fn slider_area(card: Rect) -> Rect {
    let left = sound_left_icon(card);
    let right = sound_mute_icon(card);
    let x = left.x + left.width + measures::ICON_TEXT_GAP;
    let end = right.x - measures::ICON_TEXT_GAP;
    Rect {
        x,
        y: track_center_y(card) - measures::SLIDER_HIT_HALF,
        width: (end - x).max(0.0),
        height: measures::SLIDER_HIT_HALF * 2.0,
    }
}
