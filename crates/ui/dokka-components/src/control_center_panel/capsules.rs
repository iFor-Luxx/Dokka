//! Cápsulas del Centro de control: Red en la fila 1 y Preferencias en la 3.
//! Estadio totalmente redondo con círculo interior, título y subtítulo.

use dokka_model::NetworkKind;
use dokka_scene::Rect;
use dokka_theme::{Theme, control_center as measures};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{capsule_circle, capsule_text_x, network_capsule, prefs_capsule};
use super::paint::{Pieces, flat, picture, shorten_to, subtitle_sized, tile, title_sized};

/// Suma las dos cápsulas: cristal, círculo interior, textos e iconos.
pub(crate) fn add_capsules(
    pieces: &mut Pieces,
    theme: &Theme,
    bounds: Rect,
    props: &ControlCenterProps,
    icons: &ControlCenterIcons,
) {
    let network = network_capsule(bounds);
    let prefs = prefs_capsule(bounds);
    let connected = network_connected(props);
    pieces
        .tiles
        .push(tile(theme, network, network.height / 2.0));
    pieces.tiles.push(tile(theme, prefs, prefs.height / 2.0));
    pieces.flats.push(flat(
        capsule_circle(network),
        measures::CAPSULE_CIRCLE_SIDE / 2.0,
        circle_tint(theme, connected),
    ));
    pieces.flats.push(flat(
        capsule_circle(prefs),
        measures::CAPSULE_CIRCLE_SIDE / 2.0,
        theme.tray.circle,
    ));
    let network_middle = network.y + network.height / 2.0;
    let prefs_middle = prefs.y + prefs.height / 2.0;
    pieces.texts.push(title_sized(
        theme,
        capsule_text_x(network),
        network_middle - measures::CAPSULE_TEXT_HALF,
        shorten_to(&network_name(props), measures::CAPSULE_TEXT_CHARACTERS),
        measures::CAPSULE_TITLE_SIZE,
    ));
    pieces.texts.push(subtitle_sized(
        theme,
        capsule_text_x(network),
        network_middle + measures::CAPSULE_TEXT_HALF,
        shorten_to(&network_status(props), measures::CAPSULE_TEXT_CHARACTERS),
        measures::CAPSULE_SUBTITLE_SIZE,
    ));
    pieces.texts.push(title_sized(
        theme,
        capsule_text_x(prefs),
        prefs_middle - measures::CAPSULE_TEXT_HALF,
        shorten_to("Preferencias", measures::CAPSULE_TEXT_CHARACTERS),
        measures::CAPSULE_TITLE_SIZE,
    ));
    pieces.texts.push(subtitle_sized(
        theme,
        capsule_text_x(prefs),
        prefs_middle + measures::CAPSULE_TEXT_HALF,
        shorten_to("Dokka", measures::CAPSULE_TEXT_CHARACTERS),
        measures::CAPSULE_SUBTITLE_SIZE,
    ));
    pieces.icons.push(picture(
        theme,
        icons.network_glyph(&props.network, connected),
        capsule_circle(network),
        measures::CIRCLE_ICON_SIDE,
    ));
    pieces.icons.push(picture(
        theme,
        &icons.preferences,
        capsule_circle(prefs),
        measures::CIRCLE_ICON_SIDE,
    ));
}

/// Tinte del círculo de la cápsula: blanco si hay red, cristal apagado si no.
fn circle_tint(theme: &Theme, connected: bool) -> dokka_scene::Color {
    if connected {
        theme.tray.fill
    } else {
        theme.tray.circle
    }
}

fn network_connected(props: &ControlCenterProps) -> bool {
    props.network.has_internet && !matches!(props.network.kind, NetworkKind::Offline)
}

fn network_name(props: &ControlCenterProps) -> String {
    if props.network.name.is_empty() {
        String::from("Sin conexión")
    } else {
        props.network.name.clone()
    }
}

fn network_status(props: &ControlCenterProps) -> String {
    match (props.network.kind, props.network.has_internet) {
        (NetworkKind::Offline, _) => String::from("Desconectada"),
        (_, false) => String::from("Sin internet"),
        (NetworkKind::Ethernet, true) => String::from("Ethernet"),
        (NetworkKind::Wifi, true) => String::from("Wi-Fi"),
        (NetworkKind::Other, true) => String::from("Conectada"),
    }
}
