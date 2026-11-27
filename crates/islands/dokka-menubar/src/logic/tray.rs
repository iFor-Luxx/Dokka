//! Fila de iconos a la izquierda de la fecha y colocación de sus paneles.
//!
//! Cada panel real mide lo suyo con su `<x>_size`; aquí solo se centra bajo
//! su icono sin salirse del ancho. El buscador, más ancho, va centrado.
//! Solo los iconos encendidos en Preferencias tienen caja: los apagados no
//! se dibujan ni reciben clics y los demás se recolocan sin huecos.

use dokka_model::MenubarItem;
use dokka_scene::{Rect, Size};
use dokka_theme::{menubar, menubar_tray as measures};

/// Icono de la fila, de derecha a izquierda en la barra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayKind {
    ControlCenter,
    Search,
    Language,
    Background,
    Music,
    Network,
    Volume,
}

impl TrayKind {
    /// Icono de la barra que es este icono ordenable.
    pub(crate) fn from_item(item: MenubarItem) -> Self {
        match item {
            MenubarItem::Volume => Self::Volume,
            MenubarItem::Network => Self::Network,
            MenubarItem::Music => Self::Music,
            MenubarItem::BackgroundApps => Self::Background,
            MenubarItem::Language => Self::Language,
            MenubarItem::Search => Self::Search,
            MenubarItem::ControlCenter => Self::ControlCenter,
        }
    }

    /// Orden guardado en Preferencias, de izquierda a derecha en la barra.
    pub(crate) fn saved_order(shown: &dokka_model::MenubarPreferences) -> [Self; 7] {
        let order = shown.order;
        [
            Self::from_item(order[0]),
            Self::from_item(order[1]),
            Self::from_item(order[2]),
            Self::from_item(order[3]),
            Self::from_item(order[4]),
            Self::from_item(order[5]),
            Self::from_item(order[6]),
        ]
    }

    fn cell_width(self) -> f32 {
        match self {
            Self::Language => measures::LANG_WIDTH,
            Self::ControlCenter => measures::CONTROL_CELL,
            _ => measures::CELL,
        }
    }
}

/// Panel abierto: qué icono lo abrió y dónde cae.
#[derive(Debug, Clone, Copy)]
pub(crate) struct OpenTray {
    pub(crate) kind: TrayKind,
    pub(crate) panel: Rect,
}

/// Panel de este tamaño bajo su icono, sin salirse del ancho.
pub(crate) fn tray_place(anchor: Rect, size: Size, bar_width: f32) -> Rect {
    let limit = bar_width.max(size.width);
    let center = anchor.x + anchor.width / 2.0;
    let x = (center - size.width / 2.0).clamp(0.0, limit - size.width);
    Rect {
        x,
        y: menubar::HEIGHT + menubar::MENU_GAP,
        width: size.width,
        height: size.height,
    }
}

/// Panel centrado en lo alto, para el buscador ancho.
pub(crate) fn tray_place_centered(size: Size, bar_width: f32) -> Rect {
    let x = ((bar_width - size.width) / 2.0).max(0.0);
    Rect {
        x,
        y: menubar::HEIGHT + menubar::MENU_GAP,
        width: size.width,
        height: size.height,
    }
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x <= area.x + area.width && y >= area.y && y <= area.y + area.height
}

impl crate::Menubar {
    /// Iconos visibles según Preferencias, de izquierda a derecha.
    pub(crate) fn visible_kinds(&self) -> Vec<TrayKind> {
        TrayKind::saved_order(&self.shown)
            .into_iter()
            .filter(|kind| self.is_shown(*kind))
            .collect()
    }

    /// Cajas pulsables de la fila, a la izquierda de la fecha.
    /// Solo las visibles, sin huecos: se colocan desde la fecha hacia la izquierda.
    pub(crate) fn tray_cells(&self, bar_width: f32) -> Vec<(TrayKind, Rect)> {
        let mut right =
            bar_width - menubar::SIDE_PADDING - menubar::CLOCK_SLOT - measures::CLOCK_GAP;
        let mut cells = Vec::new();
        for kind in self.visible_kinds().into_iter().rev() {
            let width = kind.cell_width();
            let left = right - width;
            cells.push((
                kind,
                Rect {
                    x: left,
                    y: 0.0,
                    width,
                    height: menubar::HEIGHT,
                },
            ));
            right = left;
        }
        cells.reverse();
        cells
    }

    /// Icono visible que hay en el punto, si es de la fila.
    pub(crate) fn tray_kind_at(&self, bar_width: f32, x: f32, y: f32) -> Option<TrayKind> {
        self.tray_cells(bar_width)
            .into_iter()
            .find(|(_, area)| contains(*area, x, y))
            .map(|(kind, _)| kind)
    }

    /// Caja del icono dado, para colocar su panel al abrirlo.
    /// Los apagados no tienen caja.
    pub(crate) fn tray_anchor(&self, kind: TrayKind, bar_width: f32) -> Option<Rect> {
        if !self.is_shown(kind) {
            return None;
        }
        self.tray_cells(bar_width)
            .into_iter()
            .find(|(cell, _)| *cell == kind)
            .map(|(_, area)| area)
    }

    /// Fondo suave del icono de la fila, al pasar o con su panel abierto.
    pub(crate) fn tray_hover_bounds(&self, kind: TrayKind, bar_width: f32) -> Rect {
        let Some((_, area)) = self
            .tray_cells(bar_width)
            .into_iter()
            .find(|(cell, _)| *cell == kind)
        else {
            return Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            };
        };
        Rect {
            x: area.x + (area.width - measures::ICON_SIDE) / 2.0 - measures::HOVER_PADDING,
            y: (menubar::HEIGHT - measures::ICON_SIDE) / 2.0 - measures::HOVER_PADDING,
            width: measures::ICON_SIDE + measures::HOVER_PADDING * 2.0,
            height: measures::ICON_SIDE + measures::HOVER_PADDING * 2.0,
        }
    }

    /// Rectángulo del panel abierto, si lo hay.
    pub(crate) fn tray_bounds(&self) -> Option<Rect> {
        self.tray.map(|open| open.panel)
    }

    /// `true` si el punto cae en el panel abierto.
    pub fn tray_contains(&self, x: f32, y: f32) -> bool {
        let Some(panel) = self.tray_bounds() else {
            return false;
        };
        x >= panel.x && x <= panel.x + panel.width && y >= panel.y && y <= panel.y + panel.height
    }
}
