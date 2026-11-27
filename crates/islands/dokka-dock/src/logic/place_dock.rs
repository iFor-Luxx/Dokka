use dokka_interaction::Magnifier;
use dokka_model::{DockApp, DockPosition};
use dokka_scene::Rect;

use super::{Entry, list_entries, pack, position_or_bottom};
use crate::Dock;
use crate::view::dock_style as style;

/// Dónde está cada cosa en este instante.
pub(crate) struct Placement<'a> {
    pub(crate) bar: Rect,
    pub(crate) slots: Vec<(Entry<'a>, Rect)>,
    pub(crate) divider: Rect,
}

pub(crate) fn place_dock<'a>(dock: &Dock, apps: &'a [DockApp]) -> Placement<'a> {
    let (entries, before_divider) = list_entries(apps, &dock.explorer);
    // Eje a lo largo: x abajo, y en laterales. Todo se calcula aquí y `pack`
    // lo convierte a x/y en un solo sitio.
    let start = style::side_room(dock);
    let end = start + style::bar_width(dock, entries.len());
    let strength = if dock.preferences.magnification {
        dock.hover.value()
    } else {
        0.0
    };
    let magnifier = Magnifier {
        cursor: dock.cursor_along(),
        range: style::magnify_range(dock),
        maximum_scale: style::maximum_scale(dock),
        strength,
    };
    // Se reparte el crecimiento por igual a ambos lados.
    let drift = (magnifier.shift_at(start) + magnifier.shift_at(end)) / 2.0;
    let moved = |rest: f32| rest + magnifier.shift_at(rest) - drift;

    let step = style::slot(dock) + style::gap(dock);
    // Posición en reposo del borde inicial de cada hueco.
    let slot_start = |index: usize| {
        let after = if index >= before_divider {
            style::divider_space(dock)
        } else {
            0.0
        };
        start + style::padding(dock) + index as f32 * step + after
    };
    // Cruce hacia dentro: la barra toca el borde en laterales izquierdos,
    // y deja la reserva por delante en el resto.
    let bar_cross = match position_or_bottom(dock) {
        DockPosition::Left => 0.0,
        _ => style::headroom(dock),
    };
    // Margen para centrar el icono en reposo dentro del grosor.
    let inset = (style::height(dock) - style::slot(dock)) / 2.0;
    let thickness = style::height(dock);
    let slots = entries
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            let rest = slot_start(index) + style::slot(dock) / 2.0;
            let side = style::slot(dock) * magnifier.scale_at(rest);
            let lift = match entry {
                Entry::App(app) | Entry::Explorer(app) => dock
                    .bounces
                    .get(&app.info.id)
                    .map_or(0.0, |bounce| bounce.lift(style::bounce_height(dock))),
                _ => 0.0,
            };
            // Hacia dentro: abajo y derecha retroceden, izquierda avanza.
            // El borde del lado del dock conserva su aire; el interior crece.
            let cross = match position_or_bottom(dock) {
                DockPosition::Left => bar_cross + inset + lift,
                _ => bar_cross + thickness - inset - side - lift,
            };
            let bounds = pack(dock, moved(rest) - side / 2.0, cross, side, side);
            (entry, bounds)
        })
        .collect();

    let divider_rest = start
        + style::padding(dock)
        + before_divider as f32 * step
        + (style::divider_space(dock) - style::gap(dock)) / 2.0;
    let bar = pack(
        dock,
        moved(start),
        bar_cross,
        moved(end) - moved(start),
        style::height(dock),
    );
    let divider = pack(
        dock,
        moved(divider_rest),
        bar_cross + style::divider_inset(dock),
        dokka_theme::dock::DIVIDER_WIDTH,
        style::height(dock) - style::divider_inset(dock) * 2.0,
    );
    // Con vertical el grosor y la línea quedan horizontales gracias a `pack`.
    Placement { bar, slots, divider }
}
