use dokka_interaction::Magnifier;
use dokka_model::DockApp;
use dokka_scene::Rect;

use super::{Entry, list_entries};
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
    let left = style::side_room(dock);
    let right = left + style::bar_width(dock, entries.len());
    let strength = if dock.preferences.magnification {
        dock.hover.value()
    } else {
        0.0
    };
    let magnifier = Magnifier {
        cursor: dock.cursor_x,
        range: style::magnify_range(dock),
        maximum_scale: style::maximum_scale(dock),
        strength,
    };
    // Se reparte el crecimiento por igual a ambos lados.
    let drift = (magnifier.shift_at(left) + magnifier.shift_at(right)) / 2.0;
    let moved = |rest: f32| rest + magnifier.shift_at(rest) - drift;

    let step = style::slot(dock) + style::gap(dock);
    // Posición en reposo del borde izquierdo de cada hueco.
    let slot_start = |index: usize| {
        let after = if index >= before_divider {
            style::divider_space(dock)
        } else {
            0.0
        };
        left + style::padding(dock) + index as f32 * step + after
    };
    let baseline = style::headroom(dock) + (style::height(dock) + style::slot(dock)) / 2.0;
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
            let bounds = Rect {
                x: moved(rest) - side / 2.0,
                y: baseline - side - lift,
                width: side,
                height: side,
            };
            (entry, bounds)
        })
        .collect();

    let divider_rest = left
        + style::padding(dock)
        + before_divider as f32 * step
        + (style::divider_space(dock) - style::gap(dock)) / 2.0;
    Placement {
        bar: Rect {
            x: moved(left),
            y: style::headroom(dock),
            width: moved(right) - moved(left),
            height: style::height(dock),
        },
        slots,
        divider: Rect {
            x: moved(divider_rest),
            y: style::headroom(dock) + style::divider_inset(dock),
            width: dokka_theme::dock::DIVIDER_WIDTH,
            height: style::height(dock) - style::divider_inset(dock) * 2.0,
        },
    }
}
