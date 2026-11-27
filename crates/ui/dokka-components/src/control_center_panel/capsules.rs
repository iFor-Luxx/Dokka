//! Cápsulas del Centro de control: Red en la fila 1 y Preferencias en la 3.
//! Estadio totalmente redondo con círculo interior, título y subtítulo.

use dokka_model::NetworkKind;
use dokka_scene::{Color, Rect};
use dokka_theme::{Theme, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{capsule_circle, capsule_text_x, network_capsule, prefs_capsule};
use super::paint::{Pieces, flat, picture, shorten, subtitle, tile, title};

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
    pieces.tiles.push(tile(theme, network, network.height / 2.0));
    pieces.tiles.push(tile(theme, prefs, prefs.height / 2.0));
    pieces.flats.push(flat(
        capsule_circle(network),
        glass::CIRCLE / 2.0,
        circle_tint(theme, connected),
    ));
    pieces.flats.push(flat(
        capsule_circle(prefs),
        glass::CIRCLE / 2.0,
        theme.tray.circle,
    ));
    pieces.texts.push(title(
        theme,
        capsule_text_x(network),
        network.y + 23.0,
        network_name(props),
    ));
    pieces.texts.push(subtitle(
        theme,
        capsule_text_x(network),
        network.y + 41.0,
        network_status(props),
    ));
    pieces.texts.push(title(
        theme,
        capsule_text_x(prefs),
        prefs.y + 23.0,
        String::from("Preferencias"),
    ));
    pieces.texts.push(subtitle(
        theme,
        capsule_text_x(prefs),
        prefs.y + 41.0,
        String::from("Dokka"),
    ));
    pieces.icons.push(picture(
        theme,
        icons.network_glyph(&props.network, connected),
        capsule_circle(network),
        glass::CIRCLE_ICON,
    ));
    pieces.icons.push(picture(
        theme,
        &icons.preferences,
        capsule_circle(prefs),
        glass::CIRCLE_ICON,
    ));
}

/// Tinte del círculo de la cápsula: blanco con icono azul si hay red.
fn circle_tint(theme: &Theme, connected: bool) -> Color {
    if connected {
        theme.tray.circle_active
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
        shorten(&props.network.name)
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
