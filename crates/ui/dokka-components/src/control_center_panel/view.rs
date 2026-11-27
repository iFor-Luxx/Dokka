use dokka_scene::{Overlay, Panel, Rect, Size};
use dokka_theme::{Theme, control_center as measures};

use super::ControlCenterProps;
use super::brightness::add_brightness;
use super::capsules::add_capsules;
use super::circles::add_circles;
use super::icons::ControlCenterIcons;
use super::music::add_music;
use super::paint::Pieces;
use super::slider::add_slider;

/// Tamaño del panel: con brillo una tarjeta más y su hueco.
/// Sin brillo es exactamente el de antes.
pub fn control_center_size(brightness: Option<f32>) -> Size {
    Size {
        width: measures::WIDTH,
        height: if brightness.is_some() {
            measures::HEIGHT_WITH_BRIGHTNESS
        } else {
            measures::HEIGHT
        },
    }
}

/// Rejilla Tahoe de cuatro columnas sobre fondo invisible: cápsula de red
/// con dos círculos blancos a la izquierda, música a la derecha (dos por
/// dos), cápsula de Preferencias con dos círculos debajo, Pantalla a todo lo
/// ancho si hay brillo DDC, Sonido a todo lo ancho y cuatro círculos al final.
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
    add_circles(&mut pieces, theme, bounds, props, icons);
    add_brightness(&mut pieces, theme, bounds, props, icons);
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
        panel_shadow: None,
        tile_shadow: Some(dokka_scene::Shadow {
            color: theme.tray.shadow,
            blur: measures::SHADOW_BLUR,
            offset: (0.0, measures::SHADOW_OFFSET_Y),
        }),
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
