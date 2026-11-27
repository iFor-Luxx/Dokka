use dokka_scene::{Dot, Panel, Rect};
use dokka_theme::{Theme, settings as measures};

use super::SwitchProps;

/// Zona de toque del interruptor, alineada a la derecha de su fila.
pub fn switch_touch(row: Rect) -> Rect {
    Rect {
        x: row.x + row.width - measures::SWITCH_WIDTH,
        y: row.y + (row.height - measures::SWITCH_HEIGHT) / 2.0,
        width: measures::SWITCH_WIDTH,
        height: measures::SWITCH_HEIGHT,
    }
}

/// Pista redondeada con el color de encendido o apagado del tema.
pub fn switch_panels(theme: &Theme, touch: Rect, props: &SwitchProps) -> [Panel; 1] {
    let tint = if props.on {
        theme.settings.switch_on
    } else {
        theme.settings.switch_off
    };
    [Panel {
        bounds: touch,
        corner_radius: touch.height / 2.0,
        border_width: 0.0,
        tint,
        border: tint,
    }]
}

/// Tirador blanco a la izquierda o a la derecha de la pista.
pub fn switch_dot(theme: &Theme, touch: Rect, props: &SwitchProps) -> Dot {
    let x = if props.on {
        touch.x + touch.width - measures::SWITCH_INSET - measures::SWITCH_KNOB
    } else {
        touch.x + measures::SWITCH_INSET
    };
    Dot {
        x,
        y: touch.y + (touch.height - measures::SWITCH_KNOB) / 2.0,
        diameter: measures::SWITCH_KNOB,
        color: theme.settings.switch_knob,
    }
}

/// `true` si el punto cae en la zona de toque del interruptor.
pub fn switch_hit(touch: Rect, x: f32, y: f32) -> bool {
    (touch.x..touch.x + touch.width).contains(&x)
        && (touch.y..touch.y + touch.height).contains(&y)
}
