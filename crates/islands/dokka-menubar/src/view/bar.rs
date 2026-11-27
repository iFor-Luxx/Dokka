//! Escena de la franja con el menú colgando como `overlay`.

use dokka_components::menu_panel::submenu_overlay;
use dokka_scene::{Icon, IconImage, Panel, Rect, Scene, Text, TextAlign};
use dokka_theme::{menubar as measures, typography};

use crate::Menubar;

impl Menubar {
    /// Franja de lado a lado con icono, app y hora; el menú va en `overlay`.
    pub fn scene(&self, bar_width: f32) -> Scene {
        let bar = Rect {
            x: 0.0,
            y: 0.0,
            width: bar_width,
            height: measures::HEIGHT,
        };
        // Plano (sin cristal) y siempre presente: así no se oscurece ni se
        // reconstruyen las capas al pasar el cursor.
        let hover = self.highlight_panel();
        Scene {
            panel: Panel {
                bounds: bar,
                corner_radius: 0.0,
                border_width: measures::BORDER_WIDTH,
                tint: self.theme.menubar.background,
                border: self.theme.menubar.border,
            },
            panels: Vec::new(),
            flat_panels: vec![hover],
            icons: vec![self.mark_icon()],
            dots: Vec::new(),
            dividers: Vec::new(),
            thumbnails: Vec::new(),
            labels: Vec::new(),
            texts: self.bar_texts(bar_width),
            highlight: None,
            overlay: self.menu_overlay(),
            submenu: None,
            opacity: 1.0,
            scale: 1.0,
            blur: 0.0,
        }
    }

    /// Franja y, abierto, el menú con su hueco para no perder el cursor.
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
}
