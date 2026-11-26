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
    let left = style::side_room();
    let right = left + style::bar_width(entries.len());
    let magnifier = Magnifier {
        cursor: dock.cursor_x,
        range: style::magnify_range(),
        maximum_scale: style::MAXIMUM_SCALE,
        strength: dock.hover.value(),
    };
    // Se reparte el crecimiento por igual a ambos lados.
    let drift = (magnifier.shift_at(left) + magnifier.shift_at(right)) / 2.0;
    let moved = |rest: f32| rest + magnifier.shift_at(rest) - drift;

    let step = style::SLOT + style::GAP;
    // Posición en reposo del borde izquierdo de cada hueco.
    let slot_start = |index: usize| {
        let after = if index >= before_divider {
            style::DIVIDER_SPACE
        } else {
            0.0
        };
        left + style::PADDING + index as f32 * step + after
    };
    let baseline = style::headroom() + (style::HEIGHT + style::SLOT) / 2.0;
    let slots = entries
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            let rest = slot_start(index) + style::SLOT / 2.0;
            let side = style::SLOT * magnifier.scale_at(rest);
            let lift = match entry {
                Entry::App(app) | Entry::Explorer(app) => dock
                    .bounces
                    .get(&app.info.id)
                    .map_or(0.0, |bounce| bounce.lift(style::BOUNCE_HEIGHT)),
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
        + style::PADDING
        + before_divider as f32 * step
        + (style::DIVIDER_SPACE - style::GAP) / 2.0;
    Placement {
        bar: Rect {
            x: moved(left),
            y: style::headroom(),
            width: moved(right) - moved(left),
            height: style::HEIGHT,
        },
        slots,
        divider: Rect {
            x: moved(divider_rest),
            y: style::headroom() + style::DIVIDER_INSET,
            width: 1.0,
            height: style::HEIGHT - style::DIVIDER_INSET * 2.0,
        },
    }
}
