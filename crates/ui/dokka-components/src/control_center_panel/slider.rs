//! Tarjeta ancha de Sonido donde la ponga la rejilla: título arriba a la
//! izquierda, pista fina del componente `slider` con relleno blanco y
//! tirador en cápsula en la parte baja e icono pequeño a cada extremo (el
//! derecho silencia), alineados con la barra. Oculta no se dibuja.

use dokka_model::ControlItem;
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{ControlLayout, slider_area, sound_left_icon, sound_mute_icon};
use super::paint::{Pieces, picture, tile, title};
use crate::slider::{SliderColors, SliderProps, slider_panels, slider_track_bounds};

/// Suma el Sonido si se ve: cristal, pista con relleno y tirador, título y
/// los dos iconos. Sin silencio el relleno es blanco; con él, atenuado.
pub(crate) fn add_slider(
    pieces: &mut Pieces,
    theme: &Theme,
    layout: &ControlLayout,
    props: &ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let Some(card) = layout.rect(ControlItem::Sound) else {
        return;
    };
    let area = slider_area(card);
    let track = slider_track_bounds(area);
    let slider = SliderProps {
        value: props.volume.level,
        neutral: None,
        active: !props.volume.is_muted,
    };
    pieces.tiles.push(tile(theme, card, glass::TILE_RADIUS));
    pieces
        .flats
        .extend(slider_panels(track, &slider, &SliderColors::glass(theme)));
    pieces.texts.push(title(
        theme,
        card.x + measures::PADDING,
        card.y + measures::SLIDER_TITLE_TOP,
        String::from("Sonido"),
    ));
    pieces.icons.push(picture(
        theme,
        icons.volume(false),
        sound_left_icon(card),
        measures::END_ICON_SIDE,
    ));
    pieces.icons.push(picture(
        theme,
        icons.volume(props.volume.is_muted),
        sound_mute_icon(card),
        measures::END_ICON_SIDE,
    ));
}
