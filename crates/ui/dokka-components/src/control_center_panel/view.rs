use dokka_model::HiddenControls;
use dokka_scene::{Overlay, Panel, Rect, Size};
use dokka_theme::{Theme, control_center as measures};

use super::ControlCenterProps;
use super::brightness::add_brightness;
use super::capsules::add_capsules;
use super::circles::add_circles;
use super::edit::{add_edit_chrome, total_height};
use super::icons::ControlCenterIcons;
use super::layout::arrange;
use super::music::add_music;
use super::paint::Pieces;
use super::slider::add_slider;

/// Tamaño del panel: rejilla repartida según los ocultos, más la burbuja de
/// editar bajo ella (solo sin editar); con todo oculto solo queda el aire
/// exterior más la burbuja.
pub fn control_center_size(hidden: HiddenControls, brightness: Option<f32>, editing: bool) -> Size {
    let dummy = Rect {
        x: 0.0,
        y: 0.0,
        width: measures::WIDTH,
        height: 0.0,
    };
    let grid = arrange(dummy, hidden, brightness.is_some()).height();
    Size {
        width: measures::WIDTH,
        height: total_height(grid, editing),
    }
}

/// Rejilla Tahoe de cuatro columnas sobre fondo invisible, repartida según
/// los ocultos: cada zona añade lo suyo si su control se ve. Sin editar se
/// suma la burbuja centrada bajo la rejilla; al editar, cada pieza visible
/// lleva su botón "−" y la burbuja desaparece.
pub fn control_center_panel(
    theme: &Theme,
    icons: &ControlCenterIcons,
    bounds: Rect,
    props: &ControlCenterProps,
    editing: bool,
) -> Overlay {
    let layout = arrange(bounds, props.hidden, props.brightness.is_some());
    let mut pieces = Pieces::new();
    add_capsules(&mut pieces, theme, &layout, props, icons);
    add_music(&mut pieces, theme, &layout, props, icons);
    add_circles(&mut pieces, theme, &layout, props, icons);
    add_brightness(&mut pieces, theme, &layout, props, icons);
    add_slider(&mut pieces, theme, &layout, props, icons);
    add_edit_chrome(
        &mut pieces,
        theme,
        &layout,
        bounds,
        layout.height(),
        editing,
    );
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
        cursors: Vec::new(),
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
