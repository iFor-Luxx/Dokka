//! Piezas fijas de la galería: casilla de búsqueda, líneas y pie.
//!
//! Sin lógica: la casilla siempre lleva el borde azul de foco (el teclado de
//! la barra escribe aquí mientras la galería está abierta) y el pie trae el
//! aviso con el botón "Aceptar".

use std::sync::Arc;

use dokka_scene::{Divider, Drawing, Icon, IconImage, Panel, Rect, Text, TextAlign, TextCursor};
use dokka_theme::{Theme, control_gallery as measures};

use super::layout::{accept_button, footer_line};
use crate::search_field::text_cursor;

/// Casilla redondeada con borde azul de foco.
pub(crate) fn search_tile(theme: &Theme, search: Rect) -> Panel {
    Panel {
        bounds: search,
        corner_radius: capped(search, measures::SEARCH_RADIUS),
        border_width: measures::BORDER_WIDTH,
        tint: theme.settings.search_background,
        border: theme.settings.switch_on,
    }
}

/// Lupa a la izquierda de la casilla, reutilizando la del buscador.
pub(crate) fn search_icon(theme: &Theme, magnifier: &Arc<Drawing>, search: Rect) -> Icon {
    Icon {
        x: search.x + measures::SEARCH_ICON_INSET,
        y: search.y + (search.height - measures::SEARCH_ICON_SIDE) / 2.0,
        side: measures::SEARCH_ICON_SIDE,
        image: Some(IconImage::Drawing(Arc::clone(magnifier))),
        placeholder: theme.settings.search_icon,
        placeholder_radius: 0.0,
    }
}

/// Lo escrito o el marcador "Buscar controles".
pub(crate) fn search_text(theme: &Theme, search: Rect, query: &str) -> Text {
    let x = search.x
        + measures::SEARCH_ICON_INSET
        + measures::SEARCH_ICON_SIDE
        + measures::SEARCH_TEXT_GAP;
    let middle = search.y + search.height / 2.0;
    if query.is_empty() {
        return Text {
            x,
            center_y: middle,
            content: measures::SEARCH_PLACEHOLDER.to_owned(),
            size: measures::SEARCH_TEXT_SIZE,
            is_bold: false,
            color: theme.tray.secondary,
            align: TextAlign::Left,
        };
    }
    Text {
        x,
        center_y: middle,
        content: query.to_owned(),
        size: measures::SEARCH_TEXT_SIZE,
        is_bold: false,
        color: theme.settings.search_text,
        align: TextAlign::Left,
    }
}

/// Cursor tras lo escrito: la plataforma mide el texto y lo coloca.
pub(crate) fn search_cursor(theme: &Theme, search: Rect, query: &str, visible: bool) -> TextCursor {
    let left = search.x
        + measures::SEARCH_ICON_INSET
        + measures::SEARCH_ICON_SIDE
        + measures::SEARCH_TEXT_GAP;
    text_cursor(TextCursor {
        text_x: left,
        center_y: search.y + search.height / 2.0,
        content: query.to_owned(),
        size: measures::SEARCH_TEXT_SIZE,
        is_bold: false,
        max_x: search.x + search.width - measures::SEARCH_ICON_INSET,
        gap: measures::SEARCH_CURSOR_GAP,
        width: measures::SEARCH_CURSOR_WIDTH,
        height: measures::SEARCH_CURSOR_HEIGHT,
        color: theme.settings.search_text,
        visible,
    })
}

/// Línea vertical entre la columna y las secciones.
pub(crate) fn column_divider(theme: &Theme, gallery: Rect) -> Divider {
    Divider {
        bounds: Rect {
            x: gallery.x + measures::LEFT_WIDTH,
            y: gallery.y + measures::PADDING,
            width: measures::DIVIDER_HEIGHT,
            height: gallery.height - measures::FOOTER_HEIGHT - measures::PADDING * 2.0,
        },
        color: theme.tray.separator,
    }
}

/// Línea sobre el pie fijo.
pub(crate) fn footer_divider(theme: &Theme, gallery: Rect) -> Divider {
    Divider {
        bounds: footer_line(gallery),
        color: theme.tray.separator,
    }
}

/// Aviso centrado del pie.
pub(crate) fn hint_text(theme: &Theme, gallery: Rect) -> Text {
    Text {
        x: gallery.x + gallery.width / 2.0,
        center_y: gallery.y + gallery.height - measures::FOOTER_HEIGHT / 2.0,
        content: String::from("Haz clic en un control para añadirlo al centro de control."),
        size: measures::HINT_TEXT_SIZE,
        is_bold: false,
        color: theme.tray.secondary,
        align: TextAlign::Center,
    }
}

/// Botón azul del pie.
pub(crate) fn accept_tile(theme: &Theme, gallery: Rect) -> Panel {
    let button = accept_button(gallery);
    Panel {
        bounds: button,
        corner_radius: capped(button, measures::ACCEPT_RADIUS),
        border_width: 0.0,
        tint: theme.settings.switch_on,
        border: theme.settings.switch_on,
    }
}

/// Etiqueta blanca del botón.
pub(crate) fn accept_label(theme: &Theme, gallery: Rect) -> Text {
    let button = accept_button(gallery);
    Text {
        x: button.x + button.width / 2.0,
        center_y: button.y + button.height / 2.0,
        content: String::from("Aceptar"),
        size: measures::ACCEPT_TEXT_SIZE,
        is_bold: true,
        color: theme.menu.highlighted_text,
        align: TextAlign::Center,
    }
}

/// Radio recortado a la mitad del lado menor; nunca negativo.
pub(crate) fn capped(tile: Rect, maximum: f32) -> f32 {
    if tile.width <= 0.0 || tile.height <= 0.0 {
        return 0.0;
    }
    maximum
        .min(tile.width / 2.0)
        .min(tile.height / 2.0)
        .max(0.0)
}
