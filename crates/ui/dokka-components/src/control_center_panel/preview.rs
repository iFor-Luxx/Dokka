//! Vista previa de un control aislado para la galería.
//!
//! La galería dibuja cada control IGUAL que el centro y al mismo tamaño:
//! coloca un reparto con solo ese control en el origen pedido y llama a la
//! misma zona (`capsules`, `music`, `circles`, `slider`, `brightness`) que el
//! panel. Sin brillo DDC no hay tarjeta de Pantalla.

use dokka_model::{ControlItem, HiddenControls};
use dokka_scene::{Icon, Panel, Rect, Size, Text};
use dokka_theme::{Theme, control_center as measures};

use super::ControlCenterProps;
use super::brightness::add_brightness;
use super::capsules::add_capsules;
use super::circles::add_circles;
use super::icons::ControlCenterIcons;
use super::layout::{arrange, cell_span, slider_height};
use super::music::add_music;
use super::paint::Pieces;
use super::slider::add_slider;

/// Tamaño del control aislado, igual que su pieza en el centro.
pub(crate) fn preview_size(item: ControlItem) -> Size {
    if let Some(height) = slider_height(item) {
        let width = 4.0 * measures::CELL + 3.0 * measures::GAP;
        return Size { width, height };
    }
    let (span_w, span_h) = cell_span(item);
    Size {
        width: span_w as f32 * measures::CELL + (span_w as f32 - 1.0) * measures::GAP,
        height: span_h as f32 * measures::CELL + (span_h as f32 - 1.0) * measures::GAP,
    }
}

/// Piezas del control en el origen dado, con los mismos datos del centro
/// (red, volumen, música y brillo). Sin DDC no dibuja la Pantalla.
pub(crate) fn preview_panels(
    theme: &Theme,
    icons: &ControlCenterIcons,
    item: ControlItem,
    props: &ControlCenterProps,
    origin: (f32, f32),
) -> (Vec<Panel>, Vec<Panel>, Vec<Text>, Vec<Icon>) {
    if item == ControlItem::Brightness && props.brightness.is_none() {
        return (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    }
    let mut hidden = HiddenControls::empty();
    for other in ControlItem::FACTORY_ORDER {
        if other != item {
            hidden.insert(other);
        }
    }
    let dummy = Rect {
        x: origin.0 - measures::PADDING,
        y: origin.1 - measures::PADDING,
        width: measures::WIDTH,
        height: 0.0,
    };
    let layout = arrange(dummy, hidden, props.brightness.is_some());
    if layout.rect(item).is_none() {
        return (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    }
    let mut pieces = Pieces::new();
    match item {
        ControlItem::Network | ControlItem::Settings => {
            add_capsules(&mut pieces, theme, &layout, props, icons);
        }
        ControlItem::Music => add_music(&mut pieces, theme, &layout, props, icons),
        ControlItem::Brightness => add_brightness(&mut pieces, theme, &layout, props, icons),
        ControlItem::Sound => add_slider(&mut pieces, theme, &layout, props, icons),
        _ => add_circles(&mut pieces, theme, &layout, props, icons),
    }
    (pieces.tiles, pieces.flats, pieces.texts, pieces.icons)
}
