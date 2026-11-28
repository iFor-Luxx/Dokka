use std::collections::HashMap;

use dokka_model::{AppId, DockApp};
use dokka_scene::Rect;

use super::{Entry, Placement, clock_label, trash_label};
use crate::Dock;
use crate::view::dock_style as style;

/// Hueco bajo el cursor al que señalaría la etiqueta, sin el texto.
/// Es la condición compartida: escondido no hay etiqueta, ni con menú.
pub(crate) fn hovered_slot<'a>(
    dock: &Dock,
    placement: &Placement<'a>,
) -> Option<(Entry<'a>, Rect)> {
    if dock.hide.is_hidden() {
        return None;
    }
    if !dock.is_hovered || dock.is_menu_open() {
        return None;
    }
    let reach = style::gap(dock) / 2.0;
    // A lo largo del dock: x abajo, y en laterales.
    let along = if super::is_vertical(dock) {
        dock.cursor_y
    } else {
        dock.cursor_x
    };
    let &(entry, slot) = placement.slots.iter().find(|(_, slot)| {
        let (start, end) = if super::is_vertical(dock) {
            (slot.y - reach, slot.y + slot.height + reach)
        } else {
            (slot.x - reach, slot.x + slot.width + reach)
        };
        along >= start && along <= end
    })?;
    // Con ventanas abiertas aparece directamente la vista previa, que ya las nombra.
    match entry {
        Entry::Explorer(app) | Entry::App(app) if app.is_running() => return None,
        Entry::Minimized(..) => return None,
        // El calendario abre su panel directamente, sin etiqueta previa.
        Entry::Calendar => return None,
        _ => {}
    }
    Some((entry, slot))
}

/// Texto de la etiqueta bajo el cursor y el hueco al que señala.
pub(crate) fn hovered_label(
    dock: &Dock,
    placement: &Placement<'_>,
    names: &HashMap<AppId, String>,
) -> Option<(String, Rect)> {
    let (entry, slot) = hovered_slot(dock, placement)?;
    let app_name = |app: &DockApp| names.get(&app.info.id).unwrap_or(&app.info.name).to_owned();
    let text = match entry {
        Entry::Explorer(_) => "Explorador".to_owned(),
        Entry::App(app) => app_name(app),
        Entry::Launchpad => "Launchpad".to_owned(),
        Entry::Clock => clock_label(
            dock.icons.time.hour,
            dock.icons.time.minute,
            dock.clock_24,
        ),
        Entry::Trash => trash_label(dock.icons.trash_size),
        // Vista previa o panel propio: ya filtrado en `hovered_slot`.
        _ => return None,
    };
    Some((text, slot))
}
