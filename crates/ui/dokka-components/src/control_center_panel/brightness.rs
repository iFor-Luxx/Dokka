//! Tarjeta ancha de Pantalla donde la ponga la rejilla: título arriba a la
//! izquierda, pista fina del componente `slider` con relleno blanco y
//! tirador en cápsula en la parte baja y un sol pequeño a cada extremo
//! (tenue a la izquierda, pleno a la derecha), alineados con la barra.
//! Solo existe visible y con brillo DDC.

use dokka_model::ControlItem;
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{ControlLayout, slider_area, sound_left_icon, sound_mute_icon};
use super::paint::{Pieces, picture, tile, title};
use crate::slider::{SliderColors, SliderProps, slider_panels, slider_track_bounds};

/// Suma la Pantalla si se ve y hay brillo; si no, no añade nada.
pub(crate) fn add_brightness(
    pieces: &mut Pieces,
    theme: &Theme,
    layout: &ControlLayout,
    props: &ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let Some(level) = props.brightness else {
        return;
    };
    let Some(card) = layout.rect(ControlItem::Brightness) else {
        return;
    };
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
