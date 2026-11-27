//! Fila derecha en la escena: iconos según los datos y su panel real.
//! Solo los encendidos en Preferencias se dibujan, sin huecos.

use std::sync::Arc;

use dokka_scene::{Icon, IconImage, Overlay, Panel, Rect, Text, TextAlign};
use dokka_theme::{menubar as bar_measures, menubar_tray as measures};

use super::super::logic::TrayKind;
use crate::Menubar;

impl Menubar {
    /// Iconos visibles de la fila: el altavoz sigue al nivel y al silencio,
    /// la red a su tipo, la lupa y los deslizadores a sus dibujos.
    /// Solo los encendidos; el idioma va como texto aparte.
    pub(crate) fn tray_bar_icons(&self, bar_width: f32) -> Vec<Icon> {
        let cells = self.tray_cells(bar_width);
        let at = |kind: TrayKind| {
            cells
                .iter()
                .find(|(cell, _)| *cell == kind)
                .map(|(_, area)| *area)
        };
        let mut icons = Vec::new();
        let mut place = |kind: TrayKind, drawing: Arc<dokka_scene::Drawing>| {
            let Some(area) = at(kind) else {
                return;
            };
            icons.push(Icon {
                x: area.x + (area.width - measures::ICON_SIDE) / 2.0,
                y: (bar_measures::HEIGHT - measures::ICON_SIDE) / 2.0,
                side: measures::ICON_SIDE,
                image: Some(IconImage::Drawing(drawing)),
                placeholder: self.theme.menubar.background,
                placeholder_radius: 0.0,
            });
        };
        // De derecha a izquierda en la barra, solo los visibles.
        for kind in self.visible_kinds() {
            match kind {
                TrayKind::ControlCenter => {
                    place(kind, Arc::clone(&self.control_icons.bar));
                }
                TrayKind::Search => {
                    place(kind, Arc::clone(self.search_icons.bar()));
                }
                TrayKind::Music => {
                    place(kind, Arc::clone(&self.tray_icons.music));
                }
                TrayKind::Network => {
                    place(
                        kind,
                        self.network_icons
                            .for_bar(super::super::logic::network_type(self.network.kind)),
                    );
                }
                TrayKind::Volume => {
                    place(
                        kind,
                        Arc::clone(
                            self.volume_icons
                                .bar(self.volume.level, self.volume.is_muted),
                        ),
                    );
                }
                TrayKind::Language => {}
            }
        }
        icons
    }

    /// Hovers en cápsula de la fila, uno por icono visible (cero si no se ve).
    pub(crate) fn tray_hover_panels(&self, bar_width: f32) -> Vec<Panel> {
        self.tray_cells(bar_width)
            .into_iter()
            .map(|(kind, _)| {
                let open = self.tray_kind() == Some(kind);
                let visible = self.tray_hovered == Some(kind) || open;
                let bounds = if visible {
                    self.tray_hover_bounds(kind, bar_width)
                } else {
                    Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 0.0,
                        height: 0.0,
                    }
                };
                Panel {
                    bounds,
                    corner_radius: capsule_radius(bounds),
                    border_width: 0.0,
                    tint: self.theme.menubar.hover,
                    border: self.theme.menubar.hover,
                }
            })
            .collect()
    }

    /// Recuadro claro tras el código del idioma, como una tecla. Siempre
    /// existe: de tamaño cero sin idioma o con el idioma apagado.
    pub(crate) fn tray_language_box(&self, bar_width: f32) -> Panel {
        let area = self
            .tray_cells(bar_width)
            .into_iter()
            .find(|(kind, _)| *kind == TrayKind::Language)
            .filter(|_| !self.language_code.is_empty())
            .map(|(_, area)| area);
        let bounds = area.map_or(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            |area| Rect {
                x: area.x + (area.width - measures::LANG_BOX_WIDTH) / 2.0,
                y: (bar_measures::HEIGHT - measures::LANG_BOX_HEIGHT) / 2.0,
                width: measures::LANG_BOX_WIDTH,
                height: measures::LANG_BOX_HEIGHT,
            },
        );
        Panel {
            bounds,
            corner_radius: measures::LANG_BOX_RADIUS,
            border_width: 0.0,
            tint: self.theme.menubar.text,
            border: self.theme.menubar.text,
        }
    }

    /// Código del idioma centrado en su hueco (como "ES").
    /// Sin hueco visible no hay texto.
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
            size: measures::LANG_TEXT_SIZE,
            is_bold: true,
            color: self.theme.panel.text,
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

/// Cápsula translúcida como en Tahoe: extremos totalmente redondos.
/// Con tamaño cero, el radio es cero para no superar la mitad del lado.
fn capsule_radius(bounds: Rect) -> f32 {
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return 0.0;
    }
    (bounds.width / 2.0).min(bounds.height / 2.0)
}
