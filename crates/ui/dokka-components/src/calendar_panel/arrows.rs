use std::sync::Arc;

use dokka_icons::{chevron_icon, chevron_left_icon};
use dokka_scene::{Drawing, Icon, IconImage, Rect};
use dokka_theme::{Theme, popup as measures};

use super::calendar_arrows;

/// Flechas del calendario ya construidas con el color del tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo.
#[derive(Debug)]
pub struct CalendarArrows {
    previous: Arc<Drawing>,
    next: Arc<Drawing>,
}

impl CalendarArrows {
    pub fn new(theme: &Theme) -> Self {
        Self {
            previous: Arc::new(chevron_left_icon(theme.panel.text)),
            next: Arc::new(chevron_icon(theme.panel.text)),
        }
    }
}

/// Iconos de las flechas, centrados en sus zonas pulsables.
/// Las zonas no cambian: siguen siendo las de `calendar_arrows`.
pub(super) fn arrow_icons(theme: &Theme, arrows: &CalendarArrows, bounds: Rect) -> Vec<Icon> {
    let (previous_area, next_area) = calendar_arrows(bounds);
    [(previous_area, &arrows.previous), (next_area, &arrows.next)]
        .iter()
        .map(|(area, drawing)| Icon {
            x: area.x + (area.width - measures::CALENDAR_ARROW_ICON_SIDE) / 2.0,
            y: area.y + (area.height - measures::CALENDAR_ARROW_ICON_SIDE) / 2.0,
            side: measures::CALENDAR_ARROW_ICON_SIDE,
            image: Some(IconImage::Drawing(Arc::clone(drawing))),
            placeholder: theme.panel.background,
            placeholder_radius: 0.0,
        })
        .collect()
}
