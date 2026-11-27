//! Contenido del menú de la línea separadora, con sus submenús.

use dokka_components::menu_panel::MenuRow;

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
pub(crate) fn dock_menu(taskbar_hidden: bool, taskbar_auto: bool) -> MenuData {
    let rows = vec![
        MenuRow::submenu("Elementos del dock"),
        MenuRow::Separator,
        MenuRow::submenu("Barra de tareas"),
        MenuRow::item("Bloquear arrastre de iconos"),
        MenuRow::submenu("Sistema"),
        MenuRow::Separator,
        MenuRow::item("Workshop"),
        MenuRow::Separator,
        MenuRow::item("Preferencias"),
        MenuRow::item("Reiniciar Dokka"),
        MenuRow::item("Salir de Dokka"),
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
        Some(dock_items_submenu()),
        None,
        Some(taskbar_submenu(taskbar_hidden, taskbar_auto)),
        None,
        Some(system_submenu()),
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

fn dock_items_submenu() -> SubmenuData {
    SubmenuData {
        rows: vec![
            MenuRow::item("Añadir app…"),
            MenuRow::item("Añadir acceso del sistema"),
            MenuRow::item("Añadir separador"),
            MenuRow::item("Quitar separador"),
        ],
        actions: vec![Some(MenuAction::AddApp), None, None, None],
    }
}

fn taskbar_submenu(taskbar_hidden: bool, taskbar_auto: bool) -> SubmenuData {
    let (label, action) = if taskbar_hidden {
        ("Mostrar ahora", MenuAction::ShowTaskbar)
    } else {
        ("Ocultar ahora", MenuAction::HideTaskbar)
    };
    SubmenuData {
        rows: vec![
            MenuRow::item(label),
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

fn system_submenu() -> SubmenuData {
    SubmenuData {
        rows: vec![
            MenuRow::item("Mostrar el escritorio"),
            MenuRow::item("Administrador de tareas"),
            MenuRow::item("Configuración de la barra de tareas"),
        ],
        actions: vec![
            Some(MenuAction::ShowDesktop),
            Some(MenuAction::TaskManager),
            Some(MenuAction::TaskbarSettings),
        ],
    }
}
