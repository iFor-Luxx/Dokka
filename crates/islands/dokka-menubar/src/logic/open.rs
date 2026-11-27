//! Cajas de la franja y apertura de sus desplegables.

use dokka_scene::Rect;
use dokka_theme::menubar as measures;

use super::{OpenCalendar, OpenMenu, menu_actions, menu_panel_for, menu_rows};

impl crate::Menubar {
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
        use dokka_components::calendar_panel::{calendar_month, calendar_weeks};

        let open = self.calendar?;
        let month = calendar_month(self.today, open.month_offset);
        Some(super::calendar_panel_for(
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
        self.calendar = None;
        self.nav_hovered = None;
        self.clock_hovered = false;
        self.close_tray();
        self.icon_hovered = true;
    }

    pub(crate) fn open_calendar(&mut self) {
        self.calendar = Some(OpenCalendar {
            month_offset: 0,
            calendar_weeks: 0,
        });
        self.menu = None;
        self.icon_hovered = false;
        self.close_tray();
        self.nav_hovered = None;
        self.clock_hovered = true;
    }

    /// Abre el panel real del icono dado, cerrando menú y calendario.
    /// Cada panel mide lo suyo y se coloca bajo su icono, salvo el buscador,
    /// que va centrado a lo ancho y separado de la barra.
    /// Los apagados en Preferencias no se abren.
    pub(crate) fn open_tray(&mut self, kind: super::TrayKind, bar_width: f32, screen_height: f32) {
        if !self.is_shown(kind) || self.tray_anchor(kind, bar_width).is_none() {
            return;
        }
        match kind {
            super::TrayKind::Volume => self.open_volume(bar_width),
            super::TrayKind::Network => self.open_network(bar_width),
            super::TrayKind::Music => self.open_music(bar_width),
            super::TrayKind::Background => self.open_background(bar_width),
            super::TrayKind::Language => self.open_language(bar_width),
            super::TrayKind::Search => self.open_search(bar_width, screen_height),
            super::TrayKind::ControlCenter => self.open_control_center(bar_width),
        }
    }

    /// Cierra menú y calendario y marca la fila como abierta.
    pub(crate) fn show_tray(&mut self, kind: super::TrayKind, panel: Rect) {
        self.tray = Some(super::OpenTray { kind, panel });
        self.menu = None;
        self.icon_hovered = false;
        self.calendar = None;
        self.nav_hovered = None;
        self.clock_hovered = false;
        self.tray_hovered = Some(kind);
        self.dragging_slider = false;
        self.dragging_brightness = false;
    }
}
