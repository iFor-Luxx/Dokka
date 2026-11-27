//! Tarjeta de música: dos columnas por dos filas con carátula, título,
//! artista y controles grandes de anterior, pausa y siguiente.

use dokka_scene::Rect;
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{music_art, music_button, music_tile};
use super::paint::{Pieces, flat, picture, shorten, subtitle, tile, title};

/// Suma la música: cristal, carátula, textos y los tres controles.
pub(crate) fn add_music(
    pieces: &mut Pieces,
    theme: &Theme,
    bounds: Rect,
    props: &ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let card = music_tile(bounds);
    pieces.tiles.push(tile(theme, card, glass::TILE_RADIUS));
    pieces.flats.push(flat(
        music_art(card),
        measures::ART_RADIUS,
        theme.tray.circle,
    ));
    pieces.texts.push(title(
        theme,
        card.x + glass::PADDING,
        card.y + 62.0,
        music_title(props),
    ));
    pieces.texts.push(subtitle(
        theme,
        card.x + glass::PADDING,
        card.y + 76.0,
        music_artist(props),
    ));
    pieces.icons.push(picture(
        theme,
        &icons.previous,
        music_button(card, 0),
        measures::MUSIC_ICON_SIDE,
    ));
    pieces.icons.push(picture(
        theme,
        icons.toggle(props.now_playing.is_playing),
        music_button(card, 1),
        measures::MUSIC_ICON_SIDE,
    ));
    pieces.icons.push(picture(
        theme,
        &icons.next,
        music_button(card, 2),
        measures::MUSIC_ICON_SIDE,
    ));
}

fn music_title(props: &ControlCenterProps) -> String {
    if props.now_playing.title.is_empty() {
        String::from("Nada en reproducción")
    } else {
        shorten(&props.now_playing.title)
    }
}

fn music_artist(props: &ControlCenterProps) -> String {
    shorten(&props.now_playing.artist)
}
