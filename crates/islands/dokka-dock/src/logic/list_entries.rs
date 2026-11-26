use dokka_model::{AppId, DockApp, WindowId};

/// Una cosa que ocupa un lugar en el dock.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Entry<'a> {
    Explorer(&'a DockApp),
    Launchpad,
    Calendar,
    Clock,
    App(&'a DockApp),
    /// Ventana minimizada de una app.
    Minimized(&'a DockApp, WindowId),
    Trash,
}

/// Entradas en orden. El número indica cuántas van antes de la línea separadora.
pub(crate) fn list_entries<'a>(apps: &'a [DockApp], explorer: &AppId) -> (Vec<Entry<'a>>, usize) {
    let is_explorer = |app: &&DockApp| &app.info.id == explorer;
    let mut entries: Vec<Entry<'a>> = apps
        .iter()
        .find(is_explorer)
        .map(Entry::Explorer)
        .into_iter()
        .collect();
    entries.extend([Entry::Launchpad, Entry::Calendar, Entry::Clock]);

    // Primero las fijadas, para que no cambien de sitio al abrir y cerrar otras.
    let others = || apps.iter().filter(|app| !is_explorer(app));
    entries.extend(others().filter(|app| app.is_pinned).map(Entry::App));
    entries.extend(others().filter(|app| !app.is_pinned).map(Entry::App));
    let before_divider = entries.len();

    entries.extend(apps.iter().flat_map(|app| {
        app.windows
            .iter()
            .filter(|window| window.is_minimized)
            .map(move |window| Entry::Minimized(app, window.id))
    }));
    entries.push(Entry::Trash);
    (entries, before_divider)
}
