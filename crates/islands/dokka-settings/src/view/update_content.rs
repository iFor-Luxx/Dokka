//! Apartado "Actualización de Dokka": versión instalada y aviso.
//!
//! Una tarjeta con el icono, "Dokka" y la versión instalada (la del paquete,
//! que pasa el shell como dato), y el texto de que aún no hay de dónde
//! descargar versiones nuevas. Sin botones ni interruptores que no hagan
//! nada, y sin red. Cabe sin bajar con la rueda.

use std::sync::Arc;

use dokka_scene::{Drawing, Icon, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::update_top;
use super::super::state::Settings;
use super::about_cards::section_header;
use super::content::{ContentScene, group_pieces};
use super::general_rows::hidden_icon;

/// Ficha completa del apartado; `badge` y `glyph` son el icono de la fila.
pub(crate) fn update_content(
    theme: &Theme,
    settings: &Settings,
    badge: &Arc<Drawing>,
    glyph: &Arc<Drawing>,
) -> ContentScene {
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let window_height = settings.window_height();
    let top = update_top(scroll);
    let heights = vec![measures::CONTENT_ROW_HEIGHT];
    let (mut panel, mut dividers, mut texts) =
        group_pieces(theme, top, &["Dokka"], &heights, &[false], width);
    if top + measures::CONTENT_ROW_HEIGHT < measures::CONTENT_TOP || top > window_height {
        panel.bounds = dokka_scene::Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        };
        dividers.clear();
        texts.clear();
    }
    let version = settings
        .dokka_version()
        .map(|version| format!("Versión {version}"))
        .unwrap_or_else(|| "Versión desconocida".to_owned());
    let middle = top + measures::CONTENT_ROW_HEIGHT / 2.0;
    if middle >= measures::CONTENT_TOP && middle <= window_height {
        texts.push(Text {
            x: measures::CONTENT_LEFT + width - measures::CONTENT_PADDING,
            center_y: middle,
            content: version,
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Right,
        });
    }
    let mut flats = vec![panel];
    let middle = top + measures::CONTENT_ROW_HEIGHT / 2.0;
    let mut icons = Vec::new();
    if middle >= measures::CONTENT_TOP && middle <= window_height {
        let badge_x = measures::CONTENT_LEFT + measures::CONTENT_PADDING;
        let badge_y = top + (measures::CONTENT_ROW_HEIGHT - measures::BADGE_SIDE) / 2.0;
        let inset = (measures::BADGE_SIDE - measures::ROW_ICON_SIDE) / 2.0;
        icons.push(Icon {
            x: badge_x,
            y: badge_y,
            side: measures::BADGE_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(badge))),
            placeholder: theme.settings.badge[1],
            placeholder_radius: measures::BADGE_RADIUS,
        });
        icons.push(Icon {
            x: badge_x + inset,
            y: badge_y + inset,
            side: measures::ROW_ICON_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(glyph))),
            placeholder: theme.settings.badge[1],
            placeholder_radius: 0.0,
        });
    } else {
        icons.push(hidden_icon(theme));
        icons.push(hidden_icon(theme));
    }
    let hint_top = top + measures::CONTENT_ROW_HEIGHT + measures::GROUP_GAP;
    texts.extend(section_header(
        theme,
        hint_top,
        "Todavía no hay un lugar desde donde descargar versiones nuevas.",
        width,
        window_height,
    ));
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}
