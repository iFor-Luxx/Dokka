//! Geometría de "Idioma y región": idiomas, ejemplo, formatos y botón.
//!
//! La tarjeta de idiomas crece con la lista del usuario; si no cabe, la
//! página baja con la rueda y lo que queda fuera no se pinta. El golpe usa
//! la misma geometría que el dibujo para no dejar piezas a medias.

use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::content::{content_row, content_segmented};

/// Filas de la tarjeta de formatos, en orden de arriba abajo.
pub(crate) fn language_format_rows() -> [&'static str; 6] {
    [
        "Región",
        "Calendario",
        "Temperatura",
        "Sistema de medida",
        "Formato de fecha",
        "Formato de número",
    ]
}

/// Borde superior de la página con el desplazamiento aplicado.
pub(crate) fn language_top(scroll: f32) -> f32 {
    measures::CONTENT_TOP - scroll
}

/// Tarjeta de idiomas: filas `count` (o una que dice que no hay).
fn languages_height(count: usize) -> f32 {
    count.max(1) as f32 * measures::CONTENT_ROW_HEIGHT
}

/// Borde superior de la tarjeta `card`: 0 idiomas, 1 formatos, 2 semana.
pub(crate) fn language_card_top(card: usize, count: usize, scroll: f32) -> f32 {
    let top = language_top(scroll);
    match card {
        0 => top,
        1 => {
            top + languages_height(count)
                + measures::GROUP_GAP
                + example_height()
                + measures::GROUP_GAP
        }
        _ => {
            top + languages_height(count)
                + measures::GROUP_GAP
                + example_height()
                + measures::GROUP_GAP
                + 6.0 * measures::CONTENT_ROW_HEIGHT
                + measures::GROUP_GAP
        }
    }
}

/// Alto de la línea de ejemplo entre las dos tarjetas.
fn example_height() -> f32 {
    measures::ABOUT_SECTION_TITLE_HEIGHT
}

/// Borde superior de la línea de ejemplo.
pub(crate) fn language_example_top(count: usize, scroll: f32) -> f32 {
    language_top(scroll) + languages_height(count) + measures::GROUP_GAP
}

/// Borde superior del botón "Ajustes de idioma y región…".
pub(crate) fn language_button_top(count: usize, scroll: f32) -> f32 {
    language_card_top(2, count, scroll) + measures::CONTENT_ROW_HEIGHT + measures::ABOUT_BUTTON_GAP
}

/// Altura total del contenido, sin recortar.
pub(crate) fn language_content_height(count: usize) -> f32 {
    languages_height(count)
        + measures::GROUP_GAP
        + example_height()
        + measures::GROUP_GAP
        + 6.0 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + measures::CONTENT_ROW_HEIGHT
        + measures::ABOUT_BUTTON_GAP
        + measures::ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
}

/// Lo que se puede bajar con la rueda en esta ventana.
pub(crate) fn language_scroll_max(window_height: f32, count: usize) -> f32 {
    let visible = window_height - measures::CONTENT_TOP - measures::GROUP_GAP;
    (language_content_height(count) - visible).max(0.0)
}

/// Rectángulo del botón centrado de la página.
fn button_rect(width: f32, count: usize, scroll: f32) -> Rect {
    Rect {
        x: measures::CONTENT_LEFT + (width - measures::ABOUT_BUTTON_WIDTH) / 2.0,
        y: language_button_top(count, scroll),
        width: measures::ABOUT_BUTTON_WIDTH,
        height: measures::ABOUT_BUTTON_HEIGHT,
    }
}

/// Segmento del primer día de la semana bajo el punto, si cae en él.
pub(crate) fn language_segment_at(
    general: crate::general::GeneralPage,
    width: f32,
    count: usize,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<usize> {
    use crate::general::GeneralPage as Page;
    if general != Page::Language {
        return None;
    }
    let heights = [measures::CONTENT_ROW_HEIGHT; 1];
    let touch = content_segmented(language_card_top(2, count, scroll), &heights, 0, width);
    dokka_components::segmented_control::segmented_at(touch, 2, x, y)
}

/// Botón "Ajustes de idioma y región…" bajo el punto, si cae en él.
pub(crate) fn language_button_at(
    general: crate::general::GeneralPage,
    width: f32,
    count: usize,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<crate::general::GeneralButton> {
    use crate::general::GeneralPage as Page;
    if general != Page::Language {
        return None;
    }
    let button = button_rect(width, count, scroll);
    in_touch(&button, x, y).then_some(crate::general::GeneralButton::LanguageSettings)
}

/// `true` si el punto cae en algún control de la página.
pub(crate) fn language_control_at(width: f32, count: usize, scroll: f32, x: f32, y: f32) -> bool {
    let heights = [measures::CONTENT_ROW_HEIGHT; 1];
    let touch = content_segmented(language_card_top(2, count, scroll), &heights, 0, width);
    dokka_components::segmented_control::segmented_at(touch, 2, x, y).is_some()
        || in_touch(&button_rect(width, count, scroll), x, y)
}

/// Fila completa para reutilizar su geometría en el dibujo.
pub(crate) fn language_row(
    card: usize,
    index: usize,
    count: usize,
    width: f32,
    scroll: f32,
) -> Rect {
    let rows = if card == 0 {
        count.max(1)
    } else if card == 1 {
        6
    } else {
        1
    };
    let heights = vec![measures::CONTENT_ROW_HEIGHT; rows];
    content_row(
        language_card_top(card, count, scroll),
        &heights,
        index,
        width,
    )
}

fn in_touch(touch: &Rect, x: f32, y: f32) -> bool {
    (touch.x..touch.x + touch.width).contains(&x) && (touch.y..touch.y + touch.height).contains(&y)
}
