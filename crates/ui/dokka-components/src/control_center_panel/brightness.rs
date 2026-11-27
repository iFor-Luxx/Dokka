//! Tarjeta ancha de Pantalla: justo encima de Sonido y con su misma forma,
//! márgenes y geometría. Título arriba a la izquierda, pista fina con relleno
//! blanco en la parte baja y un sol pequeño a cada extremo (tenue a la
//! izquierda, pleno a la derecha), alineados con la barra. Solo existe con
//! brillo DDC disponible.

use dokka_scene::Rect;
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{brightness_card, slider_track, sound_left_icon, sound_mute_icon};
use super::paint::{Pieces, flat, picture, tile, title};

/// Suma la Pantalla si hay brillo: cristal, pista y relleno, título y soles.
/// Sin brillo no añade nada y el panel queda como antes.
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
    let track = slider_track(card);
    let fill_width = track.width * level.clamp(0.0, 1.0);
    pieces.tiles.push(tile(theme, card, glass::TILE_RADIUS));
    pieces
        .flats
        .push(flat(track, glass::TRACK_HEIGHT / 2.0, theme.tray.track));
    pieces.flats.push(flat(
        fill_rect(track, fill_width),
        glass::TRACK_HEIGHT / 2.0,
        theme.tray.fill,
    ));
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

/// Tramo blanco de la pista, desde la izquierda hasta el nivel.
fn fill_rect(track: Rect, width: f32) -> Rect {
    Rect {
        x: track.x,
        y: track.y,
        width: width.clamp(0.0, track.width),
        height: track.height,
    }
}
