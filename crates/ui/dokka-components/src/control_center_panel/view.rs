use dokka_scene::{Overlay, Panel, Rect, Size};
use dokka_theme::{Theme, control_center as measures};

use super::ControlCenterProps;
use super::capsules::add_capsules;
use super::circles::add_circles;
use super::icons::ControlCenterIcons;
use super::music::add_music;
use super::paint::Pieces;
use super::slider::add_slider;

/// Tamaño fijo del panel.
pub fn control_center_size() -> Size {
    Size {
        width: measures::WIDTH,
        height: measures::HEIGHT,
    }
}

/// Rejilla Tahoe de cuatro columnas sobre fondo invisible: la cápsula de red
/// con dos círculos blancos a la izquierda, la música a la derecha, la
/// cápsula de Preferencias con un círculo debajo y Sonido a todo lo ancho.
/// Cada zona añade lo suyo; todo existe siempre: lo oculto mide cero o vacío.
pub fn control_center_panel(
    theme: &Theme,
    icons: &ControlCenterIcons,
    bounds: Rect,
    props: &ControlCenterProps,
) -> Overlay {
    let mut pieces = Pieces::new();
    add_capsules(&mut pieces, theme, bounds, props, icons);
    add_music(&mut pieces, theme, bounds, props, icons);
    add_circles(&mut pieces, theme, bounds, icons);
    add_slider(&mut pieces, theme, bounds, props, icons);
    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::CORNER_RADIUS,
            border_width: 0.0,
            tint: theme.tray.clear,
            border: theme.tray.clear,
        },
        tiles: pieces.tiles,
        pill: None,
        highlight: None,
        flat_panels: pieces.flats,
        dividers: Vec::new(),
        dots: Vec::new(),
        pointer: None,
        texts: pieces.texts,
        icons: pieces.icons,
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}
