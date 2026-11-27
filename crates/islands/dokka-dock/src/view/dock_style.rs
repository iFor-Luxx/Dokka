//! Cuentas geométricas del dock. La base viene de `dokka-theme`; el tamaño
//! final multiplica por lo ajustado en Preferencias (`icon_scale`).
//!
//! Con los valores de fábrica (escala 1, lupa completa) el resultado es el
//! mismo que el de las constantes del tema.

use dokka_model::{DOCK_ICON_SCALE_RANGE, DOCK_MAGNIFICATION_RANGE};
use dokka_theme::{dock as theme, menu, popup, tooltip};

pub use theme::{DOCK_BOTTOM_MARGIN, DOCK_EDGE_MARGIN};

use crate::Dock;

fn icon_scale(dock: &Dock) -> f32 {
    dock.preferences
        .icon_scale
        .clamp(DOCK_ICON_SCALE_RANGE.0, DOCK_ICON_SCALE_RANGE.1)
}

fn strength(dock: &Dock) -> f32 {
    dock.preferences
        .magnification_strength
        .clamp(DOCK_MAGNIFICATION_RANGE.0, DOCK_MAGNIFICATION_RANGE.1)
}

/// Escala efectiva de la lupa: 1 sin lupa, o 1 + (MAX-1) * fuerza.
pub(crate) fn maximum_scale(dock: &Dock) -> f32 {
    if !dock.preferences.magnification {
        return 1.0;
    }
    1.0 + (theme::MAXIMUM_SCALE - 1.0) * strength(dock)
}

pub(crate) fn slot(dock: &Dock) -> f32 {
    theme::SLOT * icon_scale(dock) * dock.fit()
}

pub(crate) fn gap(dock: &Dock) -> f32 {
    theme::GAP * icon_scale(dock) * dock.fit()
}

pub(crate) fn padding(dock: &Dock) -> f32 {
    theme::PADDING * icon_scale(dock) * dock.fit()
}

pub(crate) fn height(dock: &Dock) -> f32 {
    theme::HEIGHT * icon_scale(dock)
}

pub(crate) fn divider_space(dock: &Dock) -> f32 {
    theme::DIVIDER_SPACE * icon_scale(dock) * dock.fit()
}

pub(crate) fn divider_inset(dock: &Dock) -> f32 {
    theme::DIVIDER_INSET * icon_scale(dock)
}

pub(crate) fn corner_radius(dock: &Dock) -> f32 {
    theme::CORNER_RADIUS * icon_scale(dock)
}

pub(crate) fn placeholder_radius(dock: &Dock) -> f32 {
    theme::PLACEHOLDER_RADIUS * icon_scale(dock)
}

pub(crate) fn dot_diameter(dock: &Dock) -> f32 {
    theme::DOT_DIAMETER * icon_scale(dock) * dock.fit()
}

pub(crate) fn dot_from_bottom(dock: &Dock) -> f32 {
    theme::DOT_FROM_BOTTOM * icon_scale(dock)
}

pub(crate) fn bounce_height(dock: &Dock) -> f32 {
    theme::BOUNCE_HEIGHT * icon_scale(dock)
}

/// Ancho en reposo de una barra con `slots` iconos y su separador.
pub(crate) fn bar_width(dock: &Dock, slots: usize) -> f32 {
    let slots = slots.max(1) as f32;
    slots * slot(dock) + (slots - 1.0) * gap(dock) + padding(dock) * 2.0 + divider_space(dock)
}

pub(crate) fn magnify_range(dock: &Dock) -> f32 {
    theme::MAGNIFIED_NEIGHBOURS * (slot(dock) + gap(dock))
}

/// Espacio sobre la barra que usan la lupa, el salto y la etiqueta.
pub(crate) fn rise_space(dock: &Dock) -> f32 {
    (slot(dock) * (maximum_scale(dock) - 1.0) + bounce_height(dock) + tooltip::RESERVED_HEIGHT)
        .ceil()
}

/// Todo el espacio hacia dentro, incluido el de los paneles flotantes.
/// En vertical reserva además el ancho de la vista previa más ancha.
pub(crate) fn headroom(dock: &Dock) -> f32 {
    if crate::logic::is_vertical(dock) {
        let preview_width = popup::PREVIEW_SLOT_WIDTH * popup::PREVIEW_MAXIMUM_WINDOWS as f32
            + popup::PREVIEW_PADDING * 2.0
            + popup::GAP;
        rise_space(dock) + preview_width.max(popup::SPACE)
    } else {
        rise_space(dock) + popup::SPACE
    }
}

/// Espacio a cada lado por lo que puede ensancharse la barra con la lupa.
pub(crate) fn magnify_room(dock: &Dock) -> f32 {
    ((maximum_scale(dock) - 1.0) * magnify_range(dock) / 2.0).ceil() + 2.0
}

/// Margen lateral de la ventana: lo mayor entre la lupa y el menú con su
/// submenú a la derecha, para que quepan aunque el dock tenga pocos iconos.
pub(crate) fn side_room(dock: &Dock) -> f32 {
    magnify_room(dock).max(menu::WIDTH + menu::SUBMENU_GAP)
}
