//! Tirador redondo blanco sobre la barra, claramente mayor que ella.
//! Queda centrado en vertical y su recorrido está recortado para no
//! salirse de la barra ni en 0% ni en 100%. Con silencio va atenuado.

use dokka_scene::{Dot, Rect};
use dokka_theme::{Theme, volume as measures};

use super::VolumePanelProps;
use super::layout::knob_center;

/// Tirador sobre la barra, en el centro del nivel.
pub(crate) fn slider_knob(theme: &Theme, bar: Rect, props: &VolumePanelProps) -> Dot {
    let center = knob_center(bar, props.level);
    let diameter = measures::THUMB_DIAMETER.max(0.0);
    Dot {
        x: center - diameter / 2.0,
        y: bar.y + (bar.height - diameter) / 2.0,
        diameter,
        color: if props.is_muted {
            theme.tray.secondary
        } else {
            theme.tray.fill
        },
    }
}
