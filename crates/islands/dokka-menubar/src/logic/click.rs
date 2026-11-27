//! Qué significa un clic: icono, hora, píldora o cerrar fuera.

use dokka_components::calendar_panel::calendar_step;
use dokka_components::menu_panel::menu_row_at;

use crate::{Menubar, MenubarAction};

/// Resultado interno del clic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MenubarClick {
    MenuOpened,
    CalendarOpened,
    CalendarKept,
    Chosen(MenubarAction),
    Dismissed,
}

impl Menubar {
    /// Decide el clic. La hora abre el calendario y lo cierra al repetir;
    /// la píldora navega sin cerrar; fuera cierra lo abierto.
    pub fn click(&mut self, x: f32, y: f32, bar_width: f32) -> Option<MenubarAction> {
        match self.click_inner(x, y, bar_width) {
            MenubarClick::Chosen(action) => Some(action),
            MenubarClick::MenuOpened
            | MenubarClick::CalendarOpened
            | MenubarClick::CalendarKept
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
        if in_icon(x, y, &self.icon_hit()) {
            self.open_at(bar_width);
            return MenubarClick::MenuOpened;
        }
        if in_clock(x, y, &self.clock_hit(bar_width)) {
            self.open_calendar();
            return MenubarClick::CalendarOpened;
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
        // La hora cambia al calendario sin pasar por cerrado.
        if in_clock(x, y, &self.clock_hit(bar_width)) {
            self.menu = None;
            self.open_calendar();
            return MenubarClick::CalendarOpened;
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
        // El icono cambia al menú sin pasar por cerrado.
        if in_icon(x, y, &self.icon_hit()) {
            self.calendar = None;
            self.nav_hovered = None;
            self.open_at(bar_width);
            return MenubarClick::MenuOpened;
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
