use dokka_scene::Dot;
use dokka_theme::{Theme, settings as measures};

use super::TrafficButton;

/// Los tres círculos rojo, amarillo y verde de la esquina superior izquierda.
/// Grises cuando la ventana no tiene el foco, como en macOS.
pub fn traffic_dots(theme: &Theme, focused: bool) -> Vec<Dot> {
    let colors = if focused {
        theme.settings.traffic
    } else {
        [theme.settings.traffic_inactive; 3]
    };
    colors
        .into_iter()
        .enumerate()
        .map(|(index, color)| Dot {
            x: measures::TRAFFIC_LEFT + index as f32 * measures::TRAFFIC_PITCH,
            y: measures::TRAFFIC_TOP,
            diameter: measures::TRAFFIC_DIAMETER,
            color,
        })
        .collect()
}

/// Botón que hay en el punto, con aire alrededor de cada círculo.
pub fn traffic_at(x: f32, y: f32) -> Option<TrafficButton> {
    let buttons = [
        TrafficButton::Close,
        TrafficButton::Minimize,
        TrafficButton::Zoom,
    ];
    buttons.into_iter().enumerate().find_map(|(index, button)| {
        let left = measures::TRAFFIC_LEFT + index as f32 * measures::TRAFFIC_PITCH;
        let center_x = left + measures::TRAFFIC_DIAMETER / 2.0;
        let center_y = measures::TRAFFIC_TOP + measures::TRAFFIC_DIAMETER / 2.0;
        let reach = measures::TRAFFIC_DIAMETER / 2.0 + measures::TRAFFIC_HIT_PADDING;
        if (x - center_x).abs() <= reach && (y - center_y).abs() <= reach {
            Some(button)
        } else {
            None
        }
    })
}
