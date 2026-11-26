use dokka_scene::Dot;
use dokka_theme::{Theme, launchpad as measures};

use super::PageDotsProps;

/// Centro horizontal del punto `index`, repartidos alrededor del centro.
fn dot_center_x(props: &PageDotsProps, index: usize) -> f32 {
    let total = props.total.max(1) as f32;
    let pitch = measures::dot_pitch(props.screen.width);
    let first = props.screen.width / 2.0 - (total - 1.0) * pitch / 2.0;
    first + index as f32 * pitch
}

/// Puntos centrados en la línea de páginas; el actual en blanco.
pub fn page_dots(theme: &Theme, props: &PageDotsProps) -> Vec<Dot> {
    let total = props.total.max(1);
    let diameter = measures::dot_diameter(props.screen.width);
    let middle = measures::dots_center_y(props.screen.height);
    (0..total)
        .map(|index| {
            let color = if index == props.current.min(total - 1) {
                theme.launchpad.dot_active
            } else {
                theme.launchpad.dot_inactive
            };
            Dot {
                x: dot_center_x(props, index) - diameter / 2.0,
                y: middle - diameter / 2.0,
                diameter,
                color,
            }
        })
        .collect()
}

/// Qué punto hay en el punto, con una zona amplia para acertar con el mouse.
pub fn dot_at(props: &PageDotsProps, x: f32, y: f32) -> Option<usize> {
    let total = props.total.max(1);
    let middle = measures::dots_center_y(props.screen.height);
    let half = measures::DOT_HIT_SIDE / 2.0;
    (0..total).find(|&index| {
        let center = dot_center_x(props, index);
        x >= center - half && x < center + half && y >= middle - half && y < middle + half
    })
}
