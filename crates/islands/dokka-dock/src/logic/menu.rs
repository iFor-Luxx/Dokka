//! Menú propio del clic derecho: qué opciones tiene y dónde está.

use dokka_components::menu_panel::{MenuRow, menu_overlay, menu_row_at, menu_size};
use dokka_model::{AppInfo, DockApp};
use dokka_scene::{Overlay, Rect};
use dokka_theme::menu as measures;

use crate::Dock;

/// Lo que se pidió desde el menú.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    ShowDesktop,
    TaskManager,
    TaskbarSettings,
    Quit,
    Pin(AppInfo),
    Unpin(AppInfo),
}

/// Menú abierto, con sus filas y su sitio.
#[derive(Debug, Clone)]
pub(crate) struct OpenMenu {
    rows: Vec<MenuRow>,
    actions: Vec<Option<MenuAction>>,
    panel: Rect,
    tip_x: f32,
    floor: f32,
    highlighted: Option<usize>,
}

impl Dock {
    /// Abre el menú de ese punto. Devuelve si se abrió alguno.
    pub fn open_menu(&mut self, x: f32, y: f32, apps: &[DockApp]) -> bool {
        let (rows, actions) = match self.menu_content(x, y, apps) {
            Some(content) => content,
            None => return false,
        };
        let size = menu_size(&rows);
        let limit = self.surface_size(apps).width;
        let panel_x = (x - size.width / 2.0).clamp(0.0, (limit - size.width).max(0.0));
        let bar_top = self.menu_floor(apps);
        let bottom = bar_top - measures::POINTER_HEIGHT;
        let panel = Rect {
            x: panel_x,
            y: (bottom - size.height).max(0.0),
            width: size.width,
            height: size.height,
        };
        let floor = bar_top;
        self.pointer_left();
        self.menu = Some(OpenMenu {
            rows,
            actions,
            panel,
            tip_x: x,
            floor,
            highlighted: None,
        });
        true
    }

    /// Actualiza la fila resaltada bajo el cursor.
    pub fn move_over_menu(&mut self, x: f32, y: f32) {
        let Some(menu) = self.menu.as_mut() else {
            return;
        };
        menu.highlighted = menu_row_at(&menu.rows, menu.panel, x, y);
    }

    /// Opción elegida en el punto. Cierra el menú al elegir.
    pub fn click_menu(&mut self, x: f32, y: f32) -> Option<MenuAction> {
        let menu = self.menu.as_ref()?;
        let index = menu_row_at(&menu.rows, menu.panel, x, y)?;
        let action = menu.actions[index].clone()?;
        self.menu = None;
        Some(action)
    }

    /// `true` si el punto está dentro del menú, punta incluida.
    pub fn menu_contains(&self, x: f32, y: f32) -> bool {
        let Some(menu) = &self.menu else {
            return false;
        };
        let panel = menu.panel;
        x >= panel.x
            && x <= panel.x + panel.width
            && y >= panel.y
            && y <= panel.y + panel.height + measures::POINTER_HEIGHT
    }

    pub fn close_menu(&mut self) {
        self.menu = None;
    }

    pub fn is_menu_open(&self) -> bool {
        self.menu.is_some()
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

    fn menu_floor(&self, apps: &[DockApp]) -> f32 {
        super::place_dock(self, apps).bar.y
    }

    fn menu_content(
        &self,
        x: f32,
        y: f32,
        apps: &[DockApp],
    ) -> Option<(Vec<MenuRow>, Vec<Option<MenuAction>>)> {
        if self.is_on_divider(x, y, apps) {
            return Some(dock_menu());
        }
        let app = self.pin_target(x, y, apps)?;
        let label = if app.is_pinned {
            "Quitar del dock"
        } else {
            "Fijar en el dock"
        };
        let action = if app.is_pinned {
            MenuAction::Unpin(app.info.clone())
        } else {
            MenuAction::Pin(app.info.clone())
        };
        Some((vec![MenuRow::Item(label.to_owned())], vec![Some(action)]))
    }
}

fn dock_menu() -> (Vec<MenuRow>, Vec<Option<MenuAction>>) {
    let rows = vec![
        MenuRow::Item("Mostrar el escritorio".to_owned()),
        MenuRow::Item("Administrador de tareas".to_owned()),
        MenuRow::Separator,
        MenuRow::Item("Configuración de la barra de tareas".to_owned()),
        MenuRow::Separator,
        MenuRow::Item("Salir de Dokka".to_owned()),
    ];
    let actions = vec![
        Some(MenuAction::ShowDesktop),
        Some(MenuAction::TaskManager),
        None,
        Some(MenuAction::TaskbarSettings),
        None,
        Some(MenuAction::Quit),
    ];
    (rows, actions)
}
