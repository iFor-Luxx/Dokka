//! Orden de la barra superior en Preferencias: nombres y huecos.
//!
//! La fila de arriba es el icono de más a la izquierda. El golpe, el dibujo y
//! el arrastre usan los mismos huecos para caer donde se ve cada fila.

use dokka_model::{MenubarItem, MenubarPreferences};
use dokka_theme::settings as measures;

use super::content::card_heights;
use super::sections::SECTION_MENUBAR;

/// Nombre de cada icono en la lista.
pub(crate) fn menubar_label(item: MenubarItem) -> &'static str {
    match item {
        MenubarItem::Volume => "Volumen",
        MenubarItem::Network => "Red",
        MenubarItem::Music => "Música",
        MenubarItem::Language => "Idioma",
        MenubarItem::Search => "Buscar",
        MenubarItem::ControlCenter => "Centro de control",
        MenubarItem::BackgroundApps => "Apps en segundo plano",
    }
}

/// `true` si el icono está encendido en Preferencias.
pub(crate) fn menubar_on(preferences: &MenubarPreferences, item: MenubarItem) -> bool {
    match item {
        MenubarItem::Volume => preferences.volume,
        MenubarItem::Network => preferences.network,
        MenubarItem::Music => preferences.music,
        MenubarItem::Language => preferences.language,
        MenubarItem::Search => preferences.search,
        MenubarItem::ControlCenter => preferences.control_center,
        MenubarItem::BackgroundApps => preferences.background_apps,
    }
}

/// Enciende o apaga el icono en estas preferencias.
pub(crate) fn toggle_menubar(preferences: &mut MenubarPreferences, item: MenubarItem) {
    match item {
        MenubarItem::Volume => preferences.volume = !preferences.volume,
        MenubarItem::Network => preferences.network = !preferences.network,
        MenubarItem::Music => preferences.music = !preferences.music,
        MenubarItem::Language => preferences.language = !preferences.language,
        MenubarItem::Search => preferences.search = !preferences.search,
        MenubarItem::ControlCenter => {
            preferences.control_center = !preferences.control_center;
        }
        MenubarItem::BackgroundApps => {
            preferences.background_apps = !preferences.background_apps;
        }
    }
}

/// Borde superior de la tarjeta de la barra superior, si se ve.
pub(crate) fn menubar_top() -> Option<f32> {
    super::content::card_tops(SECTION_MENUBAR)
        .into_iter()
        .next()
}

/// Altura total de la tarjeta con sus siete filas.
pub(crate) fn menubar_bottom(top: f32) -> f32 {
    top + card_height_for_menubar()
}

fn card_height_for_menubar() -> f32 {
    card_heights(SECTION_MENUBAR, 0).iter().sum()
}

/// Fila bajo el punto en este orden mostrado, si cae en la tarjeta.
pub(crate) fn menubar_row_at(
    display: &[MenubarItem; 7],
    width: f32,
    x: f32,
    y: f32,
) -> Option<usize> {
    let top = menubar_top()?;
    let bottom = menubar_bottom(top);
    if !(top..bottom).contains(&y) {
        return None;
    }
    if !(measures::CONTENT_LEFT..measures::CONTENT_LEFT + width).contains(&x) {
        return None;
    }
    let slot = ((y - top) / measures::CONTENT_ROW_HEIGHT).floor() as usize;
    let slot = slot.min(6);
    if slot < display.len() {
        Some(slot)
    } else {
        None
    }
}

/// Hueco donde caería la fila arrastrada con el cursor en `y`.
pub(crate) fn reorder_target(top: f32, y: f32) -> usize {
    let clamped = y.clamp(top, top + card_height_for_menubar());
    let slot = ((clamped - top) / measures::CONTENT_ROW_HEIGHT).floor() as usize;
    slot.min(6)
}

/// Orden con la fila `from` movida al hueco `to`.
pub(crate) fn reordered(base: &[MenubarItem; 7], from: usize, to: usize) -> [MenubarItem; 7] {
    let mut kept: Vec<MenubarItem> = base.to_vec();
    let item = kept.remove(from.min(6));
    kept.insert(to.min(kept.len()), item);
    let mut order = MenubarItem::FACTORY_ORDER;
    for (slot, item) in kept.into_iter().take(7).enumerate() {
        order[slot] = item;
    }
    order
}

/// Borde superior de la fila arrastrada siguiendo al cursor, sin salirse.
pub(crate) fn float_top(top: f32, y: f32) -> f32 {
    let bottom = menubar_bottom(top);
    (y - measures::CONTENT_ROW_HEIGHT / 2.0).clamp(top, bottom - measures::CONTENT_ROW_HEIGHT)
}
