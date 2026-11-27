//! Filas fijas del menú del icono y sitio del panel colgando.
//!
//! Cada fila con acción clara lleva su icono Lucide en el color
//! del texto; los textos quedan alineados en una sola columna.

use std::sync::Arc;

use dokka_components::menu_panel::{MenuRow, menu_size};
use dokka_icons::{
    menu_desktop_icon, menu_lock_icon, menu_power_icon, menu_preferences_icon, menu_restart_icon,
    menu_task_manager_icon,
};
use dokka_scene::{Color, Rect};
use dokka_theme::menubar;

use crate::MenubarAction;

/// Filas del menú: Preferencias, escritorio y tareas, bloqueo y apagado,
/// reiniciar y salir, con sus separadores.
pub(crate) fn menu_rows(text: Color) -> Vec<MenuRow> {
    vec![
        MenuRow::item_with_icon("Preferencias…", Arc::new(menu_preferences_icon(text))),
        MenuRow::Separator,
        MenuRow::item_with_icon("Mostrar el escritorio", Arc::new(menu_desktop_icon(text))),
        MenuRow::item_with_icon(
            "Administrador de tareas",
            Arc::new(menu_task_manager_icon(text)),
        ),
        MenuRow::Separator,
        MenuRow::item_with_icon("Bloquear pantalla", Arc::new(menu_lock_icon(text))),
        MenuRow::item_with_icon("Apagar o reiniciar…", Arc::new(menu_power_icon(text))),
        MenuRow::Separator,
        MenuRow::item_with_icon("Reiniciar Dokka", Arc::new(menu_restart_icon(text))),
        MenuRow::item_with_icon("Salir de Dokka", Arc::new(menu_power_icon(text))),
    ]
}

/// Acción de cada fila elegible, en el mismo orden que `menu_rows`.
pub(crate) fn menu_actions() -> Vec<Option<MenubarAction>> {
    vec![
        Some(MenubarAction::OpenSettings),
        None,
        Some(MenubarAction::ShowDesktop),
        Some(MenubarAction::TaskManager),
        None,
        Some(MenubarAction::LockScreen),
        Some(MenubarAction::Power),
        None,
        Some(MenubarAction::Restart),
        Some(MenubarAction::Quit),
    ]
}

/// Menú abierto: filas, acciones y resaltado.
#[derive(Debug, Clone)]
pub(crate) struct OpenMenu {
    pub(crate) rows: Vec<MenuRow>,
    pub(crate) actions: Vec<Option<MenubarAction>>,
    pub(crate) panel: Rect,
    pub(crate) highlighted: Option<usize>,
}

/// Panel colgando de la franja: bajo el icono, con hueco para que las
/// esquinas se vean concéntricas, sin salirse del ancho.
pub(crate) fn menu_panel_for(rows: &[MenuRow], bar_width: f32) -> Rect {
    let size = menu_size(rows);
    let limit = bar_width.max(size.width);
    let x = menubar::MENU_X.clamp(0.0, (limit - size.width).max(0.0));
    Rect {
        x,
        y: menubar::HEIGHT + menubar::MENU_GAP,
        width: size.width,
        height: size.height,
    }
}
