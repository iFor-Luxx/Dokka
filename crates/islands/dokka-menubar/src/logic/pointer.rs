//! Resaltado al mover el cursor: icono, hora, fila, píldora y paneles.

use dokka_components::calendar_panel::calendar_hovered;
use dokka_components::menu_panel::menu_row_at;

use super::tray::tray_kind_at;
use crate::Menubar;

impl Menubar {
    pub fn pointer_left(&mut self) {
        self.icon_hovered = false;
        self.clock_hovered = false;
        self.nav_hovered = None;
        self.tray_hovered = None;
        self.volume_hovered = None;
        self.network_hovered = None;
        self.language_hovered = None;
        if let Some(menu) = self.menu.as_mut() {
            menu.highlighted = None;
        }
    }

    pub(super) fn move_over_menu(&mut self, x: f32, y: f32, bar_width: f32) {
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
        self.tray_hovered = tray_kind_at(bar_width, x, y);
    }

    pub(super) fn move_over_calendar(&mut self, x: f32, y: f32, bar_width: f32) {
        self.icon_hovered = hit(x, y, &self.icon_hit());
        self.clock_hovered = hit(x, y, &self.clock_hit(bar_width));
        self.tray_hovered = tray_kind_at(bar_width, x, y);
        self.nav_hovered = self
            .calendar_bounds(bar_width)
            .and_then(|bounds| calendar_hovered(bounds, x, y));
    }

    pub(super) fn move_over_tray(&mut self, x: f32, y: f32, bar_width: f32) {
        self.icon_hovered = hit(x, y, &self.icon_hit());
        self.clock_hovered = hit(x, y, &self.clock_hit(bar_width));
        self.tray_hovered = tray_kind_at(bar_width, x, y);
        match self.tray_kind() {
            Some(super::TrayKind::Volume) => self.volume_hover(x, y),
            Some(super::TrayKind::Network) => self.network_hover(x, y),
            Some(super::TrayKind::Language) => self.language_hover(x, y),
            Some(super::TrayKind::Search) => self.search_hover(x, y),
            Some(super::TrayKind::Music) | Some(super::TrayKind::ControlCenter) | None => {}
        }
    }
}

pub(super) fn hit(x: f32, y: f32, bounds: &dokka_scene::Rect) -> bool {
    x >= bounds.x && x <= bounds.x + bounds.width && y >= bounds.y && y <= bounds.y + bounds.height
}
