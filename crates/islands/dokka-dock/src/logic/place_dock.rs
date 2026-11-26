use dokka_interaction::Magnifier;
use dokka_model::DockApp;
use dokka_scene::Rect;

use super::bar_width;
use crate::Dock;
use crate::view::dock_style as style;

/// Dónde está cada cosa en este instante.
pub(crate) struct Placement {
    pub(crate) bar: Rect,
    /// Un rectángulo por app, en el mismo orden.
    pub(crate) icons: Vec<Rect>,
}

pub(crate) fn place_dock(dock: &Dock, apps: &[DockApp]) -> Placement {
    let width = bar_width(apps.len(), style::SLOT, style::GAP, style::PADDING);
    let left = style::side_room();
    let right = left + width;
    let magnifier = Magnifier {
        cursor: dock.cursor_x,
        range: style::magnify_range(),
        maximum_scale: style::MAXIMUM_SCALE,
        strength: dock.hover.value(),
    };
    // Se reparte el crecimiento por igual a ambos lados.
    let drift = (magnifier.shift_at(left) + magnifier.shift_at(right)) / 2.0;
    let new_left = left + magnifier.shift_at(left) - drift;
    let new_right = right + magnifier.shift_at(right) - drift;

    let baseline = style::headroom() + (style::HEIGHT + style::SLOT) / 2.0;
    let icons = apps
        .iter()
        .enumerate()
        .map(|(index, app)| {
            let rest = left
                + style::PADDING
                + index as f32 * (style::SLOT + style::GAP)
                + style::SLOT / 2.0;
            let side = style::SLOT * magnifier.scale_at(rest);
            let center = rest + magnifier.shift_at(rest) - drift;
            let lift = dock
                .bounces
                .get(&app.info.id)
                .map_or(0.0, |bounce| bounce.lift(style::BOUNCE_HEIGHT));
            Rect {
                x: center - side / 2.0,
                y: baseline - side - lift,
                width: side,
                height: side,
            }
        })
        .collect();

    Placement {
        bar: Rect {
            x: new_left,
            y: style::headroom(),
            width: new_right - new_left,
            height: style::HEIGHT,
        },
        icons,
    }
}
