//! Menú de una app normal: ventanas, abrir, opciones, fijar, cerrar.
//!
//! Cada fila con acción clara lleva su icono Lucide en el color
//! del texto; los títulos de ventana van sin icono.

use std::sync::Arc;

use dokka_components::menu_panel::MenuRow;
use dokka_icons::{
    menu_admin_icon, menu_close_icon, menu_open_icon, menu_pin_icon, menu_quit_icon,
    menu_reveal_icon, menu_unpin_icon,
};
use dokka_model::{AppWindow, DockApp, WindowId};
use dokka_scene::Color;

use super::dock_menu::{MenuData, SubmenuData};
use super::menu::MenuAction;

/// Ventanas que se listan como máximo, para que el menú quepa.
pub(crate) const MAX_WINDOWS: usize = 6;
/// Caracteres de título antes de recortar con "…"; el ancho del menú es fijo.
pub(crate) const MAX_TITLE_CHARS: usize = 30;

/// Una fila por ventana (máximo 6), abrir, administrador, opciones, fijar, cerrar.
pub(crate) fn app_menu(app: &DockApp, text: Color) -> MenuData {
    let mut rows = Vec::new();
    let mut actions: Vec<Option<MenuAction>> = Vec::new();
    let mut children: Vec<Option<SubmenuData>> = Vec::new();

    push_windows(&mut rows, &mut actions, &mut children, &app.windows);
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [
            (
                MenuRow::item_with_icon(open_label(app), Arc::new(menu_open_icon(text))),
                Some(MenuAction::OpenApp(app.info.clone())),
                None,
            ),
            (
                MenuRow::item_with_icon(
                    "Ejecutar como administrador",
                    Arc::new(menu_admin_icon(text)),
                ),
                Some(MenuAction::RunAsAdministrator(app.info.clone())),
                None,
            ),
        ],
    );
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [(MenuRow::submenu("Opciones"), None, Some(SubmenuData {
            rows: vec![MenuRow::item_with_icon(
                "Abrir ubicación del archivo",
                Arc::new(menu_reveal_icon(text)),
            )],
            actions: vec![Some(MenuAction::ShowInFolder(app.info.clone()))],
        }))],
    );
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [pin_row(app, text)],
    );
    if app.is_running() {
        push_section(&mut rows, &mut actions, &mut children, close_rows(app, text));
    }
    MenuData {
        rows,
        actions,
        children,
    }
}

/// Filas de ventanas con el título acortado; cada una trae su ventana al frente.
pub(crate) fn push_windows(
    rows: &mut Vec<MenuRow>,
    actions: &mut Vec<Option<MenuAction>>,
    children: &mut Vec<Option<SubmenuData>>,
    windows: &[AppWindow],
) {
    for window in windows.iter().take(MAX_WINDOWS) {
        rows.push(MenuRow::item(shorten_title(&window.title)));
        actions.push(Some(MenuAction::ActivateWindow(window.id)));
        children.push(None);
    }
}

/// "Abrir" si está cerrada, "Abrir otra vez" si ya está abierta.
pub(crate) fn open_label(app: &DockApp) -> &'static str {
    if app.is_running() {
        "Abrir otra vez"
    } else {
        "Abrir"
    }
}

/// Fila de fijar o quitar según el estado actual.
pub(crate) fn pin_row(
    app: &DockApp,
    text: Color,
) -> (MenuRow, Option<MenuAction>, Option<SubmenuData>) {
    if app.is_pinned {
        (
            MenuRow::item_with_icon("Quitar del dock", Arc::new(menu_unpin_icon(text))),
            Some(MenuAction::Unpin(app.info.clone())),
            None,
        )
    } else {
        (
            MenuRow::item_with_icon("Fijar en el dock", Arc::new(menu_pin_icon(text))),
            Some(MenuAction::Pin(app.info.clone())),
            None,
        )
    }
}

/// "Cerrar ventana" (la más al frente) y "Salir" (cierra todas, sin matar).
pub(crate) fn close_rows(
    app: &DockApp,
    text: Color,
) -> Vec<(MenuRow, Option<MenuAction>, Option<SubmenuData>)> {
    let ids: Vec<WindowId> = app.windows.iter().map(|window| window.id).collect();
    vec![
        (
            MenuRow::item_with_icon("Cerrar ventana", Arc::new(menu_close_icon(text))),
            Some(MenuAction::CloseFrontWindow(ids.clone())),
            None,
        ),
        (
            MenuRow::item_with_icon("Salir", Arc::new(menu_quit_icon(text))),
            Some(MenuAction::QuitApp(ids)),
            None,
        ),
    ]
}

/// Título acortado con "…" para no ensanchar el menú de ancho fijo.
pub(crate) fn shorten_title(title: &str) -> String {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return "Ventana".to_owned();
    }
    if trimmed.chars().count() <= MAX_TITLE_CHARS {
        return trimmed.to_owned();
    }
    let kept: String = trimmed.chars().take(MAX_TITLE_CHARS - 1).collect();
    format!("{}…", kept.trim_end())
}

/// Añade una sección con separador previo solo si ambas tienen contenido.
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
