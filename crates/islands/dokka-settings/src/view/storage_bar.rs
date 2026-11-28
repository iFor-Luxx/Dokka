//! Barra de almacenamiento: pista gris y un tramo por categoría.
//!
//! Sigue el patrón del deslizador: la pista existe siempre y cada tramo la
//! cubre desde la izquierda. Los tramos existen siempre (de tamaño cero sin
//! datos) para no reconstruir capas mientras se calcula. Nunca hay anchos
//! negativos: lo que sobra se recorta al final de la pista.

use dokka_model::StorageBreakdown;
use dokka_scene::{Panel, Rect};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::storage_category_color;

/// Pista y tramos de la barra para la fila que empieza en `row_top`.
pub(crate) fn storage_bar_panels(
    theme: &Theme,
    width: f32,
    row_top: f32,
    breakdown: Option<&StorageBreakdown>,
    window_height: f32,
) -> Vec<Panel> {
    let track = storage_track_bounds(width, row_top);
    let middle = track.y + track.height / 2.0;
    if middle < measures::CONTENT_TOP || middle > window_height {
        return vec![hidden_panel(theme); 10];
    }
    let mut panels = vec![Panel {
        bounds: track,
        corner_radius: measures::STORAGE_BAR_RADIUS,
        border_width: 0.0,
        tint: theme.settings.slider_track,
        border: theme.settings.slider_track,
    }];
    let Some(known) = breakdown else {
        for _ in 0..9 {
            panels.push(hidden_segment(theme));
        }
        return panels;
    };
    let Some(total) = known.total_bytes.filter(|total| *total > 0) else {
        for _ in 0..9 {
            panels.push(hidden_segment(theme));
        }
        return panels;
    };
    let parts = [
        known.apps_bytes.unwrap_or(0),
        known.documents_bytes.unwrap_or(0),
        known.images_bytes.unwrap_or(0),
        known.music_bytes.unwrap_or(0),
        known.videos_bytes.unwrap_or(0),
        known.downloads_bytes.unwrap_or(0),
        known.recycle_bytes.unwrap_or(0),
        known.windows_bytes.unwrap_or(0),
        known.other_bytes().unwrap_or(0),
    ];
    let mut cursor = track.x;
    let end = track.x + track.width;
    for (index, bytes) in parts.into_iter().enumerate() {
        let share = bytes as f32 / total as f32 * track.width;
        let segment_width = share.min(end - cursor).max(0.0);
        if segment_width <= 0.0 {
            panels.push(hidden_segment(theme));
            continue;
        }
        // Radio completo como el relleno del deslizador: los extremos de la
        // pista quedan redondos y entre tramos se ve un filo de la pista.
        let color = storage_category_color(theme, index);
        panels.push(Panel {
            bounds: Rect {
                x: cursor,
                y: track.y,
                width: segment_width,
                height: track.height,
            },
            corner_radius: measures::STORAGE_BAR_RADIUS,
            border_width: 0.0,
            tint: color,
            border: color,
        });
        cursor += segment_width;
    }
    panels
}

/// Pista de la barra dentro de su fila: aire a los lados y hueco arriba.
fn storage_track_bounds(width: f32, row_top: f32) -> Rect {
    let x = measures::CONTENT_LEFT + measures::CONTENT_PADDING;
    Rect {
        x,
        y: row_top + measures::STORAGE_BAR_TOP_GAP,
        width: (width - measures::CONTENT_PADDING * 2.0).max(0.0),
        height: measures::STORAGE_BAR_HEIGHT,
    }
}

/// Tramo sin datos: existe de tamaño cero para no reconstruir capas.
fn hidden_segment(theme: &Theme) -> Panel {
    hidden_panel(theme)
}

/// Panel de tamaño cero con el estilo de la tarjeta.
fn hidden_panel(theme: &Theme) -> Panel {
    Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: measures::STORAGE_BAR_RADIUS,
        border_width: 0.0,
        tint: theme.settings.slider_track,
        border: theme.settings.slider_track,
    }
}
