//! Qué significa un clic: abrir con el icono, elegir fila o cerrar fuera.

use dokka_components::menu_panel::menu_row_at;

use crate::{Menubar, MenubarAction};

/// Resultado interno del clic: abrir, elegir o solo cerrar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MenubarClick {
    Opened,
    Chosen(MenubarAction),
    Dismissed,
}

impl Menubar {
    /// Decide el clic. Abre con el icono, devuelve la acción elegida y
    /// cierra al elegir, al repetir en el icono o al pulsar fuera.
    pub fn click(&mut self, x: f32, y: f32, bar_width: f32) -> Option<MenubarAction> {
        match self.click_inner(x, y, bar_width) {
            MenubarClick::Chosen(action) => Some(action),
            MenubarClick::Opened | MenubarClick::Dismissed => None,
        }
    }

    pub(crate) fn click_inner(
        &mut self,
        x: f32,
        y: f32,
        bar_width: f32,
    ) -> MenubarClick {
        if self.menu.is_none() {
            if in_icon(x, y, &self.icon_hit()) {
                self.open_at(bar_width);
                return MenubarClick::Opened;
            }
            return MenubarClick::Dismissed;
        }
        if let Some(menu) = self.menu.as_ref()
            && let Some(index) = menu_row_at(&menu.rows, menu.panel, x, y)
        {
            let action = menu.actions[index];
            if let Some(action) = action {
                self.menu = None;
                self.icon_hovered = in_icon(x, y, &self.icon_hit());
                return MenubarClick::Chosen(action);
            }
            return MenubarClick::Dismissed;
        }
        if in_icon(x, y, &self.icon_hit()) {
            self.menu = None;
            self.icon_hovered = true;
            return MenubarClick::Dismissed;
        }
        self.menu = None;
        self.icon_hovered = false;
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
    x >= bounds.x
        && x <= bounds.x + bounds.width
        && y >= bounds.y
        && y <= bounds.y + bounds.height
}
