//! Sitio del submenú y paso seguro entre el menú y él.

use dokka_components::menu_panel::{MenuRow, menu_size};
use dokka_scene::Rect;
use dokka_theme::menu as measures;

/// Submenú abierto, con su madre y su sitio.
#[derive(Debug, Clone)]
pub(crate) struct OpenSubmenu {
    pub(crate) mother: usize,
    pub(crate) rows: Vec<MenuRow>,
    pub(crate) actions: Vec<Option<super::menu::MenuAction>>,
    pub(crate) panel: Rect,
    pub(crate) highlighted: Option<usize>,
}

/// Panel del submenú: a la derecha de su madre, a la izquierda si no cabe,
/// sin salirse de la ventana por abajo ni por arriba.
pub(crate) fn submenu_panel(
    main: Rect,
    mother_top: f32,
    sub_rows: &[MenuRow],
    limit: f32,
    floor: f32,
) -> Rect {
    let size = menu_size(sub_rows);
    let mut y = mother_top - measures::PADDING;
    y = y.clamp(0.0, (floor - size.height).max(0.0));
    let right = main.x + main.width + measures::SUBMENU_GAP;
    let x = if right + size.width <= limit {
        right
    } else {
        let left = main.x - measures::SUBMENU_GAP - size.width;
        left.clamp(0.0, (limit - size.width).max(0.0))
    };
    Rect {
        x,
        y,
        width: size.width,
        height: size.height,
    }
}

/// `true` si el punto va en diagonal o recto de la madre a su submenú.
pub(crate) fn in_bridge(main: Rect, mother_top: f32, sub: Rect, x: f32, y: f32) -> bool {
    let main_edge = if sub.x >= main.x + main.width {
        main.x + main.width
    } else {
        main.x
    };
    let sub_edge = if sub.x >= main.x + main.width {
        sub.x
    } else {
        sub.x + sub.width
    };
    let (left, right) = if main_edge <= sub_edge {
        (main_edge, sub_edge)
    } else {
        (sub_edge, main_edge)
    };
    let top = mother_top.min(sub.y);
    let bottom = (mother_top + measures::ROW_HEIGHT).max(sub.y + sub.height);
    x >= left && x <= right && y >= top && y <= bottom
}

/// `true` si el punto cae en el panel del submenú.
pub(crate) fn submenu_contains(panel: Rect, x: f32, y: f32) -> bool {
    x >= panel.x && x <= panel.x + panel.width && y >= panel.y && y <= panel.y + panel.height
}
