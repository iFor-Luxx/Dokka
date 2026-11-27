//! Tarjeta ancha de Pantalla: justo encima de Sonido y con su misma forma,
//! márgenes y geometría. Título arriba a la izquierda, pista fina del
//! componente `slider` con relleno blanco y tirador en cápsula en la parte
//! baja y un sol pequeño a cada extremo (tenue a la izquierda, pleno a la
//! derecha), alineados con la barra. Solo existe con brillo DDC disponible.

use dokka_scene::Rect;
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{brightness_card, slider_area, sound_left_icon, sound_mute_icon};
use super::paint::{Pieces, picture, tile, title};
use crate::slider::{SliderColors, SliderProps, slider_panels, slider_track_bounds};

/// Suma la Pantalla si hay brillo: cristal, pista con relleno y tirador,
/// título y soles. Sin brillo no añade nada y el panel queda como antes.
pub(crate) fn add_brightness(
    pieces: &mut Pieces,
    theme: &Theme,
    bounds: Rect,
    props: &ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let Some(level) = props.brightness else {
        return;
    };
    let card = brightness_card(bounds);
    let area = slider_area(card);
    let track = slider_track_bounds(area);
    let slider = SliderProps {
        value: level,
        neutral: None,
        active: true,
    };
    pieces.tiles.push(tile(theme, card, glass::TILE_RADIUS));
    pieces
        .flats
        .extend(slider_panels(track, &slider, &SliderColors::glass(theme)));
    pieces.texts.push(title(
        theme,
        card.x + measures::PADDING,
        card.y + measures::SLIDER_TITLE_TOP,
        String::from("Pantalla"),
    ));
    pieces.icons.push(picture(
        theme,
        &icons.sun_dim,
        sound_left_icon(card),
        measures::END_ICON_SIDE,
    ));
    pieces.icons.push(picture(
        theme,
        &icons.sun,
        sound_mute_icon(card),
        measures::END_ICON_SIDE,
    ));
}
