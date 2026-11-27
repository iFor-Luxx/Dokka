//! Tarjeta ancha de Sonido: título arriba a la izquierda, pista fina con
//! relleno blanco sin bolita e icono pequeño a cada extremo (el derecho
//! silencia).

use dokka_scene::Rect;
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{slider_track, sound_card, sound_left_icon, sound_mute_box};
use super::paint::{Pieces, flat, picture, tile, title};

/// Suma el Sonido: cristal, pista y relleno, título y los dos iconos.
pub(crate) fn add_slider(
    pieces: &mut Pieces,
    theme: &Theme,
    bounds: Rect,
    props: &ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let card = sound_card(bounds);
    let track = slider_track(card);
    let level = props.volume.level.clamp(0.0, 1.0);
    let fill_width = if props.volume.is_muted {
        0.0
    } else {
        track.width * level
    };
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
        card.x + glass::PADDING,
        card.y + 22.0,
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
        sound_mute_box(card),
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
