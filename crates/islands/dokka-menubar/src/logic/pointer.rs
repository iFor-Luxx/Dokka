//! Resaltado al mover el cursor: icono a la izquierda y filas del menú.

use dokka_components::menu_panel::menu_row_at;

use crate::Menubar;

impl Menubar {
    /// Actualiza el icono y la fila bajo el cursor.
    pub fn pointer_moved(&mut self, x: f32, y: f32) {
        let Some(menu) = self.menu.as_mut() else {
            self.icon_hovered = hit_icon(x, y, &self.icon_hit());
            return;
        };
        if let Some(index) = menu_row_at(&menu.rows, menu.panel, x, y) {
            menu.highlighted = Some(index);
            self.icon_hovered = true;
            return;
        }
        menu.highlighted = None;
        self.icon_hovered = hit_icon(x, y, &self.icon_hit());
    }

    pub fn pointer_left(&mut self) {
        self.icon_hovered = false;
        if let Some(menu) = self.menu.as_mut() {
            menu.highlighted = None;
        }
    }
}

fn hit_icon(x: f32, y: f32, bounds: &dokka_scene::Rect) -> bool {
    x >= bounds.x
        && x <= bounds.x + bounds.width
        && y >= bounds.y
        && y <= bounds.y + bounds.height
}
