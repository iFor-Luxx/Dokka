//! Menús de los iconos del sistema: explorador, papelera y Launchpad.
//!
//! Cada fila con acción clara lleva su icono Lucide en el color
//! del texto; los títulos de ventana van sin icono.

use std::sync::Arc;

use dokka_components::menu_panel::MenuRow;
use dokka_icons::{
    menu_add_app_icon, menu_desktop_icon, menu_documents_icon, menu_downloads_icon,
    menu_empty_trash_icon, menu_launchpad_icon, menu_new_window_icon, menu_pictures_icon,
    menu_trash_open_icon,
};
use dokka_model::DockApp;
use dokka_scene::Color;

use super::dock_menu::{MenuData, SubmenuData};
use super::menu::MenuAction;

/// Ventanas, nueva ventana, carpetas y cierre. Sin fijar ni administrador.
pub(crate) fn explorer_menu(app: &DockApp, text: Color) -> MenuData {
    let mut rows = Vec::new();
    let mut actions: Vec<Option<MenuAction>> = Vec::new();
    let mut children: Vec<Option<SubmenuData>> = Vec::new();

    super::app_menu::push_windows(&mut rows, &mut actions, &mut children, &app.windows);
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [(
            MenuRow::item_with_icon("Nueva ventana", Arc::new(menu_new_window_icon(text))),
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
                MenuRow::item_with_icon("Descargas", Arc::new(menu_downloads_icon(text))),
                Some(MenuAction::OpenDownloads),
                None,
            ),
            (
                MenuRow::item_with_icon("Documentos", Arc::new(menu_documents_icon(text))),
                Some(MenuAction::OpenDocuments),
                None,
            ),
            (
                MenuRow::item_with_icon("Escritorio", Arc::new(menu_desktop_icon(text))),
                Some(MenuAction::OpenDesktop),
                None,
            ),
            (
                MenuRow::item_with_icon("Imágenes", Arc::new(menu_pictures_icon(text))),
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
            super::app_menu::close_rows(app, text),
        );
    }
    MenuData {
        rows,
        actions,
        children,
    }
}

/// "Abrir" y, solo con contenido, "Vaciar papelera".
pub(crate) fn trash_menu(trash_size: Option<u64>, text: Color) -> MenuData {
    let mut rows = vec![MenuRow::item_with_icon(
        "Abrir",
        Arc::new(menu_trash_open_icon(text)),
    )];
    let mut actions = vec![Some(MenuAction::OpenTrash)];
    let mut children = vec![None];
    if trash_size.is_some() {
        rows.push(MenuRow::Separator);
        actions.push(None);
        children.push(None);
        rows.push(MenuRow::item_with_icon(
            "Vaciar papelera",
            Arc::new(menu_empty_trash_icon(text)),
        ));
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
pub(crate) fn launchpad_menu(text: Color) -> MenuData {
    MenuData {
        rows: vec![
            MenuRow::item_with_icon("Abrir Launchpad", Arc::new(menu_launchpad_icon(text))),
            MenuRow::item_with_icon("Añadir app…", Arc::new(menu_add_app_icon(text))),
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
