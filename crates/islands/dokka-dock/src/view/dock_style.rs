// Provisional: estos valores pasarán a `dokka-theme`.

use dokka_scene::Color;

/// Distancia entre el dock y el borde inferior del área de trabajo.
pub const DOCK_BOTTOM_MARGIN: f32 = 8.0;

pub(crate) const HEIGHT: f32 = 80.0;
pub(crate) const SLOT: f32 = 52.0;
pub(crate) const GAP: f32 = 12.0;
pub(crate) const PADDING: f32 = 14.0;
pub(crate) const CORNER_RADIUS: f32 = 24.0;
pub(crate) const TINT: Color = Color::rgba(255, 255, 255, 40);
pub(crate) const BORDER: Color = Color::rgba(255, 255, 255, 96);
pub(crate) const PLACEHOLDER: Color = Color::rgba(255, 255, 255, 80);
pub(crate) const PLACEHOLDER_RADIUS: f32 = 12.0;

/// Ancho extra que ocupa la línea separadora y su aire.
pub(crate) const DIVIDER_SPACE: f32 = 13.0;
pub(crate) const DIVIDER_INSET: f32 = 14.0;
pub(crate) const DIVIDER_COLOR: Color = Color::rgba(255, 255, 255, 110);

/// Tamaño de la insignia respecto al hueco, y qué parte queda dentro de él.
pub(crate) const BADGE_SIZE: f32 = 0.44;
pub(crate) const BADGE_INSIDE: f32 = 0.85;

pub(crate) const MAXIMUM_SCALE: f32 = 1.6;
/// Iconos afectados a cada lado del cursor.
const MAGNIFIED_NEIGHBOURS: f32 = 3.0;
pub(crate) const BOUNCE_HEIGHT: f32 = 26.0;

pub(crate) const DOT_DIAMETER: f32 = 5.0;
pub(crate) const DOT_FROM_BOTTOM: f32 = 10.0;
pub(crate) const DOT_COLOR: Color = Color::rgba(255, 255, 255, 220);

/// Ancho en reposo de una barra con `slots` iconos y su separador.
pub(crate) fn bar_width(slots: usize) -> f32 {
    let slots = slots.max(1) as f32;
    slots * SLOT + (slots - 1.0) * GAP + PADDING * 2.0 + DIVIDER_SPACE
}

pub(crate) fn magnify_range() -> f32 {
    MAGNIFIED_NEIGHBOURS * (SLOT + GAP)
}

/// Espacio sobre la barra para la lupa y el salto.
pub(crate) fn headroom() -> f32 {
    (SLOT * (MAXIMUM_SCALE - 1.0) + BOUNCE_HEIGHT + 4.0).ceil()
}

/// Espacio a cada lado: la mitad de lo que puede ensancharse la barra.
pub(crate) fn side_room() -> f32 {
    ((MAXIMUM_SCALE - 1.0) * magnify_range() / 2.0).ceil() + 2.0
}
