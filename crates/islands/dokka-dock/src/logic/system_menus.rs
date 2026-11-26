//! Menús de los iconos del sistema: explorador, papelera y Launchpad.

use dokka_components::menu_panel::MenuRow;
use dokka_model::DockApp;

use super::dock_menu::{MenuData, SubmenuData};
use super::menu::MenuAction;

/// Ventanas, nueva ventana, carpetas y cierre. Sin fijar ni administrador.
pub(crate) fn explorer_menu(app: &DockApp) -> MenuData {
    let mut rows = Vec::new();
    let mut actions: Vec<Option<MenuAction>> = Vec::new();
    let mut children: Vec<Option<SubmenuData>> = Vec::new();

    super::app_menu::push_windows(&mut rows, &mut actions, &mut children, &app.windows);
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [(
            MenuRow::item("Nueva ventana"),
            Some(MenuAction::OpenApp(app.info.clone())),
            None,
        )],
    );
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [
            (
                MenuRow::item("Descargas"),
                Some(MenuAction::OpenDownloads),
                None,
            ),
            (
                MenuRow::item("Documentos"),
                Some(MenuAction::OpenDocuments),
                None,
            ),
            (
                MenuRow::item("Escritorio"),
                Some(MenuAction::OpenDesktop),
                None,
            ),
            (
                MenuRow::item("Imágenes"),
                Some(MenuAction::OpenPictures),
                None,
            ),
        ],
    );
    if app.is_running() {
        push_section(
            &mut rows,
            &mut actions,
            &mut children,
            super::app_menu::close_rows(app),
        );
    }
    MenuData {
        rows,
        actions,
        children,
    }
}

/// "Abrir" y, solo con contenido, "Vaciar papelera".
pub(crate) fn trash_menu(trash_size: Option<u64>) -> MenuData {
    let mut rows = vec![MenuRow::item("Abrir")];
    let mut actions = vec![Some(MenuAction::OpenTrash)];
    let mut children = vec![None];
    if trash_size.is_some() {
        rows.push(MenuRow::Separator);
        actions.push(None);
        children.push(None);
        rows.push(MenuRow::item("Vaciar papelera"));
        actions.push(Some(MenuAction::EmptyTrash));
        children.push(None);
    }
    MenuData {
        rows,
        actions,
        children,
    }
}

/// "Abrir Launchpad" por el mismo camino que su icono, y "Añadir app…".
pub(crate) fn launchpad_menu() -> MenuData {
    MenuData {
        rows: vec![
            MenuRow::item("Abrir Launchpad"),
            MenuRow::item("Añadir app…"),
        ],
        actions: vec![Some(MenuAction::OpenLaunchpad), Some(MenuAction::AddApp)],
        children: vec![None, None],
    }
}

/// Secciones separadas solo cuando ambas tienen contenido.
fn push_section(
    rows: &mut Vec<MenuRow>,
    actions: &mut Vec<Option<MenuAction>>,
    children: &mut Vec<Option<SubmenuData>>,
    section: impl IntoIterator<Item = (MenuRow, Option<MenuAction>, Option<SubmenuData>)>,
) {
    let section: Vec<_> = section.into_iter().collect();
    if section.is_empty() {
        return;
    }
    if !rows.is_empty() {
        rows.push(MenuRow::Separator);
        actions.push(None);
        children.push(None);
    }
    for (row, action, child) in section {
        rows.push(row);
        actions.push(action);
        children.push(child);
    }
}
