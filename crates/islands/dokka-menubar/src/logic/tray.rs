//! Fila de iconos a la izquierda de la fecha y colocación de sus paneles.
//!
//! Cada panel real mide lo suyo con su `<x>_size`; aquí solo se centra bajo
//! su icono sin salirse del ancho. El buscador, más ancho, va centrado.

use dokka_scene::{Rect, Size};
use dokka_theme::{menubar, menubar_tray as measures};

/// Icono de la fila, de derecha a izquierda en la barra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayKind {
    ControlCenter,
    Search,
    Language,
    Music,
    Network,
    Volume,
}

impl TrayKind {
    /// De izquierda a derecha en la barra (al revés del enunciado).
    pub(crate) fn order() -> [Self; 6] {
        [
            Self::Volume,
            Self::Network,
            Self::Music,
            Self::Language,
            Self::Search,
            Self::ControlCenter,
        ]
    }

    fn cell_width(self) -> f32 {
        match self {
            Self::Language => measures::LANG_WIDTH,
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

/// Cajas pulsables de la fila, a la izquierda del hueco de la fecha.
pub(crate) fn tray_cells(bar_width: f32) -> Vec<(TrayKind, Rect)> {
    let mut right = bar_width - menubar::SIDE_PADDING - menubar::CLOCK_SLOT - measures::CLOCK_GAP;
    let mut cells = Vec::with_capacity(6);
    for kind in TrayKind::order().into_iter().rev() {
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

/// Icono que hay en el punto, si es de la fila.
pub(crate) fn tray_kind_at(bar_width: f32, x: f32, y: f32) -> Option<TrayKind> {
    tray_cells(bar_width)
        .into_iter()
        .find(|(_, area)| contains(*area, x, y))
        .map(|(kind, _)| kind)
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

/// Caja del icono dado, para colocar su panel al abrirlo.
pub(crate) fn tray_anchor(kind: TrayKind, bar_width: f32) -> Option<Rect> {
    tray_cells(bar_width)
        .into_iter()
        .find(|(cell, _)| *cell == kind)
        .map(|(_, area)| area)
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x <= area.x + area.width && y >= area.y && y <= area.y + area.height
}

impl crate::Menubar {
    /// Cajas pulsables de la fila, a la izquierda de la fecha.
    pub(crate) fn tray_cells(&self, bar_width: f32) -> Vec<(TrayKind, Rect)> {
        tray_cells(bar_width)
    }

    /// Fondo suave del icono de la fila, al pasar o con su panel abierto.
    pub(crate) fn tray_hover_bounds(&self, kind: TrayKind, bar_width: f32) -> Rect {
        let Some((_, area)) = tray_cells(bar_width)
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
