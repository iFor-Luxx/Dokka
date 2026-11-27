//! Fila de iconos a la izquierda de la fecha y su panel provisional.
//!
//! PROVISIONAL: cada icono abre una tarjeta vacía con su título hasta que
//! llegue su componente. El enganche real vive aquí, un archivo por panel
//! (ver `README.md`): `open_<panel>` usará `<x>_size` para medir,
//! `<x>_panel` para dibujar y `<x>_hit` para el clic, con su `<X>Icons`.

use dokka_components::popup_panel::popup_panel;
use dokka_scene::{Overlay, Rect, Text, TextAlign};
use dokka_theme::{Theme, menubar, menubar_tray as measures, typography};

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
    /// Título del panel provisional; el real pondrá el suyo.
    pub fn title(self) -> &'static str {
        match self {
            Self::ControlCenter => "Centro de control",
            Self::Search => "Buscar",
            Self::Language => "Idioma",
            Self::Music => "Música",
            Self::Network => "Red",
            Self::Volume => "Volumen",
        }
    }

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

/// Panel provisional abierto: qué icono lo abrió y dónde cae.
#[derive(Debug, Clone, Copy)]
pub(crate) struct OpenTray {
    pub(crate) kind: TrayKind,
    pub(crate) panel: Rect,
}

/// Cajas pulsables de la fila, a la izquierda del hueco de la fecha.
pub(crate) fn tray_cells(bar_width: f32) -> Vec<(TrayKind, Rect)> {
    let mut right = bar_width
        - menubar::SIDE_PADDING
        - menubar::CLOCK_SLOT
        - measures::CLOCK_GAP;
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

/// Panel provisional bajo su icono, sin salirse del ancho.
pub(crate) fn tray_panel_for(anchor: Rect, bar_width: f32) -> Rect {
    let limit = bar_width.max(measures::PANEL_WIDTH);
    let center = anchor.x + anchor.width / 2.0;
    let x = (center - measures::PANEL_WIDTH / 2.0).clamp(0.0, limit - measures::PANEL_WIDTH);
    Rect {
        x,
        y: menubar::HEIGHT + menubar::MENU_GAP,
        width: measures::PANEL_WIDTH,
        height: measures::PANEL_HEIGHT,
    }
}

/// Caja del panel para el icono dado (para abrirlo desde el clic).
pub(crate) fn tray_open_panel(kind: TrayKind, bar_width: f32) -> Option<Rect> {
    let anchor = tray_cells(bar_width)
        .into_iter()
        .find(|(cell, _)| *cell == kind)
        .map(|(_, area)| area)?;
    Some(tray_panel_for(anchor, bar_width))
}

/// Tarjeta provisional vacía con su título. Los paneles reales la
/// sustituyen llamando a su `<x>_panel` con el mismo `bounds`.
pub(crate) fn tray_placeholder_overlay(theme: &Theme, bounds: Rect, kind: TrayKind) -> Overlay {
    Overlay {
        panel: popup_panel(theme, bounds),
        pill: None,
        highlight: None,
        flat_panels: Vec::new(),
        dividers: Vec::new(),
        dots: Vec::new(),
        pointer: None,
        texts: vec![Text {
            x: bounds.x + bounds.width / 2.0,
            center_y: bounds.y + 28.0,
            content: kind.title().to_owned(),
            size: typography::TITLE_SIZE,
            is_bold: true,
            color: theme.panel.text,
            align: TextAlign::Center,
        }],
        icons: Vec::new(),
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
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

    /// Rectángulo del panel provisional abierto, si lo hay.
    pub(crate) fn tray_bounds(&self) -> Option<Rect> {
        self.tray.map(|open| open.panel)
    }

    /// `true` si el punto cae en el panel provisional abierto.
    pub fn tray_contains(&self, x: f32, y: f32) -> bool {
        let Some(panel) = self.tray_bounds() else {
            return false;
        };
        x >= panel.x
            && x <= panel.x + panel.width
            && y >= panel.y
            && y <= panel.y + panel.height
    }
}
