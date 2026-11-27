use std::sync::Arc;

use dokka_components::calendar_panel::{calendar_month, calendar_weeks};
use dokka_components::menu_panel::{MenuChecks, MenuChevrons};
use dokka_components::nav_pill::{NavHalf, NavPillIcons};
use dokka_icons::menubar_icon;
use dokka_scene::{Drawing, Rect};
use dokka_theme::{Theme, menubar as measures};

use super::logic::{
    OpenCalendar, OpenMenu, calendar_panel_for, menu_actions, menu_panel_for, menu_rows,
};

/// Huella de la barra ya dibujada: app, hora, hoy, aperturas, resaltados,
/// mes y píldora. Vive aquí para no exponer los tipos del componente.
#[derive(Debug, Clone, PartialEq)]
pub struct MenubarPainted {
    pub app: String,
    pub clock: String,
    pub today: (u16, u16, u16),
    pub menu_open: bool,
    pub icon_hovered: bool,
    pub highlighted: Option<usize>,
    pub calendar_open: bool,
    pub month_offset: i32,
    pub weeks_seen: u16,
    pub nav_hovered: Option<NavHalf>,
    pub clock_hovered: bool,
}

/// Lo que la barra recuerda entre eventos: nombre, hora, menú, calendario y resaltados.
#[derive(Debug)]
pub struct Menubar {
    pub(crate) theme: Arc<Theme>,
    pub(crate) icon: Arc<Drawing>,
    pub(crate) chevrons: MenuChevrons,
    pub(crate) checks: MenuChecks,
    pub(crate) nav_pill: NavPillIcons,
    pub(crate) app_name: String,
    pub(crate) clock: String,
    pub(crate) icon_hovered: bool,
    pub(crate) clock_hovered: bool,
    pub(crate) nav_hovered: Option<NavHalf>,
    pub(crate) menu: Option<OpenMenu>,
    pub(crate) calendar: Option<OpenCalendar>,
    pub(crate) today: (u16, u16, u16),
}

impl Menubar {
    pub fn new(theme: Arc<Theme>) -> Self {
        let icon = Arc::new(menubar_icon(theme.menubar.text));
        Self {
            chevrons: MenuChevrons::new(&theme),
            checks: MenuChecks::new(&theme),
            nav_pill: NavPillIcons::new(&theme),
            theme,
            icon,
            app_name: String::new(),
            clock: String::new(),
            icon_hovered: false,
            clock_hovered: false,
            nav_hovered: None,
            menu: None,
            calendar: None,
            today: (0, 1, 1),
        }
    }

    pub fn set_app(&mut self, name: String) {
        self.app_name = name;
    }

    pub fn set_clock(&mut self, text: String) {
        self.clock = text;
    }

    pub fn set_today(&mut self, year: u16, month: u16, day: u16) {
        self.today = (year, month, day);
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

    pub fn is_calendar_open(&self) -> bool {
        self.calendar.is_some()
    }

    pub fn is_icon_hovered(&self) -> bool {
        self.icon_hovered
    }

    pub(crate) fn calendar_month_offset(&self) -> i32 {
        self.calendar.map(|open| open.month_offset).unwrap_or(0)
    }

    pub(crate) fn calendar_weeks_seen(&self) -> u16 {
        self.calendar.map(|open| open.calendar_weeks).unwrap_or(0)
    }

    pub fn highlighted(&self) -> Option<usize> {
        self.menu.as_ref().and_then(|menu| menu.highlighted)
    }

    /// Huella de lo dibujado (app, hora, hoy, aperturas, resaltados, mes y
    /// píldora): el shell solo repinta si cambia.
    pub fn painted(&self) -> MenubarPainted {
        MenubarPainted {
            app: self.app_name.clone(),
            clock: self.clock.clone(),
            today: self.today,
            menu_open: self.menu.is_some(),
            icon_hovered: self.icon_hovered,
            highlighted: self.highlighted(),
            calendar_open: self.calendar.is_some(),
            month_offset: self.calendar_month_offset(),
            weeks_seen: self.calendar_weeks_seen(),
            nav_hovered: self.nav_hovered,
            clock_hovered: self.clock_hovered,
        }
    }

    pub fn close_menu(&mut self) {
        self.menu = None;
        self.icon_hovered = false;
    }

    pub fn close_calendar(&mut self) {
        self.calendar = None;
        self.nav_hovered = None;
        self.clock_hovered = false;
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

    /// Caja de la hora: hueco derecho de la barra.
    pub(crate) fn clock_hit(&self, bar_width: f32) -> Rect {
        Rect {
            x: bar_width - measures::SIDE_PADDING - measures::CLOCK_SLOT,
            y: 0.0,
            width: measures::CLOCK_SLOT,
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

    /// Fondo suave detrás de la hora, al pasar o con el calendario abierto.
    pub(crate) fn clock_hover_bounds(&self, bar_width: f32) -> Rect {
        Rect {
            x: bar_width - measures::SIDE_PADDING - measures::CLOCK_SLOT - measures::HOVER_PADDING,
            y: (measures::HEIGHT - measures::ICON_SIDE) / 2.0 - measures::HOVER_PADDING,
            width: measures::CLOCK_SLOT + measures::HOVER_PADDING * 2.0,
            height: measures::ICON_SIDE + measures::HOVER_PADDING * 2.0,
        }
    }

    /// Rectángulo del calendario abierto, con el recuerdo de semanas.
    pub(crate) fn calendar_bounds(&self, bar_width: f32) -> Option<Rect> {
        let open = self.calendar?;
        let month = calendar_month(self.today, open.month_offset);
        Some(calendar_panel_for(
            bar_width,
            calendar_weeks(&month).max(open.calendar_weeks),
        ))
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

    pub(crate) fn open_calendar(&mut self) {
        self.calendar = Some(OpenCalendar {
            month_offset: 0,
            calendar_weeks: 0,
        });
        self.nav_hovered = None;
        self.clock_hovered = true;
    }
}
