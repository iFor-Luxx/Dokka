use dokka_theme::settings as measures;

use super::TrafficButton;

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
