use dokka_scene::Drawing;
use dokka_theme::IconColors;

use crate::shapes::{circle, rounded};

/// Papelera, vacía o con contenido asomando.
pub fn trash_icon(colors: &IconColors, has_items: bool) -> Drawing {
    let mut shapes = Vec::new();
    if has_items {
        shapes.push(circle(34.0, 22.0, 13.0, colors.trash_paper));
        shapes.push(circle(52.0, 18.0, 15.0, colors.trash_waste));
        shapes.push(circle(66.0, 24.0, 11.0, colors.trash_paper));
    }
    shapes.push(rounded(22.0, 26.0, 56.0, 68.0, 10.0, colors.trash_body));
    shapes.push(rounded(18.0, 23.0, 64.0, 9.0, 4.0, colors.trash_rim));
    let ribs = [36.0, 50.0, 64.0];
    shapes.extend(ribs.map(|x| rounded(x - 2.0, 42.0, 4.0, 40.0, 2.0, colors.trash_rib)));
    Drawing { shapes }
}
