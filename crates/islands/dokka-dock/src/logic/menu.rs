//! Menú propio del clic derecho: qué opciones tiene y dónde está.

use dokka_components::menu_panel::{MenuRow, menu_row_at, menu_row_top, menu_size};
use dokka_model::{AppInfo, DockApp, WindowId};
use dokka_scene::Rect;
use dokka_theme::menu as measures;

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
    pub(crate) floor: f32,
    pub(crate) limit: f32,
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
        self.pointer_left();
        self.menu = Some(OpenMenu {
            rows,
            actions,
            children,
            panel,
            tip_x: x,
            floor: bar_top,
            limit,
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
                let panel = submenu_panel(menu.panel, top, &data.rows, menu.limit, menu.floor);
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

    fn menu_floor(&self, apps: &[DockApp]) -> f32 {
        super::place_dock(self, apps).bar.y
    }

    fn menu_content(
        &self,
        x: f32,
        y: f32,
        apps: &[DockApp],
        taskbar_hidden: bool,
        taskbar_auto: bool,
    ) -> Option<MenuData> {
        let text = self.theme.menu.text;
        if self.is_on_divider(x, y, apps) {
            return Some(dock_menu(text, taskbar_hidden, taskbar_auto));
        }
        let placement = super::place_dock(self, apps);
        match super::decide_click::entry_at(
            &placement,
            x,
            y,
            super::super::view::dock_style::gap(self),
        )? {
            super::Entry::App(app) | super::Entry::Minimized(app, _) => {
                Some(super::app_menu::app_menu(app, text))
            }
            super::Entry::Explorer(app) => Some(super::system_menus::explorer_menu(app, text)),
            super::Entry::Trash => {
                Some(super::system_menus::trash_menu(self.icons.trash_size, text))
            }
            super::Entry::Launchpad => Some(super::system_menus::launchpad_menu(text)),
            super::Entry::Calendar | super::Entry::Clock => None,
        }
    }
}
