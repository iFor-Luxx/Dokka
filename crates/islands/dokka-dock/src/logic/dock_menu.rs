//! Contenido del menú de la línea separadora, con sus submenús.
//!
//! Cada fila con acción clara lleva su icono Lucide de `dokka-icons`
//! en el color del texto; las madres y las opciones sin acción clara
//! van sin icono para dejar la columna alineada.

use std::sync::Arc;

use dokka_components::menu_panel::MenuRow;
use dokka_icons::{
    menu_add_app_icon, menu_add_separator_icon, menu_add_system_icon, menu_desktop_icon,
    menu_hide_taskbar_icon, menu_power_icon, menu_preferences_icon, menu_remove_separator_icon,
    menu_restart_icon, menu_show_taskbar_icon, menu_task_manager_icon, menu_taskbar_settings_icon,
};
use dokka_scene::Color;

use super::menu::MenuAction;

/// Submenú de una fila madre: sus filas y acciones.
#[derive(Debug, Clone)]
pub(crate) struct SubmenuData {
    pub(crate) rows: Vec<MenuRow>,
    pub(crate) actions: Vec<Option<MenuAction>>,
}

/// Menú completo: filas, acciones y submenú de cada fila madre.
#[derive(Debug, Clone)]
pub(crate) struct MenuData {
    pub(crate) rows: Vec<MenuRow>,
    pub(crate) actions: Vec<Option<MenuAction>>,
    pub(crate) children: Vec<Option<SubmenuData>>,
}

/// Filas, acciones y submenús del menú principal.
pub(crate) fn dock_menu(text: Color, taskbar_hidden: bool, taskbar_auto: bool) -> MenuData {
    let rows = vec![
        MenuRow::submenu("Elementos del dock"),
        MenuRow::Separator,
        MenuRow::submenu("Barra de tareas"),
        MenuRow::item("Bloquear arrastre de iconos"),
        MenuRow::submenu("Sistema"),
        MenuRow::Separator,
        MenuRow::item("Workshop"),
        MenuRow::Separator,
        MenuRow::item_with_icon("Preferencias", Arc::new(menu_preferences_icon(text))),
        MenuRow::item_with_icon("Reiniciar Dokka", Arc::new(menu_restart_icon(text))),
        MenuRow::item_with_icon("Salir de Dokka", Arc::new(menu_power_icon(text))),
    ];
    let actions = vec![
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(MenuAction::OpenSettings),
        Some(MenuAction::Restart),
        Some(MenuAction::Quit),
    ];
    let children = vec![
        Some(dock_items_submenu(text)),
        None,
        Some(taskbar_submenu(text, taskbar_hidden, taskbar_auto)),
        None,
        Some(system_submenu(text)),
        None,
        None,
        None,
        None,
        None,
        None,
    ];
    MenuData {
        rows,
        actions,
        children,
    }
}

fn dock_items_submenu(text: Color) -> SubmenuData {
    SubmenuData {
        rows: vec![
            MenuRow::item_with_icon("Añadir app…", Arc::new(menu_add_app_icon(text))),
            MenuRow::item_with_icon(
                "Añadir acceso del sistema",
                Arc::new(menu_add_system_icon(text)),
            ),
            MenuRow::item_with_icon(
                "Añadir separador",
                Arc::new(menu_add_separator_icon(text)),
            ),
            MenuRow::item_with_icon(
                "Quitar separador",
                Arc::new(menu_remove_separator_icon(text)),
            ),
        ],
        actions: vec![Some(MenuAction::AddApp), None, None, None],
    }
}

fn taskbar_submenu(text: Color, taskbar_hidden: bool, taskbar_auto: bool) -> SubmenuData {
    let (label, action, icon) = if taskbar_hidden {
        (
            "Mostrar ahora",
            MenuAction::ShowTaskbar,
            menu_show_taskbar_icon(text),
        )
    } else {
        (
            "Ocultar ahora",
            MenuAction::HideTaskbar,
            menu_hide_taskbar_icon(text),
        )
    };
    SubmenuData {
        rows: vec![
            MenuRow::item_with_icon(label, Arc::new(icon)),
            MenuRow::Separator,
            MenuRow::checked("Manual", !taskbar_auto),
            MenuRow::checked("Automático al abrir Dokka", taskbar_auto),
        ],
        actions: vec![
            Some(action),
            None,
            Some(MenuAction::TaskbarManual),
            Some(MenuAction::TaskbarAuto),
        ],
    }
}

fn system_submenu(text: Color) -> SubmenuData {
    SubmenuData {
        rows: vec![
            MenuRow::item_with_icon(
                "Mostrar el escritorio",
                Arc::new(menu_desktop_icon(text)),
            ),
            MenuRow::item_with_icon(
                "Administrador de tareas",
                Arc::new(menu_task_manager_icon(text)),
            ),
            MenuRow::item_with_icon(
                "Configuración de la barra de tareas",
                Arc::new(menu_taskbar_settings_icon(text)),
            ),
        ],
        actions: vec![
            Some(MenuAction::ShowDesktop),
            Some(MenuAction::TaskManager),
            Some(MenuAction::TaskbarSettings),
        ],
    }
}
