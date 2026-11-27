//! Fila derecha en la escena: iconos según los datos y su panel real.

use std::sync::Arc;

use dokka_scene::{Icon, IconImage, Overlay, Panel, Rect, Text, TextAlign};
use dokka_theme::{menubar as bar_measures, menubar_tray as measures, typography};

use super::super::logic::TrayKind;
use crate::Menubar;

impl Menubar {
    /// Iconos de la fila: el altavoz sigue al nivel y al silencio, la red a
    /// su tipo, la lupa y los deslizadores a sus dibujos; el idioma es texto.
    /// Siempre 5 iconos + texto: no se reconstruyen las capas al pasar.
    pub(crate) fn tray_bar_icons(&self, bar_width: f32) -> Vec<Icon> {
        let cells = self.tray_cells(bar_width);
        let at = |kind: TrayKind| {
            cells
                .iter()
                .find(|(cell, _)| *cell == kind)
                .map(|(_, area)| *area)
        };
        let place = |kind: TrayKind, drawing: Arc<dokka_scene::Drawing>| {
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
                image: Some(IconImage::Drawing(drawing)),
                placeholder: self.theme.menubar.background,
                placeholder_radius: 0.0,
            }
        };
        vec![
            place(TrayKind::ControlCenter, Arc::clone(&self.control_icons.bar)),
            place(TrayKind::Search, Arc::clone(self.search_icons.bar())),
            place(TrayKind::Music, Arc::clone(&self.tray_icons.music)),
            place(
                TrayKind::Network,
                self.network_icons
                    .for_bar(super::super::logic::network_type(self.network.kind)),
            ),
            place(
                TrayKind::Volume,
                Arc::clone(
                    self.volume_icons
                        .bar(self.volume.level, self.volume.is_muted),
                ),
            ),
        ]
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
            .find(|(kind, _)| *kind == TrayKind::Language)?;
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

    /// Panel real bajo su icono, según qué fila lo abrió.
    pub(crate) fn tray_overlay(&self) -> Option<Overlay> {
        let open = self.tray?;
        Some(match open.kind {
            TrayKind::Volume => self.volume_overlay(),
            TrayKind::Network => self.network_overlay(),
            TrayKind::Music => self.music_overlay(),
            TrayKind::Language => self.language_overlay(),
            TrayKind::Search => self.search_overlay(),
            TrayKind::ControlCenter => self.control_overlay(),
        })
    }
}
