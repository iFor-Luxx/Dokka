//! Menú de una app normal: ventanas, abrir, opciones, fijar, cerrar.

use dokka_components::menu_panel::MenuRow;
use dokka_model::{AppWindow, DockApp, WindowId};

use super::dock_menu::{MenuData, SubmenuData};
use super::menu::MenuAction;

/// Ventanas que se listan como máximo, para que el menú quepa.
pub(crate) const MAX_WINDOWS: usize = 6;
/// Caracteres de título antes de recortar con "…"; el ancho del menú es fijo.
pub(crate) const MAX_TITLE_CHARS: usize = 30;

/// Menú de Preferencias: solo traer al frente y ocultar. Sin abrir otra
/// instancia, sin fijar y sin salir: cerrar solo oculta la ventana.
pub(crate) fn settings_menu(app: &DockApp) -> MenuData {
    let mut rows = Vec::new();
    let mut actions: Vec<Option<MenuAction>> = Vec::new();
    let mut children: Vec<Option<SubmenuData>> = Vec::new();

    push_windows(&mut rows, &mut actions, &mut children, &app.windows);
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [(
            MenuRow::item("Cerrar ventana"),
            Some(MenuAction::CloseFrontWindow(
                app.windows.iter().map(|window| window.id).collect(),
            )),
            None,
        )],
    );
    MenuData {
        rows,
        actions,
        children,
    }
}

/// Una fila por ventana (máximo 6), abrir, administrador, opciones, fijar, cerrar.
pub(crate) fn app_menu(app: &DockApp) -> MenuData {
    if super::settings_app::is_settings_app(app) {
        return settings_menu(app);
    }
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
                MenuRow::item(open_label(app)),
                Some(MenuAction::OpenApp(app.info.clone())),
                None,
            ),
            (
                MenuRow::item("Ejecutar como administrador"),
                Some(MenuAction::RunAsAdministrator(app.info.clone())),
                None,
            ),
        ],
    );
    push_section(
        &mut rows,
        &mut actions,
        &mut children,
        [(
            MenuRow::submenu("Opciones"),
            None,
            Some(SubmenuData {
                rows: vec![MenuRow::item("Abrir ubicación del archivo")],
                actions: vec![Some(MenuAction::ShowInFolder(app.info.clone()))],
            }),
        )],
    );
    push_section(&mut rows, &mut actions, &mut children, [pin_row(app)]);
    if app.is_running() {
        push_section(&mut rows, &mut actions, &mut children, close_rows(app));
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
pub(crate) fn pin_row(app: &DockApp) -> (MenuRow, Option<MenuAction>, Option<SubmenuData>) {
    if app.is_pinned {
        (
            MenuRow::item("Quitar del dock"),
            Some(MenuAction::Unpin(app.info.clone())),
            None,
        )
    } else {
        (
            MenuRow::item("Fijar en el dock"),
            Some(MenuAction::Pin(app.info.clone())),
            None,
        )
    }
}

/// "Cerrar ventana" (la más al frente) y "Salir" (cierra todas, sin matar).
pub(crate) fn close_rows(app: &DockApp) -> Vec<(MenuRow, Option<MenuAction>, Option<SubmenuData>)> {
    let ids: Vec<WindowId> = app.windows.iter().map(|window| window.id).collect();
    vec![
        (
            MenuRow::item("Cerrar ventana"),
            Some(MenuAction::CloseFrontWindow(ids.clone())),
            None,
        ),
        (MenuRow::item("Salir"), Some(MenuAction::QuitApp(ids)), None),
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
