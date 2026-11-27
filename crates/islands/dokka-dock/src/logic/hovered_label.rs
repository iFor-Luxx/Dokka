use std::collections::HashMap;

use dokka_model::{AppId, DockApp};
use dokka_scene::Rect;

use super::{Entry, Placement, clock_label, trash_label};
use crate::Dock;
use crate::view::dock_style as style;

/// Texto de la etiqueta bajo el cursor y el hueco al que señala.
pub(crate) fn hovered_label(
    dock: &Dock,
    placement: &Placement<'_>,
    names: &HashMap<AppId, String>,
) -> Option<(String, Rect)> {
    if !dock.is_hovered || dock.is_menu_open() {
        return None;
    }
    let reach = style::gap(dock) / 2.0;
    let &(entry, slot) = placement.slots.iter().find(|(_, slot)| {
        dock.cursor_x >= slot.x - reach && dock.cursor_x <= slot.x + slot.width + reach
    })?;

    let app_name = |app: &DockApp| names.get(&app.info.id).unwrap_or(&app.info.name).to_owned();
    let text = match entry {
        // Con ventanas abiertas aparece directamente la vista previa, que ya las nombra.
        Entry::Explorer(app) | Entry::App(app) if app.is_running() => return None,
        Entry::Minimized(..) => return None,
        // El calendario abre su panel directamente, sin etiqueta previa.
        Entry::Calendar => return None,
        Entry::Explorer(_) => "Explorador".to_owned(),
        Entry::App(app) => app_name(app),
        Entry::Launchpad => "Launchpad".to_owned(),
        Entry::Clock => clock_label(dock.icons.time.hour, dock.icons.time.minute),
        Entry::Trash => trash_label(dock.icons.trash_size),
    };
    Some((text, slot))
}
