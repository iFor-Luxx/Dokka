use std::sync::Arc;

use dokka_scene::{Dot, Drawing, Icon, IconImage, Overlay, Panel, Rect, Size, Text, TextAlign};
use dokka_theme::{Theme, control_center as measures, typography};

use super::ControlCenterProps;
use super::icons::ControlCenterIcons;
use super::layout::{
    card_icon, music_button, music_card, mute_area, network_card, round_button, slider_track,
    volume_card,
};
use crate::popup_panel::popup_panel;

/// Tamaño fijo del panel.
pub fn control_center_size() -> Size {
    Size {
        width: measures::WIDTH,
        height: measures::HEIGHT,
    }
}

/// Panel de cristal con tres tarjetas planas (red, volumen y música) y la
/// fila de botones redondos. Las cantidades de planos, textos, puntos e
/// iconos no cambian entre estados: lo oculto mide cero o cadena vacía.
pub fn control_center_panel(
    theme: &Theme,
    icons: &ControlCenterIcons,
    bounds: Rect,
    props: &ControlCenterProps,
) -> Overlay {
    let network = network_card(bounds);
    let volume = volume_card(bounds);
    let music = music_card(bounds);
    let track = slider_track(volume);
    let level = props.volume.level.clamp(0.0, 1.0);
    let effective = if props.volume.is_muted { 0.0 } else { level };

    let flat_panels = vec![
        card_panel(theme, network),
        card_panel(theme, volume),
        card_panel(theme, music),
        Panel {
            bounds: track,
            corner_radius: measures::TRACK_RADIUS,
            border_width: 0.0,
            tint: theme.panel.border,
            border: theme.panel.border,
        },
        Panel {
            bounds: Rect {
                x: track.x,
                y: track.y,
                width: track.width * effective,
                height: track.height,
            },
            corner_radius: measures::TRACK_RADIUS,
            border_width: 0.0,
            tint: theme.panel.text,
            border: theme.panel.text,
        },
        card_panel(theme, round_button(bounds, 0)),
        card_panel(theme, round_button(bounds, 1)),
        card_panel(theme, round_button(bounds, 2)),
        card_panel(theme, round_button(bounds, 3)),
    ];

    let thumb = Dot {
        x: track.x + track.width * effective - measures::THUMB_DIAMETER / 2.0,
        y: track.y + track.height / 2.0 - measures::THUMB_DIAMETER / 2.0,
        diameter: measures::THUMB_DIAMETER,
        color: theme.panel.text,
    };

    let texts = vec![
        left_text(theme, network_text_x(network), network.y + 24.0, network_name(props), true),
        left_text(
            theme,
            network_text_x(network),
            network.y + 44.0,
            network_status(props),
            false,
        ),
        left_text(
            theme,
            network_text_x(network),
            mute_area(volume).y + measures::MUTE_HIT_SIDE / 2.0,
            volume_text(props),
            false,
        ),
        left_text(theme, music.x + measures::TEXT_INSET, music.y + 20.0, music_title(props), true),
        left_text(
            theme,
            music.x + measures::TEXT_INSET,
            music.y + 40.0,
            music_artist(props),
            false,
        ),
    ];

    let network_icon = card_icon(network);
    let mute_icon = card_icon(volume);
    let icon = |drawing: &Arc<Drawing>, area: Rect| Icon {
        x: area.x + (area.width - measures::ICON_SIDE) / 2.0,
        y: area.y + (area.height - measures::ICON_SIDE) / 2.0,
        side: measures::ICON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.panel.highlight,
        placeholder_radius: 0.0,
    };
    let icons = vec![
        icon(icons.network(&props.network), network_icon),
        icon(icons.volume(props.volume.is_muted), mute_icon),
        icon(&icons.previous, music_button(music, 0)),
        icon(icons.toggle(props.now_playing.is_playing), music_button(music, 1)),
        icon(&icons.next, music_button(music, 2)),
        icon(&icons.preferences, round_button(bounds, 0)),
        icon(&icons.desktop, round_button(bounds, 1)),
        icon(&icons.lock, round_button(bounds, 2)),
        icon(&icons.task_manager, round_button(bounds, 3)),
    ];

    Overlay {
        panel: popup_panel(theme, bounds),
        pill: None,
        highlight: None,
        flat_panels,
        dividers: Vec::new(),
        dots: vec![thumb],
        pointer: None,
        texts,
        icons,
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Tarjeta plana sobre el cristal: tinte claro y borde luminoso.
fn card_panel(theme: &Theme, bounds: Rect) -> Panel {
    Panel {
        bounds,
        corner_radius: measures::CARD_RADIUS,
        border_width: measures::CARD_BORDER_WIDTH,
        tint: theme.panel.highlight,
        border: theme.panel.border,
    }
}

fn left_text(theme: &Theme, x: f32, center_y: f32, content: String, is_bold: bool) -> Text {
    Text {
        x,
        center_y,
        content,
        size: typography::BODY_SIZE,
        is_bold,
        color: theme.panel.text,
        align: TextAlign::Left,
    }
}

fn network_text_x(card: Rect) -> f32 {
    card.x + measures::TEXT_INSET + measures::ICON_SIDE + measures::ICON_TEXT_GAP
}

fn network_name(props: &ControlCenterProps) -> String {
    if props.network.name.is_empty() {
        String::from("Sin conexión")
    } else {
        shorten(&props.network.name)
    }
}

fn network_status(props: &ControlCenterProps) -> String {
    use dokka_model::NetworkKind;
    match (props.network.kind, props.network.has_internet) {
        (NetworkKind::Offline, _) => String::from("Desconectada"),
        (_, false) => String::from("Sin internet"),
        (NetworkKind::Ethernet, true) => String::from("Ethernet"),
        (NetworkKind::Wifi, true) => String::from("Wi-Fi"),
        (NetworkKind::Other, true) => String::from("Conectada"),
    }
}

fn volume_text(props: &ControlCenterProps) -> String {
    if props.volume.is_muted {
        String::from("Volumen · Silenciado")
    } else {
        let percent = (props.volume.level.clamp(0.0, 1.0) * 100.0).round() as i32;
        format!("Volumen · {percent}%")
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
    let kept: String = content.chars().take(measures::TEXT_CHARACTERS - 1).collect();
    format!("{}…", kept.trim_end())
}
