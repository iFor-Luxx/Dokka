//! Dónde se coloca el menú según el borde del dock.
//! Solo colocación; las filas se construyen en `dock_menu`, `app_menu` y `system_menus`.

use dokka_components::menu_panel::{MenuRow, menu_size};
use dokka_model::{DockApp, DockPosition};
use dokka_scene::Rect;
use dokka_theme::menu as measures;

use crate::Dock;

/// Sitio del menú ya calculado.
pub(crate) struct MenuPlacement {
    pub(crate) panel: Rect,
    pub(crate) tip_x: f32,
    pub(crate) tip_y: f32,
    pub(crate) floor: f32,
    pub(crate) limit: f32,
    pub(crate) cross_limit: f32,
    pub(crate) position: DockPosition,
}

pub(crate) fn place_menu(
    dock: &Dock,
    apps: &[DockApp],
    x: f32,
    y: f32,
    rows: &[MenuRow],
) -> MenuPlacement {
    let size = menu_size(rows);
    let surface = dock.surface_size(apps);
    let position = dock.preferences.position;
    match position {
        DockPosition::Left | DockPosition::Right => {
            let bar = super::place_dock(dock, apps).bar;
            let bar_side = if position == DockPosition::Left {
                bar.x + bar.width
            } else {
                bar.x
            };
            let limit = surface.height;
            let panel_y = (y - size.height / 2.0).clamp(0.0, (limit - size.height).max(0.0));
            let panel_x = if position == DockPosition::Left {
                bar_side + measures::POINTER_HEIGHT
            } else {
                (bar_side - measures::POINTER_HEIGHT - size.width).max(0.0)
            };
            MenuPlacement {
                panel: Rect {
                    x: panel_x,
                    y: panel_y,
                    width: size.width,
                    height: size.height,
                },
                tip_x: x,
                tip_y: y,
                floor: bar_side,
                limit,
                cross_limit: surface.width,
                position,
            }
        }
        _ => {
            let limit = surface.width;
            let panel_x = (x - size.width / 2.0).clamp(0.0, (limit - size.width).max(0.0));
            let bar_top = super::place_dock(dock, apps).bar.y;
            let bottom = bar_top - measures::POINTER_HEIGHT;
            MenuPlacement {
                panel: Rect {
                    x: panel_x,
                    y: (bottom - size.height).max(0.0),
                    width: size.width,
                    height: size.height,
                },
                tip_x: x,
                tip_y: y,
                floor: bar_top,
                limit,
                cross_limit: surface.height,
                position,
            }
        }
    }
}
