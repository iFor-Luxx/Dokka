//! Cuentas geométricas del dock. Los valores vienen de `dokka-theme`.

pub use dokka_theme::dock::*;
use dokka_theme::popup;
use dokka_theme::tooltip;

/// Ancho en reposo de una barra con `slots` iconos y su separador.
pub(crate) fn bar_width(slots: usize) -> f32 {
    let slots = slots.max(1) as f32;
    slots * SLOT + (slots - 1.0) * GAP + PADDING * 2.0 + DIVIDER_SPACE
}

pub(crate) fn magnify_range() -> f32 {
    MAGNIFIED_NEIGHBOURS * (SLOT + GAP)
}

/// Espacio sobre la barra que usan la lupa, el salto y la etiqueta.
pub(crate) fn rise_space() -> f32 {
    (SLOT * (MAXIMUM_SCALE - 1.0) + BOUNCE_HEIGHT + tooltip::RESERVED_HEIGHT).ceil()
}

/// Todo el espacio sobre la barra, incluido el de los paneles flotantes.
pub(crate) fn headroom() -> f32 {
    rise_space() + popup::SPACE
}

/// Espacio a cada lado: la mitad de lo que puede ensancharse la barra.
pub(crate) fn side_room() -> f32 {
    ((MAXIMUM_SCALE - 1.0) * magnify_range() / 2.0).ceil() + 2.0
}
