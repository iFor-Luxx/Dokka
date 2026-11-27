//! Fila derecha en la escena: iconos, idioma, hovers y panel provisional.

use dokka_scene::{Icon, IconImage, Overlay, Panel, Rect, Text, TextAlign};
use dokka_theme::{menubar as bar_measures, menubar_tray as measures, typography};

use super::super::logic::tray_placeholder_overlay;
use crate::Menubar;

impl Menubar {
    /// Iconos Lucide de la fila (el idioma es texto, no icono).
    /// Siempre 5: no se reconstruyen las capas al pasar el cursor.
    pub(crate) fn tray_bar_icons(&self, bar_width: f32) -> Vec<Icon> {
        let cells = self.tray_cells(bar_width);
        let at = |kind: super::super::logic::TrayKind| {
            cells
                .iter()
                .find(|(cell, _)| *cell == kind)
                .map(|(_, area)| *area)
        };
        let drawings = [
            (
                super::super::logic::TrayKind::ControlCenter,
                &self.tray_icons.control,
            ),
            (
                super::super::logic::TrayKind::Search,
                &self.tray_icons.search,
            ),
            (
                super::super::logic::TrayKind::Music,
                &self.tray_icons.music,
            ),
            (
                super::super::logic::TrayKind::Network,
                &self.tray_icons.network,
            ),
            (
                super::super::logic::TrayKind::Volume,
                &self.tray_icons.volume,
            ),
        ];
        drawings
            .into_iter()
            .map(|(kind, drawing)| {
                let area = at(kind).unwrap_or(Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                });
                Icon {
                    x: area.x + (area.width - measures::ICON_SIDE) / 2.0,
                    y: (bar_measures::HEIGHT - measures::ICON_SIDE) / 2.0,
                    side: measures::ICON_SIDE,
                    image: Some(IconImage::Drawing(std::sync::Arc::clone(drawing))),
                    placeholder: self.theme.menubar.background,
                    placeholder_radius: 0.0,
                }
            })
            .collect()
    }

    /// Hovers planos de la fila, uno por icono. Siempre 6 (cero si no se ve).
    pub(crate) fn tray_hover_panels(&self, bar_width: f32) -> Vec<Panel> {
        self.tray_cells(bar_width)
            .into_iter()
            .map(|(kind, _)| {
                let open = self.tray_kind() == Some(kind);
                let visible = self.tray_hovered == Some(kind) || open;
                Panel {
                    bounds: if visible {
                        self.tray_hover_bounds(kind, bar_width)
                    } else {
                        Rect {
                            x: 0.0,
                            y: 0.0,
                            width: 0.0,
                            height: 0.0,
                        }
                    },
                    corner_radius: measures::HOVER_RADIUS,
                    border_width: 0.0,
                    tint: self.theme.menubar.hover,
                    border: self.theme.menubar.hover,
                }
            })
            .collect()
    }

    /// Código del idioma centrado en su hueco (como "ES").
    pub(crate) fn tray_language_text(&self, bar_width: f32) -> Option<Text> {
        if self.language_code.is_empty() {
            return None;
        }
        let (_, area) = self
            .tray_cells(bar_width)
            .into_iter()
            .find(|(kind, _)| *kind == super::super::logic::TrayKind::Language)?;
        Some(Text {
            x: area.x + area.width / 2.0,
            center_y: bar_measures::HEIGHT / 2.0,
            content: self.language_code.clone(),
            size: typography::MENU_SIZE,
            is_bold: false,
            color: self.theme.menubar.text,
            align: TextAlign::Center,
        })
    }

    /// Panel provisional bajo su icono. Los reales sustituyen esta llamada
    /// por su `<x>_panel` con el mismo `bounds` (ver `README.md`).
    pub(crate) fn tray_overlay(&self) -> Option<Overlay> {
        let open = self.tray?;
        Some(tray_placeholder_overlay(&self.theme, open.panel, open.kind))
    }
}
