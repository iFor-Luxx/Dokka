use dokka_scene::{Panel, Rect};
use dokka_theme::settings as measures;

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

/// Barra fina centrada en vertical dentro de su zona, con aire a los lados
/// de medio tirador: el centro del tirador recorre la pista sin salirse del
/// área y en los extremos el tirador enrasa con ella.
pub fn slider_track_bounds(area: Rect) -> Rect {
    let side = measures::SLIDER_KNOB_WIDTH / 2.0;
    Rect {
        x: area.x + side,
        y: area.y + (area.height - measures::SLIDER_TRACK_HEIGHT) / 2.0,
        width: (area.width - side * 2.0).max(0.0),
        height: measures::SLIDER_TRACK_HEIGHT,
    }
}

/// Barra de fondo con el color recibido, extremos totalmente redondos.
pub fn slider_track_panel(track: Rect, colors: &SliderColors) -> Panel {
    Panel {
        bounds: track,
        corner_radius: track.height / 2.0,
        border_width: 0.0,
        tint: colors.track,
        border: colors.track,
    }
}

/// Relleno hasta el valor con el color recibido (principal o atenuado).
/// Existe siempre (de ancho cero al mínimo) y su radio no depende del ancho:
/// así el estilo no cambia al arrastrar y no se reconstruyen capas.
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
        corner_radius: track.height / 2.0,
        border_width: 0.0,
        tint,
        border: tint,
    }
}

/// Tirador en cápsula blanca centrado sobre el valor, con borde fino para
/// verse sobre la tarjeta. Es un plano porque los puntos son círculos.
pub fn slider_knob_bounds(track: Rect, value: f32) -> Rect {
    let center = track.x + track.width * clamp01(value);
    Rect {
        x: center - measures::SLIDER_KNOB_WIDTH / 2.0,
        y: track.y + (track.height - measures::SLIDER_KNOB_HEIGHT) / 2.0,
        width: measures::SLIDER_KNOB_WIDTH,
        height: measures::SLIDER_KNOB_HEIGHT,
    }
}

/// Tirador con el borde sutil del tema sobre la tarjeta clara.
pub fn slider_knob_panel(
    track: Rect,
    value: f32,
    knob: dokka_scene::Color,
    edge: dokka_scene::Color,
) -> Panel {
    let bounds = slider_knob_bounds(track, value);
    Panel {
        bounds,
        corner_radius: bounds.height / 2.0,
        border_width: measures::SLIDER_KNOB_BORDER_WIDTH,
        tint: knob,
        border: edge,
    }
}

/// Barra, relleno y tirador, en ese orden y siempre los tres.
pub fn slider_panels(track: Rect, props: &SliderProps, colors: &SliderColors) -> [Panel; 3] {
    [
        slider_track_panel(track, colors),
        slider_fill_panel(track, props.value, props.active, colors),
        slider_knob_panel(track, props.value, colors.knob, colors.knob_border),
    ]
}

/// Valor bajo el punto sobre la pista, de 0 a 1. Usa la misma pista que el
/// dibujo: el centro del tirador coincide con el valor pulsado.
pub fn slider_value_at(track: Rect, x: f32) -> f32 {
    if track.width <= 0.0 {
        return 0.0;
    }
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
