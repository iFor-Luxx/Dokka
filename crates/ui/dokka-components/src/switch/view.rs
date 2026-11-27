use dokka_scene::{Panel, Rect};
use dokka_theme::{Theme, settings as measures};

use super::SwitchProps;

/// Zona de toque del interruptor, alineada a la derecha de su fila con el
/// mismo aire que el texto a la izquierda.
pub fn switch_touch(row: Rect) -> Rect {
    Rect {
        x: row.x + row.width - measures::CONTENT_PADDING - measures::SWITCH_WIDTH,
        y: row.y + (row.height - measures::SWITCH_HEIGHT) / 2.0,
        width: measures::SWITCH_WIDTH,
        height: measures::SWITCH_HEIGHT,
    }
}

/// Pista redondeada con el color de encendido o apagado del tema.
fn switch_track_panel(theme: &Theme, touch: Rect, props: &SwitchProps) -> Panel {
    let tint = if props.on {
        theme.settings.switch_on
    } else {
        theme.settings.switch_off
    };
    Panel {
        bounds: touch,
        corner_radius: touch.height / 2.0,
        border_width: 0.0,
        tint,
        border: tint,
    }
}

/// Tirador alargado en cápsula blanca, con 2 de aire al borde.
pub fn switch_knob_bounds(touch: Rect, on: bool) -> Rect {
    let x = if on {
        touch.x + touch.width - measures::SWITCH_INSET - measures::SWITCH_KNOB_WIDTH
    } else {
        touch.x + measures::SWITCH_INSET
    };
    Rect {
        x,
        y: touch.y + (touch.height - measures::SWITCH_KNOB_HEIGHT) / 2.0,
        width: measures::SWITCH_KNOB_WIDTH,
        height: measures::SWITCH_KNOB_HEIGHT,
    }
}

/// Tirador como plano: los puntos son círculos y aquí hace falta una cápsula.
fn switch_knob_panel(theme: &Theme, bounds: Rect) -> Panel {
    Panel {
        bounds,
        corner_radius: bounds.height / 2.0,
        border_width: 0.0,
        tint: theme.settings.switch_knob,
        border: theme.settings.switch_knob,
    }
}

/// Pista y tirador, en ese orden. El estilo no cambia al moverse el tirador:
/// solo su posición, así `SceneLayers` lo mueve sin reconstruir.
pub fn switch_panels(theme: &Theme, touch: Rect, props: &SwitchProps) -> [Panel; 2] {
    [
        switch_track_panel(theme, touch, props),
        switch_knob_panel(theme, switch_knob_bounds(touch, props.on)),
    ]
}

/// `true` si el punto cae en la zona de toque del interruptor.
pub fn switch_hit(touch: Rect, x: f32, y: f32) -> bool {
    (touch.x..touch.x + touch.width).contains(&x) && (touch.y..touch.y + touch.height).contains(&y)
}
