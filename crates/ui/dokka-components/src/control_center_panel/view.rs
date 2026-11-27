use std::sync::Arc;

use dokka_model::NetworkKind;
use dokka_scene::{Color, Drawing, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{
    capsule_circle, capsule_text_x, music_art, music_button, music_tile, network_capsule,
    prefs_capsule, round_button, slider_track, sound_card, sound_left_icon, sound_mute_box,
};

/// Tamaño fijo del panel.
pub fn control_center_size() -> Size {
    Size {
        width: measures::WIDTH,
        height: measures::HEIGHT,
    }
}

/// Rejilla Tahoe sobre fondo invisible: dos cápsulas a la izquierda (Red con
/// su estado y Preferencias de Dokka), música cuadrada a la derecha, tres
/// círculos debajo y Sonido a todo lo ancho. Cada pieza de cristal va en
/// `tiles`; los círculos interiores, la carátula y las barras van en
/// `flat_panels`. Todo existe siempre: lo oculto mide cero o es vacío.
pub fn control_center_panel(
    theme: &Theme,
    icons: &ControlCenterIcons,
    bounds: Rect,
    props: &ControlCenterProps,
) -> Overlay {
    let network = network_capsule(bounds);
    let prefs = prefs_capsule(bounds);
    let music = music_tile(bounds);
    let sound = sound_card(bounds);
    let track = slider_track(sound);
    let level = props.volume.level.clamp(0.0, 1.0);
    let fill_width = if props.volume.is_muted {
        0.0
    } else {
        track.width * level
    };
    let connected = network_connected(props);

    Overlay {
        panel: Panel {
            bounds,
            corner_radius: measures::CORNER_RADIUS,
            border_width: 0.0,
            tint: theme.tray.clear,
            border: theme.tray.clear,
        },
        tiles: vec![
            tile(theme, network, network.height / 2.0),
            tile(theme, prefs, prefs.height / 2.0),
            tile(theme, music, glass::TILE_RADIUS),
            tile(
                theme,
                round_button(bounds, 0),
                measures::ROUND_DIAMETER / 2.0,
            ),
            tile(
                theme,
                round_button(bounds, 1),
                measures::ROUND_DIAMETER / 2.0,
            ),
            tile(
                theme,
                round_button(bounds, 2),
                measures::ROUND_DIAMETER / 2.0,
            ),
            tile(theme, sound, glass::TILE_RADIUS),
        ],
        pill: None,
        highlight: None,
        flat_panels: vec![
            flat(
                capsule_circle(network),
                glass::CIRCLE / 2.0,
                circle_tint(theme, connected),
            ),
            flat(
                capsule_circle(prefs),
                glass::CIRCLE / 2.0,
                theme.tray.circle,
            ),
            flat(music_art(music), measures::ART_RADIUS, theme.tray.circle),
            flat(track, glass::TRACK_HEIGHT / 2.0, theme.tray.track),
            flat(
                fill_rect(track, fill_width),
                glass::TRACK_HEIGHT / 2.0,
                theme.tray.fill,
            ),
        ],
        dividers: Vec::new(),
        dots: Vec::new(),
        pointer: None,
        texts: vec![
            title(
                theme,
                capsule_text_x(network),
                network.y + 23.0,
                network_name(props),
            ),
            subtitle(
                theme,
                capsule_text_x(network),
                network.y + 41.0,
                network_status(props),
            ),
            title(
                theme,
                capsule_text_x(prefs),
                prefs.y + 23.0,
                String::from("Preferencias"),
            ),
            subtitle(
                theme,
                capsule_text_x(prefs),
                prefs.y + 41.0,
                String::from("Dokka"),
            ),
            title(
                theme,
                music.x + glass::PADDING,
                music.y + 60.0,
                music_title(props),
            ),
            subtitle(
                theme,
                music.x + glass::PADDING,
                music.y + 74.0,
                music_artist(props),
            ),
            title(
                theme,
                sound.x + glass::PADDING,
                sound.y + 22.0,
                String::from("Sonido"),
            ),
        ],
        icons: vec![
            picture(
                theme,
                icons.network_glyph(&props.network, connected),
                capsule_circle(network),
                glass::CIRCLE_ICON,
            ),
            picture(
                theme,
                &icons.preferences,
                capsule_circle(prefs),
                glass::CIRCLE_ICON,
            ),
            picture(
                theme,
                &icons.previous,
                music_button(music, 0),
                measures::MUSIC_ICON_SIDE,
            ),
            picture(
                theme,
                icons.toggle(props.now_playing.is_playing),
                music_button(music, 1),
                measures::MUSIC_ICON_SIDE,
            ),
            picture(
                theme,
                &icons.next,
                music_button(music, 2),
                measures::MUSIC_ICON_SIDE,
            ),
            picture(
                theme,
                &icons.desktop,
                round_button(bounds, 0),
                glass::CIRCLE_ICON,
            ),
            picture(
                theme,
                &icons.lock,
                round_button(bounds, 1),
                glass::CIRCLE_ICON,
            ),
            picture(
                theme,
                &icons.task_manager,
                round_button(bounds, 2),
                glass::CIRCLE_ICON,
            ),
            picture(
                theme,
                icons.volume(false),
                sound_left_icon(sound),
                measures::END_ICON_SIDE,
            ),
            picture(
                theme,
                icons.volume(props.volume.is_muted),
                sound_mute_box(sound),
                measures::END_ICON_SIDE,
            ),
        ],
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Pieza de cristal separada con su borde fino y luminoso.
fn tile(theme: &Theme, bounds: Rect, radius: f32) -> Panel {
    Panel {
        bounds,
        corner_radius: clip(radius, bounds),
        border_width: glass::TILE_BORDER_WIDTH,
        tint: theme.tray.tile,
        border: theme.tray.tile_border,
    }
}

/// Forma plana sin borde: círculo interior, carátula o barra.
fn flat(bounds: Rect, radius: f32, color: Color) -> Panel {
    Panel {
        bounds,
        corner_radius: clip(radius, bounds),
        border_width: 0.0,
        tint: color,
        border: color,
    }
}

/// Recorta el radio a la mitad del lado menor; nunca negativo.
fn clip(radius: f32, bounds: Rect) -> f32 {
    radius
        .min(bounds.width / 2.0)
        .min(bounds.height / 2.0)
        .max(0.0)
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

/// Tinte del círculo de la cápsula: blanco con icono azul si hay red.
fn circle_tint(theme: &Theme, connected: bool) -> Color {
    if connected {
        theme.tray.circle_active
    } else {
        theme.tray.circle
    }
}

/// Icono blanco centrado en su zona, con el dibujo ya construido.
fn picture(theme: &Theme, drawing: &Arc<Drawing>, area: Rect, side: f32) -> Icon {
    Icon {
        x: area.x + (area.width - side) / 2.0,
        y: area.y + (area.height - side) / 2.0,
        side,
        image: Some(IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.tray.circle_icon,
        placeholder_radius: 0.0,
    }
}

fn title(theme: &Theme, x: f32, center_y: f32, content: String) -> Text {
    Text {
        x,
        center_y,
        content,
        size: glass::TITLE_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

fn subtitle(theme: &Theme, x: f32, center_y: f32, content: String) -> Text {
    Text {
        x,
        center_y,
        content,
        size: glass::SUBTITLE_SIZE,
        is_bold: false,
        color: theme.tray.secondary,
        align: TextAlign::Left,
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

fn shorten(content: &str) -> String {
    if content.chars().count() <= measures::TEXT_CHARACTERS {
        return content.to_owned();
    }
    let kept: String = content
        .chars()
        .take(measures::TEXT_CHARACTERS - 1)
        .collect();
    format!("{}…", kept.trim_end())
}
