use dokka_scene::{Dot, Panel, Rect, Size};
use dokka_theme::{Theme, volume as measures};

use super::SliderProps;

/// Recorta un valor al rango del deslizador, de 0 a 1.
/// Sin `clamp`: el silencio (`NaN`) queda en 0 en vez de propagarse.
fn clamp01(value: f32) -> f32 {
    if !(value >= 0.0) {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

/// Tamaño del deslizador para una barra de este ancho: la barra más
/// el tirador, que sobresale por arriba y por abajo.
pub fn slider_size(track_width: f32) -> Size {
    Size {
        width: track_width,
        height: measures::KNOB_DIAMETER.max(measures::TRACK_HEIGHT),
    }
}

/// Barra centrada en vertical dentro de su zona de toque.
pub fn slider_track_bounds(area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y + (area.height - measures::TRACK_HEIGHT) / 2.0,
        width: area.width,
        height: measures::TRACK_HEIGHT,
    }
}

/// Barra de fondo, gris tenue.
pub fn slider_track_panel(theme: &Theme, track: Rect) -> Panel {
    Panel {
        bounds: track,
        corner_radius: measures::TRACK_RADIUS,
        border_width: 0.0,
        tint: theme.settings.search_background,
        border: theme.settings.search_background,
    }
}

/// Relleno hasta el valor: azul si responde, gris si está atenuado.
/// Existe siempre (de ancho cero al mínimo) para no reconstruir capas.
pub fn slider_fill_panel(theme: &Theme, track: Rect, value: f32, active: bool) -> Panel {
    let (tint, border) = if active {
        (theme.menu.highlight, theme.menu.highlight)
    } else {
        (theme.menu.separator, theme.menu.separator)
    };
    Panel {
        bounds: Rect {
            x: track.x,
            y: track.y,
            width: track.width * clamp01(value),
            height: track.height,
        },
        corner_radius: measures::TRACK_RADIUS,
        border_width: 0.0,
        tint,
        border,
    }
}

/// Tirador blanco centrado sobre el valor.
pub fn slider_knob(theme: &Theme, track: Rect, value: f32) -> Dot {
    let center_x = track.x + track.width * clamp01(value);
    let center_y = track.y + track.height / 2.0;
    Dot {
        x: center_x - measures::KNOB_DIAMETER / 2.0,
        y: center_y - measures::KNOB_DIAMETER / 2.0,
        diameter: measures::KNOB_DIAMETER,
        color: theme.panel.highlight,
    }
}

/// Barra y relleno, en ese orden.
pub fn slider_panels(theme: &Theme, track: Rect, props: &SliderProps) -> [Panel; 2] {
    [
        slider_track_panel(theme, track),
        slider_fill_panel(theme, track, props.value, props.active),
    ]
}

/// Valor bajo el punto sobre la barra, de 0 a 1.
pub fn slider_value_at(track: Rect, x: f32) -> f32 {
    clamp01((x - track.x) / track.width)
}

/// Valor elegido si el punto cae en la zona de toque; fuera, nada.
pub fn slider_hit(area: Rect, x: f32, y: f32) -> Option<f32> {
    if (area.x..area.x + area.width).contains(&x)
        && (area.y..area.y + area.height).contains(&y)
    {
        Some(slider_value_at(slider_track_bounds(area), x))
    } else {
        None
    }
}
