use dokka_scene::{Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::{SegmentedProps, layout::{segment_bounds, segmented_touch}};

/// Cápsula gris de fondo con el color del tema, extremos redondos.
fn segment_track_panel(theme: &Theme, touch: Rect) -> Panel {
    Panel {
        bounds: touch,
        corner_radius: touch.height / 2.0,
        border_width: 0.0,
        tint: theme.settings.segment_track,
        border: theme.settings.segment_track,
    }
}

/// Pastilla blanca del segmento elegido, encogida con el aire interior:
/// radio concéntrico (exterior menos separación).
fn segment_selected_panel(theme: &Theme, touch: Rect, selected: usize, count: usize) -> Panel {
    let cell = segment_bounds(touch, selected, count);
    let bounds = Rect {
        x: cell.x + measures::SEGMENTED_INSET,
        y: cell.y + measures::SEGMENTED_INSET,
        width: (cell.width - measures::SEGMENTED_INSET * 2.0).max(0.0),
        height: (cell.height - measures::SEGMENTED_INSET * 2.0).max(0.0),
    };
    Panel {
        bounds,
        corner_radius: (bounds.height / 2.0).max(0.0),
        border_width: 0.0,
        tint: theme.settings.segment_selected,
        border: theme.settings.segment_selected,
    }
}

/// Fondo y pastilla elegida, siempre dos planos: al cambiar de segmento solo
/// se mueve la pastilla, sin reconstruir capas.
pub fn segmented_panels(theme: &Theme, touch: Rect, props: &SegmentedProps) -> [Panel; 2] {
    let count = props.options.len().max(1);
    let selected = props.selected.min(count.saturating_sub(1));
    [
        segment_track_panel(theme, touch),
        segment_selected_panel(theme, touch, selected, count),
    ]
}

/// Textos centrados en cada tercio: con el ancho del tema caben sin salirse.
/// El elegido va en tinta, el resto en gris de la tarjeta.
pub fn segmented_texts(theme: &Theme, touch: Rect, props: &SegmentedProps) -> Vec<Text> {
    let count = props.options.len();
    props
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let cell = segment_bounds(touch, index, count.max(1));
            Text {
                x: cell.x + cell.width / 2.0,
                center_y: touch.y + touch.height / 2.0,
                content: option.clone(),
                size: measures::SEGMENTED_TEXT_SIZE,
                is_bold: false,
                color: if index == props.selected.min(count.saturating_sub(1)) {
                    theme.settings.row_text
                } else {
                    theme.settings.card_text
                },
                align: TextAlign::Center,
            }
        })
        .collect()
}

/// Segmento bajo el punto con la misma geometría del dibujo: la cápsula
/// completa dividida en tercios iguales; fuera, nada.
pub fn segmented_at(touch: Rect, count: usize, x: f32, y: f32) -> Option<usize> {
    if count == 0 {
        return None;
    }
    if !(touch.x..touch.x + touch.width).contains(&x)
        || !(touch.y..touch.y + touch.height).contains(&y)
    {
        return None;
    }
    let ratio = (x - touch.x) / touch.width;
    let index = (ratio * count as f32).floor() as usize;
    Some(index.min(count - 1))
}

/// Segmento bajo el punto en esta fila, si hay alguno.
pub fn segmented_hit(row: Rect, props: &SegmentedProps, x: f32, y: f32) -> Option<usize> {
    segmented_at(segmented_touch(row), props.options.len(), x, y)
}
