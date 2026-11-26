//! Contenido del menú de la línea separadora.

use dokka_components::menu_panel::MenuRow;

use super::menu::MenuAction;

/// Filas y acciones del menú de la separadora. Las filas sin acción llevan `None`.
pub(crate) fn dock_menu() -> (Vec<MenuRow>, Vec<Option<MenuAction>>) {
    let rows = vec![
        MenuRow::Item("Añadir app…".to_owned()),
        MenuRow::Item("Añadir acceso del sistema".to_owned()),
        MenuRow::Item("Añadir separador".to_owned()),
        MenuRow::Item("Quitar separador".to_owned()),
        MenuRow::Separator,
        MenuRow::Item("Mostrar barra de tareas".to_owned()),
        MenuRow::Item("Ocultar barra de tareas".to_owned()),
        MenuRow::Item("Bloquear arrastre de iconos".to_owned()),
        MenuRow::Item("Mostrar el escritorio".to_owned()),
        MenuRow::Item("Administrador de tareas".to_owned()),
        MenuRow::Item("Configuración de la barra de tareas".to_owned()),
        MenuRow::Separator,
        MenuRow::Item("Workshop".to_owned()),
        MenuRow::Separator,
        MenuRow::Item("Preferencias".to_owned()),
        MenuRow::Item("Reiniciar Dokka".to_owned()),
        MenuRow::Item("Salir de Dokka".to_owned()),
    ];
    let actions = vec![
        Some(MenuAction::AddApp),
        None,
        None,
        None,
        None,
        Some(MenuAction::ShowTaskbar),
        Some(MenuAction::HideTaskbar),
        None,
        Some(MenuAction::ShowDesktop),
        Some(MenuAction::TaskManager),
        Some(MenuAction::TaskbarSettings),
        None,
        None,
        None,
        None,
        Some(MenuAction::Restart),
        Some(MenuAction::Quit),
    ];
    (rows, actions)
}
