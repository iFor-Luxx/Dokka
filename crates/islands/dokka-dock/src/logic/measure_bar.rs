/// Un dock sin apps conserva el ancho de un hueco para seguir siendo visible.
const MINIMUM_SLOTS: usize = 1;

/// Ancho necesario para los huecos, sus separaciones y el margen interior.
pub(crate) fn bar_width(app_count: usize, slot: f32, gap: f32, padding: f32) -> f32 {
    let slots = app_count.max(MINIMUM_SLOTS) as f32;
    slots * slot + (slots - 1.0) * gap + padding * 2.0
}
