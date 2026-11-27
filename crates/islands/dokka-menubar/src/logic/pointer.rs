//! Resaltado al mover el cursor: icono, hora y píldora del calendario.

use dokka_components::calendar_panel::calendar_hovered;
use dokka_components::menu_panel::menu_row_at;

use crate::Menubar;

impl Menubar {
    /// Actualiza el icono, la hora, la fila del menú y la píldora.
    pub fn pointer_moved(&mut self, x: f32, y: f32, bar_width: f32) {
        if self.menu.is_some() {
            self.move_over_menu(x, y, bar_width);
            return;
        }
        if self.calendar.is_some() {
            self.move_over_calendar(x, y, bar_width);
            return;
        }
        self.icon_hovered = hit(x, y, &self.icon_hit());
        self.clock_hovered = hit(x, y, &self.clock_hit(bar_width));
    }

    pub fn pointer_left(&mut self) {
        self.icon_hovered = false;
        self.clock_hovered = false;
        self.nav_hovered = None;
        if let Some(menu) = self.menu.as_mut() {
            menu.highlighted = None;
        }
    }

    fn move_over_menu(&mut self, x: f32, y: f32, bar_width: f32) {
        let Some(menu) = self.menu.as_mut() else {
            return;
        };
        if let Some(index) = menu_row_at(&menu.rows, menu.panel, x, y) {
            menu.highlighted = Some(index);
            self.icon_hovered = true;
            return;
        }
        menu.highlighted = None;
        self.icon_hovered = hit(x, y, &self.icon_hit());
        self.clock_hovered = hit(x, y, &self.clock_hit(bar_width));
    }

    fn move_over_calendar(&mut self, x: f32, y: f32, bar_width: f32) {
        self.icon_hovered = hit(x, y, &self.icon_hit());
        self.clock_hovered = hit(x, y, &self.clock_hit(bar_width));
        self.nav_hovered = self
            .calendar_bounds(bar_width)
            .and_then(|bounds| calendar_hovered(bounds, x, y));
    }
}

fn hit(x: f32, y: f32, bounds: &dokka_scene::Rect) -> bool {
    x >= bounds.x && x <= bounds.x + bounds.width && y >= bounds.y && y <= bounds.y + bounds.height
}
