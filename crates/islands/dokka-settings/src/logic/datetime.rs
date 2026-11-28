//! Geometría de "Fecha y hora": tres tarjetas y su botón.
//!
//! La página cabe sin bajar: el máximo es cero y el golpe usa el mismo
//! desplazamiento que el dibujo para no dejar piezas a medias.

use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::content::{content_row, content_switch};

/// Etiquetas de las tres tarjetas, en orden de arriba abajo.
pub(crate) fn datetime_labels(card: usize) -> [&'static str; 2] {
    match card {
        0 => ["Fecha y hora automáticas", "Servidor"],
        1 => ["Fecha y hora", "Formato de 24 horas"],
        _ => ["Zona horaria automática", "Zona horaria"],
    }
}

/// Borde superior de la página con el desplazamiento aplicado.
pub(crate) fn datetime_top(scroll: f32) -> f32 {
    measures::CONTENT_TOP - scroll
}

/// Borde superior de la tarjeta `card` (0-2).
pub(crate) fn datetime_card_top(card: usize, scroll: f32) -> f32 {
    datetime_top(scroll) + card as f32 * (2.0 * measures::CONTENT_ROW_HEIGHT + measures::GROUP_GAP)
}

/// Borde superior del botón "Ajustes de fecha y hora…".
pub(crate) fn datetime_button_top(scroll: f32) -> f32 {
    datetime_card_top(2, scroll) + 2.0 * measures::CONTENT_ROW_HEIGHT + measures::ABOUT_BUTTON_GAP
}

/// Altura total del contenido, sin recortar.
pub(crate) fn datetime_content_height() -> f32 {
    3.0 * 2.0 * measures::CONTENT_ROW_HEIGHT
        + 2.0 * measures::GROUP_GAP
        + measures::ABOUT_BUTTON_GAP
        + measures::ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
}

/// Lo que se puede bajar con la rueda en esta ventana: cabe sin bajar.
pub(crate) fn datetime_scroll_max(window_height: f32) -> f32 {
    let visible = window_height - measures::CONTENT_TOP - measures::GROUP_GAP;
    (datetime_content_height() - visible).max(0.0)
}

/// Rectángulo del botón centrado de la página.
fn button_rect(width: f32, scroll: f32) -> Rect {
    Rect {
        x: measures::CONTENT_LEFT + (width - measures::ABOUT_BUTTON_WIDTH) / 2.0,
        y: datetime_button_top(scroll),
        width: measures::ABOUT_BUTTON_WIDTH,
        height: measures::ABOUT_BUTTON_HEIGHT,
    }
}

/// Interruptor de formato de 24 horas bajo el punto, si cae en él.
pub(crate) fn datetime_switch_at(
    general: crate::general::GeneralPage,
    width: f32,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<()> {
    use crate::general::GeneralPage as Page;
    if general != Page::DateTime {
        return None;
    }
    let heights = [measures::CONTENT_ROW_HEIGHT; 2];
    let touch = content_switch(datetime_card_top(1, scroll), &heights, 1, width);
    in_touch(&touch, x, y).then_some(())
}

/// Botón "Ajustes de fecha y hora…" bajo el punto, si cae en él.
pub(crate) fn datetime_button_at(
    general: crate::general::GeneralPage,
    width: f32,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<crate::general::GeneralButton> {
    use crate::general::GeneralPage as Page;
    if general != Page::DateTime {
        return None;
    }
    let button = button_rect(width, scroll);
    in_touch(&button, x, y).then_some(crate::general::GeneralButton::DateTimeSettings)
}

/// `true` si el punto cae en algún control de la página.
pub(crate) fn datetime_control_at(width: f32, scroll: f32, x: f32, y: f32) -> bool {
    let heights = [measures::CONTENT_ROW_HEIGHT; 2];
    let touch = content_switch(datetime_card_top(1, scroll), &heights, 1, width);
    in_touch(&touch, x, y) || in_touch(&button_rect(width, scroll), x, y)
}

/// Fila completa para reutilizar su geometría en el dibujo.
pub(crate) fn datetime_row(card: usize, index: usize, width: f32, scroll: f32) -> Rect {
    let heights = [measures::CONTENT_ROW_HEIGHT; 2];
    content_row(datetime_card_top(card, scroll), &heights, index, width)
}

fn in_touch(touch: &Rect, x: f32, y: f32) -> bool {
    (touch.x..touch.x + touch.width).contains(&x) && (touch.y..touch.y + touch.height).contains(&y)
}
