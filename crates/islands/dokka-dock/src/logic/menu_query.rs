//! Lo elegido en el menú y dónde dibujarlo.

use dokka_components::menu_panel::{menu_overlay, menu_row_at, menu_row_top, submenu_overlay};
use dokka_scene::{Overlay, Rect};
use dokka_theme::menu as measures;

use super::submenu::{in_bridge, submenu_contains};
use crate::Dock;

impl Dock {
    /// Opción elegida en el punto. Cierra el menú al elegir.
    pub fn click_menu(&mut self, x: f32, y: f32) -> Option<super::menu::MenuAction> {
        let menu = self.menu.as_ref()?;
        let sub_hit = menu.submenu.as_ref().and_then(|sub| {
            if submenu_contains(sub.panel, x, y) {
                menu_row_at(&sub.rows, sub.panel, x, y)
                    .and_then(|index| sub.actions[index].clone().map(|action| (index, action)))
            } else {
                None
            }
        });
        if let Some((_, action)) = sub_hit {
            self.menu = None;
            return Some(action);
        }
        if menu.submenu.as_ref().is_some_and(|sub| {
            submenu_contains(sub.panel, x, y)
                || in_bridge(menu.panel, sub_top(menu, sub.mother), sub.panel, x, y)
        }) {
            return None;
        }
        let index = menu_row_at(&menu.rows, menu.panel, x, y)?;
        let action = menu.actions[index].clone()?;
        self.menu = None;
        Some(action)
    }

    /// `true` si el punto está dentro del menú o su submenú, punta incluida.
    pub fn menu_contains(&self, x: f32, y: f32) -> bool {
        let Some(menu) = &self.menu else {
            return false;
        };
        if panel_contains(menu.panel, x, y) {
            return true;
        }
        let panel = menu.panel;
        if x >= panel.x
            && x <= panel.x + panel.width
            && y >= panel.y
            && y <= panel.y + panel.height + measures::POINTER_HEIGHT
        {
            return true;
        }
        let Some(sub) = &menu.submenu else {
            return false;
        };
        if submenu_contains(sub.panel, x, y) {
            return true;
        }
        let mother_top = menu_row_top(&menu.rows, menu.panel, sub.mother);
        in_bridge(menu.panel, mother_top, sub.panel, x, y)
    }

    /// Dibujo del menú abierto.
    pub fn menu_overlay(&self) -> Option<Overlay> {
        let menu = self.menu.as_ref()?;
        Some(menu_overlay(
            &self.theme,
            menu.panel,
            menu.tip_x,
            &menu.rows,
            menu.highlighted,
        ))
    }

    /// Dibujo del submenú, sin punta.
    pub fn submenu_overlay(&self) -> Option<Overlay> {
        let sub = self.menu.as_ref()?.submenu.as_ref()?;
        Some(submenu_overlay(
            &self.theme,
            sub.panel,
            &sub.rows,
            sub.highlighted,
        ))
    }

    /// Zona que debe verse y recibir el mouse: el menú y el hueco hasta el dock.
    pub fn menu_area(&self) -> Option<Rect> {
        let menu = self.menu.as_ref()?;
        Some(Rect {
            x: menu.panel.x,
            y: menu.panel.y,
            width: menu.panel.width,
            height: (menu.floor - menu.panel.y).max(menu.panel.height),
        })
    }

    /// Zona del submenú, con el hueco hasta el menú para no perder el mouse.
    pub fn submenu_area(&self) -> Option<Rect> {
        let menu = self.menu.as_ref()?;
        let sub = menu.submenu.as_ref()?;
        let gap = measures::SUBMENU_GAP;
        if sub.panel.x >= menu.panel.x + menu.panel.width {
            Some(Rect {
                x: menu.panel.x + menu.panel.width,
                y: sub.panel.y,
                width: sub.panel.width + gap,
                height: sub.panel.height,
            })
        } else {
            Some(Rect {
                x: sub.panel.x,
                y: sub.panel.y,
                width: sub.panel.width + gap,
                height: sub.panel.height,
            })
        }
    }
}

fn sub_top(menu: &super::menu::OpenMenu, mother: usize) -> f32 {
    menu_row_top(&menu.rows, menu.panel, mother)
}

fn panel_contains(panel: Rect, x: f32, y: f32) -> bool {
    x >= panel.x && x <= panel.x + panel.width && y >= panel.y && y <= panel.y + panel.height
}
