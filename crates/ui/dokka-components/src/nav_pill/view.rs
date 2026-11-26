use std::sync::Arc;

use dokka_scene::{Divider, Icon, Panel, Rect};
use dokka_theme::{Theme, nav_pill as measures};

use super::icons::NavPillIcons;
use super::props::{NavHalf, NavPillProps};

/// Mitades izquierda y derecha de la cápsula.
fn halves(bounds: Rect) -> (Rect, Rect) {
    let half = bounds.width / 2.0;
    (
        Rect {
            width: half,
            ..bounds
        },
        Rect {
            x: bounds.x + half,
            width: bounds.width - half,
            ..bounds
        },
    )
}

/// Cápsula gris muy clara con borde fino y luminoso.
pub fn nav_pill_panel(theme: &Theme, props: &NavPillProps) -> Panel {
    Panel {
        bounds: props.bounds,
        corner_radius: measures::radius(props.bounds.height),
        border_width: measures::BORDER_WIDTH,
        tint: theme.nav_pill.background,
        border: theme.nav_pill.border,
    }
}

/// Resaltado suave de la mitad bajo el cursor. Existe siempre, con el mismo
/// estilo: de tamaño cero cuando no hay mitad pulsable bajo el cursor, para
/// no reconstruir capas al moverse (igual que el hover de las filas).
/// Es una cápsula con margen interior dentro de su mitad, sin tocar la línea
/// central ni salirse del borde. Al ser plano se ve como sombra neutra.
pub fn nav_pill_highlight(theme: &Theme, props: &NavPillProps) -> Panel {
    let highlight_radius = measures::radius(props.bounds.height - measures::HOVER_INSET * 2.0);
    let hidden = Panel {
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        corner_radius: highlight_radius,
        border_width: 0.0,
        tint: theme.nav_pill.hover,
        border: theme.nav_pill.hover,
    };
    let (previous, next) = halves(props.bounds);
    let area = match props.hovered {
        Some(NavHalf::Previous) if props.can_previous => previous,
        Some(NavHalf::Next) if props.can_next => next,
        _ => return hidden,
    };
    let inset = measures::HOVER_INSET;
    Panel {
        bounds: Rect {
            x: area.x + inset,
            y: area.y + inset,
            width: area.width - inset * 2.0,
            height: area.height - inset * 2.0,
        },
        corner_radius: highlight_radius,
        border_width: 0.0,
        tint: theme.nav_pill.hover,
        border: theme.nav_pill.hover,
    }
}

/// Línea vertical fina y corta que separa ambas mitades.
pub fn nav_pill_divider(theme: &Theme, props: &NavPillProps) -> Divider {
    let height = measures::divider_height(props.bounds.height);
    Divider {
        bounds: Rect {
            x: props.bounds.x + props.bounds.width / 2.0 - measures::DIVIDER_WIDTH / 2.0,
            y: props.bounds.y + (props.bounds.height - height) / 2.0,
            width: measures::DIVIDER_WIDTH,
            height,
        },
        color: theme.nav_pill.divider,
    }
}

/// Chevrones oscuros centrados en cada mitad; el lado no disponible, atenuado.
pub fn nav_pill_icons(theme: &Theme, drawings: &NavPillIcons, props: &NavPillProps) -> Vec<Icon> {
    let (previous_area, next_area) = halves(props.bounds);
    [
        (previous_area, drawings.previous(props.can_previous)),
        (next_area, drawings.next(props.can_next)),
    ]
    .iter()
    .map(|(area, drawing)| Icon {
        x: area.x + (area.width - measures::ICON_SIDE) / 2.0,
        y: area.y + (area.height - measures::ICON_SIDE) / 2.0,
        side: measures::ICON_SIDE,
        image: Some(dokka_scene::IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.nav_pill.icon,
        placeholder_radius: 0.0,
    })
    .collect()
}

/// Mitad pulsable que hay en el punto, si cae en la píldora y está disponible.
pub fn nav_pill_at(props: &NavPillProps, x: f32, y: f32) -> Option<NavHalf> {
    let bounds = props.bounds;
    if !(bounds.x..bounds.x + bounds.width).contains(&x)
        || !(bounds.y..bounds.y + bounds.height).contains(&y)
    {
        return None;
    }
    if x < bounds.x + bounds.width / 2.0 {
        props.can_previous.then_some(NavHalf::Previous)
    } else {
        props.can_next.then_some(NavHalf::Next)
    }
}

/// `true` si el punto cae en la píldora, esté disponible o no.
pub fn nav_pill_contains(bounds: Rect, x: f32, y: f32) -> bool {
    (bounds.x..bounds.x + bounds.width).contains(&x)
        && (bounds.y..bounds.y + bounds.height).contains(&y)
}
