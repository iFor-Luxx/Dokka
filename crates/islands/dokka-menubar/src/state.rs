use std::sync::Arc;

use dokka_components::menu_panel::{MenuChecks, MenuChevrons};
use dokka_icons::menubar_icon;
use dokka_scene::{Drawing, Rect};
use dokka_theme::{Theme, menubar as measures};

use super::logic::{OpenMenu, menu_actions, menu_panel_for, menu_rows};

/// Lo que la barra recuerda entre eventos: nombre, hora, menú y resaltados.
#[derive(Debug)]
pub struct Menubar {
    pub(crate) theme: Arc<Theme>,
    pub(crate) icon: Arc<Drawing>,
    pub(crate) chevrons: MenuChevrons,
    pub(crate) checks: MenuChecks,
    pub(crate) app_name: String,
    pub(crate) clock: String,
    pub(crate) icon_hovered: bool,
    pub(crate) menu: Option<OpenMenu>,
}

impl Menubar {
    pub fn new(theme: Arc<Theme>) -> Self {
        let icon = Arc::new(menubar_icon(theme.menubar.text));
        Self {
            chevrons: MenuChevrons::new(&theme),
            checks: MenuChecks::new(&theme),
            theme,
            icon,
            app_name: String::new(),
            clock: String::new(),
            icon_hovered: false,
            menu: None,
        }
    }

    pub fn set_app(&mut self, name: String) {
        self.app_name = name;
    }

    pub fn set_clock(&mut self, text: String) {
        self.clock = text;
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn clock_text(&self) -> &str {
        &self.clock
    }

    pub fn is_menu_open(&self) -> bool {
        self.menu.is_some()
    }

    pub fn is_icon_hovered(&self) -> bool {
        self.icon_hovered
    }

    pub fn highlighted(&self) -> Option<usize> {
        self.menu.as_ref().and_then(|menu| menu.highlighted)
    }

    pub fn close_menu(&mut self) {
        self.menu = None;
        self.icon_hovered = false;
    }

    /// Caja del icono: franja izquierda de la barra.
    pub(crate) fn icon_hit(&self) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width: measures::SIDE_PADDING + measures::ICON_SIDE + measures::ICON_GAP,
            height: measures::HEIGHT,
        }
    }

    /// Fondo suave detrás del icono, al pasar o con el menú abierto.
    pub(crate) fn icon_hover_bounds(&self) -> Rect {
        Rect {
            x: measures::SIDE_PADDING - measures::HOVER_PADDING,
            y: (measures::HEIGHT - measures::ICON_SIDE) / 2.0 - measures::HOVER_PADDING,
            width: measures::ICON_SIDE + measures::HOVER_PADDING * 2.0,
            height: measures::ICON_SIDE + measures::HOVER_PADDING * 2.0,
        }
    }

    pub(crate) fn open_at(&mut self, bar_width: f32) {
        let rows = menu_rows();
        let panel = menu_panel_for(&rows, bar_width);
        self.menu = Some(OpenMenu {
            rows,
            actions: menu_actions(),
            panel,
            highlighted: None,
        });
        self.icon_hovered = true;
    }
}
