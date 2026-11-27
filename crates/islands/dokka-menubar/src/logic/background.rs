//! Panel de apps en segundo plano: lista real, apertura y elección.
//!
//! La isla guarda la lista que le pasa el shell (ya con nombres limpios);
//! el shell la relee cada `Tick` y solo la pasa si cambió la revisión del
//! `TrayHost`. El panel muestra como máximo las que quepan en la ventana.

use dokka_components::background_panel::{
    BackgroundApp, BackgroundHit, background_hit, background_panel, background_size,
};
use dokka_scene::Overlay;
use dokka_theme::background as measures;

use super::tray::{tray_anchor, tray_place};
use crate::{Menubar, MenubarAction};

impl Menubar {
    /// Apps en segundo plano para el panel; recoloca el alto si está abierto.
    /// Sin ancho a la vista: el panel mantiene su `x` y solo cambia de alto
    /// (el ancho es fijo), así no hace falta la barra para reubicarlo.
    pub fn set_background_apps(&mut self, apps: Vec<BackgroundApp>) {
        self.background_apps = apps;
        if let Some(hovered) = self.background_hovered {
            let still = match hovered {
                BackgroundHit::Row(id) => self.background_apps.iter().any(|app| app.id == id),
            };
            if !still {
                self.background_hovered = None;
            }
        }
        if self.tray_kind() == Some(super::TrayKind::Background) {
            let visible = measures::visible_rows(self.background_apps.len());
            let height = measures::panel_height(visible);
            self.tray = self.tray.map(|open| super::OpenTray {
                panel: dokka_scene::Rect {
                    x: open.panel.x,
                    y: open.panel.y,
                    width: background_size(self.background_apps.len()).width,
                    height,
                },
                ..open
            });
        }
    }

    /// Mide con su tamaño real y lo coloca bajo su icono.
    pub fn open_background(&mut self, bar_width: f32) {
        let Some(anchor) = tray_anchor(super::TrayKind::Background, bar_width) else {
            return;
        };
        let size = background_size(self.background_apps.len());
        self.show_tray(
            super::TrayKind::Background,
            tray_place(anchor, size, bar_width),
        );
        self.background_hovered = None;
    }

    pub(crate) fn background_overlay(&self) -> Overlay {
        let bounds = self.tray_bounds().unwrap_or(dokka_scene::Rect {
            x: 0.0,
            y: dokka_theme::menubar::HEIGHT,
            width: background_size(self.background_apps.len()).width,
            height: background_size(self.background_apps.len()).height,
        });
        background_panel(
            &self.theme,
            bounds,
            &self.background_apps,
            self.background_hovered,
        )
    }

    /// Clic izquierdo en una fila: activar esa app.
    pub(crate) fn background_click(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match background_hit(bounds, x, y, &self.background_apps) {
            Some(BackgroundHit::Row(id)) => Some(MenubarAction::TrayActivate(id)),
            None => None,
        }
    }

    /// Clic derecho en una fila: el menú propio de esa app.
    pub(crate) fn background_right_click(&mut self, x: f32, y: f32) -> Option<MenubarAction> {
        let bounds = self.tray_bounds()?;
        match background_hit(bounds, x, y, &self.background_apps) {
            Some(BackgroundHit::Row(id)) => Some(MenubarAction::TrayMenu(id)),
            None => None,
        }
    }

    /// Clic derecho en la fila: en una app pide su menú; en el botón,
    /// abre el panel como el clic izquierdo.
    pub fn background_right_clicked(
        &mut self,
        x: f32,
        y: f32,
        bar_width: f32,
    ) -> Option<MenubarAction> {
        if self.tray_kind() == Some(super::TrayKind::Background) && self.tray_contains(x, y) {
            return self.background_right_click(x, y);
        }
        if super::tray::tray_kind_at(bar_width, x, y) == Some(super::TrayKind::Background) {
            self.open_background(bar_width);
        }
        None
    }

    pub(crate) fn background_hover(&mut self, x: f32, y: f32) {
        self.background_hovered = self
            .tray_bounds()
            .and_then(|bounds| background_hit(bounds, x, y, &self.background_apps));
    }
}
