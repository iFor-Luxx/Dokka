use dokka_scene::{Panel, Rect, Size};
use dokka_theme::tray_glass as measures;

use super::{SliderColors, SliderProps};

/// Recorta un valor al rango del deslizador, de 0 a 1.
/// Sin `clamp`: el silencio (`NaN`) queda en 0 en vez de propagarse.
fn clamp01(value: f32) -> f32 {
    if value.is_nan() || value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

/// Radio recortado para no superar la mitad del lado menor.
/// Con ancho o alto a cero, el radio es cero.
fn capped_radius(maximum: f32, width: f32, height: f32) -> f32 {
    if width <= 0.0 || height <= 0.0 {
        return 0.0;
    }
    maximum.min(width / 2.0).min(height / 2.0).max(0.0)
}

/// Tamaño de la cápsula para este ancho: extremos totalmente redondos.
pub fn slider_size(track_width: f32) -> Size {
    Size {
        width: track_width.max(0.0),
        height: measures::TRACK_HEIGHT,
    }
}

/// Barra centrada en vertical dentro de su zona de toque (más alta que ella).
pub fn slider_track_bounds(area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y + (area.height - measures::TRACK_HEIGHT) / 2.0,
        width: area.width,
        height: measures::TRACK_HEIGHT,
    }
}

/// Barra de fondo con el color recibido, extremos totalmente redondos.
pub fn slider_track_panel(track: Rect, colors: &SliderColors) -> Panel {
    Panel {
        bounds: track,
        corner_radius: capped_radius(track.height / 2.0, track.width, track.height),
        border_width: 0.0,
        tint: colors.track,
        border: colors.track,
    }
}

/// Relleno hasta el valor con el color recibido (principal o atenuado).
/// Existe siempre (de ancho cero al mínimo) para no reconstruir capas.
pub fn slider_fill_panel(track: Rect, value: f32, active: bool, colors: &SliderColors) -> Panel {
    let width = track.width * clamp01(value);
    let tint = if active { colors.fill } else { colors.muted };
    Panel {
        bounds: Rect {
            x: track.x,
            y: track.y,
            width,
            height: track.height,
        },
        corner_radius: capped_radius(track.height / 2.0, width, track.height),
        border_width: 0.0,
        tint,
        border: tint,
    }
}

/// Barra y relleno, en ese orden.
pub fn slider_panels(track: Rect, props: &SliderProps, colors: &SliderColors) -> [Panel; 2] {
    [
        slider_track_panel(track, colors),
        slider_fill_panel(track, props.value, props.active, colors),
    ]
}

/// Valor bajo el punto sobre la barra, de 0 a 1.
pub fn slider_value_at(track: Rect, x: f32) -> f32 {
    clamp01((x - track.x) / track.width)
}

/// Valor elegido si el punto cae en la zona de toque; fuera, nada.
pub fn slider_hit(area: Rect, x: f32, y: f32) -> Option<f32> {
    if (area.x..area.x + area.width).contains(&x) && (area.y..area.y + area.height).contains(&y) {
        Some(slider_value_at(slider_track_bounds(area), x))
    } else {
        None
    }
}
