//! Menú propio del clic derecho: qué opciones tiene y dónde está.

use dokka_components::menu_panel::{MenuRow, menu_row_at, menu_row_top};
use dokka_model::{AppInfo, DockApp, WindowId};
use dokka_scene::Rect;

use super::dock_menu::{MenuData, dock_menu};
use super::submenu::{OpenSubmenu, submenu_contains, submenu_panel};
use crate::Dock;

/// Lo que se pidió desde el menú.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    AddApp,
    OpenSettings,
    ShowTaskbar,
    HideTaskbar,
    TaskbarManual,
    TaskbarAuto,
    ShowDesktop,
    TaskManager,
    TaskbarSettings,
    Restart,
    Quit,
    Pin(AppInfo),
    Unpin(AppInfo),
    ActivateWindow(WindowId),
    OpenApp(AppInfo),
    RunAsAdministrator(AppInfo),
    ShowInFolder(AppInfo),
    CloseFrontWindow(Vec<WindowId>),
    QuitApp(Vec<WindowId>),
    OpenDownloads,
    OpenDocuments,
    OpenDesktop,
    OpenPictures,
    OpenTrash,
    EmptyTrash,
    OpenLaunchpad,
}

/// Menú abierto, con sus filas, su sitio y el submenú de cada madre.
#[derive(Debug, Clone)]
pub(crate) struct OpenMenu {
    pub(crate) rows: Vec<MenuRow>,
    pub(crate) actions: Vec<Option<MenuAction>>,
    pub(crate) children: Vec<Option<super::dock_menu::SubmenuData>>,
    pub(crate) panel: Rect,
    pub(crate) tip_x: f32,
    pub(crate) tip_y: f32,
    pub(crate) floor: f32,
    pub(crate) limit: f32,
    pub(crate) cross_limit: f32,
    pub(crate) position: dokka_model::DockPosition,
    pub(crate) highlighted: Option<usize>,
    pub(crate) submenu: Option<OpenSubmenu>,
}

impl Dock {
    /// Abre el menú de ese punto. Devuelve si se abrió alguno.
    pub fn open_menu(
        &mut self,
        x: f32,
        y: f32,
        apps: &[DockApp],
        taskbar_hidden: bool,
        taskbar_auto: bool,
    ) -> bool {
        let MenuData {
            rows,
            actions,
            children,
        } = match self.menu_content(x, y, apps, taskbar_hidden, taskbar_auto) {
            Some(content) => content,
            None => return false,
        };
        let placed = super::menu_place::place_menu(self, apps, x, y, &rows);
        self.pointer_left();
        self.menu = Some(OpenMenu {
            rows,
            actions,
            children,
            panel: placed.panel,
            tip_x: placed.tip_x,
            tip_y: placed.tip_y,
            floor: placed.floor,
            limit: placed.limit,
            cross_limit: placed.cross_limit,
            position: placed.position,
            highlighted: None,
            submenu: None,
        });
        true
    }

    /// Actualiza la fila resaltada bajo el cursor, abriendo submenús al pasar.
    pub fn move_over_menu(&mut self, x: f32, y: f32) {
        let Some(menu) = self.menu.as_mut() else {
            return;
        };
        if menu
            .submenu
            .as_ref()
            .is_some_and(|sub| submenu_contains(sub.panel, x, y))
        {
            if let Some(sub) = menu.submenu.as_mut() {
                sub.highlighted = menu_row_at(&sub.rows, sub.panel, x, y);
            }
            if let Some(mother) = menu.submenu.as_ref().map(|sub| sub.mother) {
                menu.highlighted = Some(mother);
            }
            return;
        }
        if let Some(index) = menu_row_at(&menu.rows, menu.panel, x, y) {
            menu.highlighted = Some(index);
            let same_mother = menu.submenu.as_ref().is_some_and(|sub| sub.mother == index);
            if same_mother {
                if let Some(sub) = menu.submenu.as_mut() {
                    sub.highlighted = None;
                }
                return;
            }
            let child = menu
                .rows
                .get(index)
                .is_some_and(|row| matches!(row, MenuRow::Item(item) if item.has_submenu));
            let data = if child {
                menu.children.get(index).and_then(|slot| slot.clone())
            } else {
                None
            };
            if let Some(data) = data {
                let top = menu_row_top(&menu.rows, menu.panel, index);
                let panel = submenu_panel(
                    menu.panel,
                    top,
                    &data.rows,
                    menu.limit,
                    menu.floor,
                    menu.position,
                    menu.cross_limit,
                );
                menu.submenu = Some(OpenSubmenu {
                    mother: index,
                    rows: data.rows,
                    actions: data.actions,
                    panel,
                    highlighted: None,
                });
            } else {
                menu.submenu = None;
            }
            return;
        }
        menu.highlighted = None;
        let bridge = menu
            .submenu
            .as_ref()
            .map(|sub| (menu_row_top(&menu.rows, menu.panel, sub.mother), sub.panel));
        let Some((mother_top, sub_panel)) = bridge else {
            return;
        };
        if super::submenu::in_bridge(menu.panel, mother_top, sub_panel, x, y) {
            if let Some(mother) = menu.submenu.as_ref().map(|sub| sub.mother) {
                menu.highlighted = Some(mother);
            }
            return;
        }
        menu.submenu = None;
    }

    pub fn close_menu(&mut self) {
        self.menu = None;
    }

    pub fn is_menu_open(&self) -> bool {
        self.menu.is_some()
    }

    fn menu_content(
        &self,
        x: f32,
        y: f32,
        apps: &[DockApp],
        taskbar_hidden: bool,
        taskbar_auto: bool,
    ) -> Option<MenuData> {
        if self.is_on_divider(x, y, apps) {
            return Some(dock_menu(taskbar_hidden, taskbar_auto));
        }
        let placement = super::place_dock(self, apps);
        match super::decide_click::entry_at(
            &placement,
            x,
            y,
            super::super::view::dock_style::gap(self),
        )? {
            super::Entry::App(app) | super::Entry::Minimized(app, _) => {
                Some(super::app_menu::app_menu(app))
            }
            super::Entry::Explorer(app) => Some(super::system_menus::explorer_menu(app)),
            super::Entry::Trash => Some(super::system_menus::trash_menu(self.icons.trash_size)),
            super::Entry::Launchpad => Some(super::system_menus::launchpad_menu()),
            super::Entry::Calendar | super::Entry::Clock => None,
        }
    }
}
