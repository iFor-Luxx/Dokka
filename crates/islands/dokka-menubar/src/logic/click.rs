//! Qué significa un clic: icono, hora, fila, píldora o cerrar fuera.

use dokka_components::calendar_panel::calendar_step;
use dokka_components::menu_panel::menu_row_at;

use super::tray::tray_kind_at;
use crate::{Menubar, MenubarAction};

/// Resultado interno del clic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MenubarClick {
    MenuOpened,
    CalendarOpened,
    TrayOpened,
    CalendarKept,
    TrayKept,
    Chosen(MenubarAction),
    Dismissed,
}

impl Menubar {
    /// Decide el clic. Solo uno abierto a la vez: la hora, el icono y cada
    /// icono de la fila cambian directamente de uno a otro; repetir el
    /// mismo lo cierra; fuera o el panel del dock lo cierran.
    pub fn click(&mut self, x: f32, y: f32, bar_width: f32) -> Option<MenubarAction> {
        match self.click_inner(x, y, bar_width) {
            MenubarClick::Chosen(action) => Some(action),
            MenubarClick::MenuOpened
            | MenubarClick::CalendarOpened
            | MenubarClick::TrayOpened
            | MenubarClick::CalendarKept
            | MenubarClick::TrayKept
            | MenubarClick::Dismissed => None,
        }
    }

    pub(crate) fn click_inner(&mut self, x: f32, y: f32, bar_width: f32) -> MenubarClick {
        if self.menu.is_some() {
            return self.click_over_menu(x, y, bar_width);
        }
        if self.calendar.is_some() {
            return self.click_over_calendar(x, y, bar_width);
        }
        if self.tray.is_some() {
            return self.click_over_tray(x, y, bar_width);
        }
        if in_icon(x, y, &self.icon_hit()) {
            self.open_at(bar_width);
            return MenubarClick::MenuOpened;
        }
        if in_clock(x, y, &self.clock_hit(bar_width)) {
            self.open_calendar();
            return MenubarClick::CalendarOpened;
        }
        if let Some(kind) = tray_kind_at(bar_width, x, y) {
            self.open_tray(kind, bar_width);
            return MenubarClick::TrayOpened;
        }
        MenubarClick::Dismissed
    }

    fn click_over_menu(&mut self, x: f32, y: f32, bar_width: f32) -> MenubarClick {
        if let Some(menu) = self.menu.as_ref()
            && let Some(index) = menu_row_at(&menu.rows, menu.panel, x, y)
        {
            let action = menu.actions[index];
            if let Some(action) = action {
                self.menu = None;
                self.icon_hovered = in_icon(x, y, &self.icon_hit());
                self.clock_hovered = in_clock(x, y, &self.clock_hit(bar_width));
                return MenubarClick::Chosen(action);
            }
            return MenubarClick::Dismissed;
        }
        // La hora cambia al calendario y la fila a su panel, sin cerrar.
        if in_clock(x, y, &self.clock_hit(bar_width)) {
            self.menu = None;
            self.open_calendar();
            return MenubarClick::CalendarOpened;
        }
        if let Some(kind) = tray_kind_at(bar_width, x, y) {
            self.menu = None;
            self.open_tray(kind, bar_width);
            return MenubarClick::TrayOpened;
        }
        if in_icon(x, y, &self.icon_hit()) {
            self.menu = None;
            self.icon_hovered = true;
            return MenubarClick::Dismissed;
        }
        self.menu = None;
        self.icon_hovered = false;
        self.clock_hovered = false;
        MenubarClick::Dismissed
    }

    fn click_over_calendar(&mut self, x: f32, y: f32, bar_width: f32) -> MenubarClick {
        if let Some(bounds) = self.calendar_bounds(bar_width)
            && let Some((offset, weeks)) = calendar_step(
                self.today,
                self.calendar_month_offset(),
                self.calendar_weeks_seen(),
                bounds,
                x,
                y,
            )
        {
            if let Some(open) = self.calendar.as_mut() {
                open.month_offset = offset;
                open.calendar_weeks = weeks;
            }
            return MenubarClick::CalendarKept;
        }
        // Un clic dentro de la tarjeta (fuera de la píldora) no cierra.
        if let Some(bounds) = self.calendar_bounds(bar_width)
            && contains(bounds, x, y)
        {
            return MenubarClick::CalendarKept;
        }
        // El icono cambia al menú y la fila a su panel, sin cerrar.
        if in_icon(x, y, &self.icon_hit()) {
            self.calendar = None;
            self.nav_hovered = None;
            self.open_at(bar_width);
            return MenubarClick::MenuOpened;
        }
        if let Some(kind) = tray_kind_at(bar_width, x, y) {
            self.calendar = None;
            self.nav_hovered = None;
            self.open_tray(kind, bar_width);
            return MenubarClick::TrayOpened;
        }
        if in_clock(x, y, &self.clock_hit(bar_width)) {
            self.calendar = None;
            self.nav_hovered = None;
            self.clock_hovered = true;
            return MenubarClick::Dismissed;
        }
        self.calendar = None;
        self.nav_hovered = None;
        self.icon_hovered = false;
        self.clock_hovered = false;
        MenubarClick::Dismissed
    }

    fn click_over_tray(&mut self, x: f32, y: f32, bar_width: f32) -> MenubarClick {
        // Dentro del panel provisional no se cierra (los reales deciden).
        if self.tray_contains(x, y) {
            return MenubarClick::TrayKept;
        }
        // Repetir el mismo icono lo cierra; otro cambia de panel.
        if let Some(kind) = tray_kind_at(bar_width, x, y) {
            if self.tray_kind() == Some(kind) {
                self.tray = None;
                self.tray_hovered = Some(kind);
                return MenubarClick::Dismissed;
            }
            self.open_tray(kind, bar_width);
            return MenubarClick::TrayOpened;
        }
        if in_icon(x, y, &self.icon_hit()) {
            self.tray = None;
            self.tray_hovered = None;
            self.open_at(bar_width);
            return MenubarClick::MenuOpened;
        }
        if in_clock(x, y, &self.clock_hit(bar_width)) {
            self.tray = None;
            self.tray_hovered = None;
            self.open_calendar();
            return MenubarClick::CalendarOpened;
        }
        self.tray = None;
        self.tray_hovered = None;
        self.icon_hovered = false;
        self.clock_hovered = false;
        MenubarClick::Dismissed
    }

    /// `true` si el punto cae en el menú abierto.
    pub fn menu_contains(&self, x: f32, y: f32) -> bool {
        let Some(menu) = &self.menu else {
            return false;
        };
        x >= menu.panel.x
            && x <= menu.panel.x + menu.panel.width
            && y >= menu.panel.y
            && y <= menu.panel.y + menu.panel.height
    }
}

fn in_icon(x: f32, y: f32, bounds: &dokka_scene::Rect) -> bool {
    contains(*bounds, x, y)
}

fn in_clock(x: f32, y: f32, bounds: &dokka_scene::Rect) -> bool {
    contains(*bounds, x, y)
}

fn contains(area: dokka_scene::Rect, x: f32, y: f32) -> bool {
    x >= area.x && x <= area.x + area.width && y >= area.y && y <= area.y + area.height
}
