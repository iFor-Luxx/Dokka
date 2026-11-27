//! Escena de la franja con el menú, el calendario o la fila colgando.

use dokka_components::calendar_panel::{calendar_month, calendar_panel, calendar_pill_props};
use dokka_components::menu_panel::submenu_overlay;
use dokka_scene::{Icon, IconImage, Panel, Rect, Scene, Text, TextAlign};
use dokka_theme::{menubar as measures, typography};

use crate::Menubar;

impl Menubar {
    /// Franja de lado a lado con icono, app, fila derecha y hora; el menú,
    /// el calendario o el panel provisional va en `overlay` (nunca dos).
    pub fn scene(&self, bar_width: f32) -> Scene {
        let bar = Rect {
            x: 0.0,
            y: 0.0,
            width: bar_width,
            height: measures::HEIGHT,
        };
        // Planos (sin cristal) y siempre presentes: así no se oscurecen ni se
        // reconstruyen las capas al pasar el cursor (2 fijos + 6 de la fila).
        let mut flat_panels = vec![
            self.highlight_panel(),
            self.clock_highlight_panel(bar_width),
        ];
        flat_panels.extend(self.tray_hover_panels(bar_width));
        // El recuadro del idioma va sobre su hover.
        flat_panels.push(self.tray_language_box(bar_width));
        let mut icons = vec![self.mark_icon()];
        icons.extend(self.tray_bar_icons(bar_width));
        Scene {
            panel: Panel {
                bounds: bar,
                corner_radius: 0.0,
                border_width: measures::BORDER_WIDTH,
                tint: self.theme.menubar.background,
                border: self.theme.menubar.border,
            },
            panels: Vec::new(),
            flat_panels,
            icons,
            dots: Vec::new(),
            dividers: Vec::new(),
            thumbnails: Vec::new(),
            labels: Vec::new(),
            texts: self.bar_texts(bar_width),
            highlight: None,
            overlay: self
                .menu_overlay()
                .or_else(|| self.calendar_overlay(bar_width))
                .or_else(|| self.tray_overlay()),
            submenu: None,
            opacity: 1.0,
            scale: 1.0,
            blur: 0.0,
        }
    }

    /// Franja y, abierto, el menú, el calendario o la fila con su hueco.
    pub fn active_areas(&self, bar_width: f32) -> Vec<Rect> {
        let mut areas = vec![Rect {
            x: 0.0,
            y: 0.0,
            width: bar_width,
            height: measures::HEIGHT,
        }];
        if let Some(menu) = &self.menu {
            areas.push(Rect {
                x: menu.panel.x,
                y: measures::HEIGHT,
                width: menu.panel.width,
                height: menu.panel.y - measures::HEIGHT + menu.panel.height,
            });
        } else if let Some(bounds) = self.calendar_bounds(bar_width) {
            areas.push(Rect {
                x: bounds.x,
                y: measures::HEIGHT,
                width: bounds.width,
                height: bounds.y - measures::HEIGHT + bounds.height,
            });
        } else if let Some(bounds) = self.tray_bounds() {
            areas.push(Rect {
                x: bounds.x,
                y: measures::HEIGHT,
                width: bounds.width,
                height: bounds.y - measures::HEIGHT + bounds.height,
            });
        }
        areas
    }

    fn mark_icon(&self) -> Icon {
        Icon {
            x: measures::SIDE_PADDING,
            y: (measures::HEIGHT - measures::ICON_SIDE) / 2.0,
            side: measures::ICON_SIDE,
            image: Some(IconImage::Drawing(std::sync::Arc::clone(&self.icon))),
            placeholder: self.theme.menubar.background,
            placeholder_radius: 0.0,
        }
    }

    fn highlight_panel(&self) -> Panel {
        let visible = self.icon_hovered || self.menu.is_some();
        Panel {
            bounds: if visible {
                self.icon_hover_bounds()
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
    }

    fn clock_highlight_panel(&self, bar_width: f32) -> Panel {
        let visible = self.clock_hovered || self.calendar.is_some();
        let bounds = if visible {
            self.clock_hover_bounds(bar_width)
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
    }

    fn bar_texts(&self, bar_width: f32) -> Vec<Text> {
        let mut texts = Vec::new();
        if !self.app_name.is_empty() {
            texts.push(Text {
                x: measures::SIDE_PADDING + measures::ICON_SIDE + measures::ICON_GAP,
                center_y: measures::HEIGHT / 2.0,
                content: self.app_name.clone(),
                size: typography::MENU_SIZE,
                is_bold: true,
                color: self.theme.menubar.text,
                align: TextAlign::Left,
            });
        }
        if let Some(language) = self.tray_language_text(bar_width) {
            texts.push(language);
        }
        if !self.clock.is_empty() {
            texts.push(Text {
                x: bar_width - measures::SIDE_PADDING - measures::CLOCK_SLOT / 2.0,
                center_y: measures::HEIGHT / 2.0,
                content: self.clock.clone(),
                size: typography::MENU_SIZE,
                is_bold: false,
                color: self.theme.menubar.text,
                align: TextAlign::Center,
            });
        }
        texts
    }

    fn menu_overlay(&self) -> Option<dokka_scene::Overlay> {
        let menu = self.menu.as_ref()?;
        Some(submenu_overlay(
            &self.theme,
            &self.chevrons,
            &self.checks,
            menu.panel,
            &menu.rows,
            menu.highlighted,
        ))
    }

    fn calendar_overlay(&self, bar_width: f32) -> Option<dokka_scene::Overlay> {
        let bounds = self.calendar_bounds(bar_width)?;
        let open = self.calendar?;
        let month = calendar_month(self.today, open.month_offset);
        let pill = calendar_pill_props(bounds, self.nav_hovered);
        Some(calendar_panel(
            &self.theme,
            &self.nav_pill,
            bounds,
            &month,
            &pill,
        ))
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
