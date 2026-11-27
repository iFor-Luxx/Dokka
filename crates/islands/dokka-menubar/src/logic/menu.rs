//! Filas fijas del menú del icono y sitio del panel colgando.

use dokka_components::menu_panel::{MenuRow, menu_size};
use dokka_scene::Rect;
use dokka_theme::menubar;

use crate::MenubarAction;

/// Filas del menú: Preferencias, escritorio y tareas, bloqueo y apagado,
/// reiniciar y salir, con sus separadores.
pub(crate) fn menu_rows() -> Vec<MenuRow> {
    vec![
        MenuRow::item("Preferencias…"),
        MenuRow::Separator,
        MenuRow::item("Mostrar el escritorio"),
        MenuRow::item("Administrador de tareas"),
        MenuRow::Separator,
        MenuRow::item("Bloquear pantalla"),
        MenuRow::item("Apagar o reiniciar…"),
        MenuRow::Separator,
        MenuRow::item("Reiniciar Dokka"),
        MenuRow::item("Salir de Dokka"),
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
