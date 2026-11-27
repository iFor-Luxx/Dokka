//! Tarjeta de música: dos columnas por dos filas con carátula, título,
//! artista y controles grandes de anterior, pausa y siguiente.
//! La carátula es un icono con píxeles cuando hay foto; sin ella, un
//! marcador transparente sobre el plano de detrás. Siempre hay icono,
//! con o sin imagen: la cantidad de elementos no cambia.

use std::sync::Arc;

use dokka_scene::{Bitmap, Icon, IconImage, Rect};
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
    let art = music_art(card);
    pieces.tiles.push(tile(theme, card, glass::TILE_RADIUS));
    pieces
        .flats
        .push(flat(art, measures::ART_RADIUS, theme.tray.circle));
    pieces
        .icons
        .push(artwork_icon(theme, art, props.artwork.as_ref()));
    pieces.texts.push(title(
        theme,
        card.x + measures::PADDING,
        card.y + measures::MUSIC_TITLE_TOP,
        music_title(props),
    ));
    pieces.texts.push(subtitle(
        theme,
        card.x + measures::PADDING,
        card.y + measures::MUSIC_SUBTITLE_TOP,
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

/// La foto en el sitio del cuadro; sin foto, un marcador transparente
/// sobre el plano redondeado de detrás. Siempre hay icono, con o sin
/// imagen: la plataforma solo reconstruye la capa al pasar de una a otra.
fn artwork_icon(theme: &Theme, art: Rect, artwork: Option<&Arc<Bitmap>>) -> Icon {
    Icon {
        x: art.x,
        y: art.y,
        side: art.width,
        image: artwork.map(|bitmap| IconImage::Pixels(Arc::clone(bitmap))),
        placeholder: theme.tray.clear,
        placeholder_radius: 0.0,
    }
}
