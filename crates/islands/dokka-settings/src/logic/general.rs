//! Geometría de la sección General: lista y ficha del equipo.
//!
//! La lista trae una tarjeta de cabecera centrada y dos tarjetas con filas
//! que entran con chevrón. La ficha "Acerca de este PC" es más alta que la
//! ventana: la zona derecha se desplaza con la rueda y lo que queda fuera
//! no se pinta. Radios concéntricos como el resto.

use dokka_scene::{Color, Drawing};
use dokka_theme::{Theme, settings as measures};

use super::sections::SECTION_GENERAL;

/// Filas de la lista de General: 3 + 2.
pub(crate) const GENERAL_ROWS: usize = 5;

/// Nombres de las filas de General, en orden de arriba abajo.
pub(crate) fn general_rows() -> [&'static str; GENERAL_ROWS] {
    [
        "Acerca de este PC",
        "Actualización de Dokka",
        "Almacenamiento",
        "Fecha y hora",
        "Idioma y región",
    ]
}

/// Tarjetas de la lista: primera con 3 filas, segunda con 2.
pub(crate) fn general_cards() -> [usize; 2] {
    [3, 2]
}

/// Color del cuadrito de cada fila de General.
pub(crate) fn general_badge_color(theme: &Theme, row: usize) -> Color {
    // Grises y azules de Tahoe: los mismos de la barra, en otro orden.
    let badge = theme.settings.badge;
    match row {
        0 => badge[0],
        1 => badge[1],
        2 => badge[3],
        3 => badge[4],
        _ => badge[2],
    }
}

/// Dibujo blanco de cada fila de General.
pub(crate) fn general_row_drawing(row: usize, color: Color) -> Drawing {
    match row {
        0 => dokka_icons::general_about_icon(color),
        1 => dokka_icons::general_update_icon(color),
        2 => dokka_icons::general_storage_icon(color),
        3 => dokka_icons::general_datetime_icon(color),
        _ => dokka_icons::general_language_icon(color),
    }
}

/// Alto de la tarjeta de cabecera: engranaje grande, título y ayuda.
pub(crate) const GENERAL_HEADER_HEIGHT: f32 = 176.0;

/// Lado del engranaje grande de la cabecera.
pub(crate) const GENERAL_GEAR_SIDE: f32 = 56.0;

/// Aire entre el engranaje y el título, y entre el título y la ayuda.
pub(crate) const GENERAL_HEADER_GAP: f32 = 8.0;

/// Tamaño del título "General" en su tarjeta.
pub(crate) const GENERAL_HEADER_TITLE: f32 = 20.0;

/// Tamaño de la ayuda bajo el título.
pub(crate) const GENERAL_HEADER_SUBTITLE: f32 = 13.0;

/// Texto de ayuda de la cabecera de General.
pub(crate) const GENERAL_HEADER_HELP: &str = "Ajustes generales de tu equipo y de Dokka: actualizaciones, almacenamiento, fecha, idioma y más.";

fn header_top(scroll: f32) -> f32 {
    measures::CONTENT_TOP - scroll
}

/// Borde superior de la tarjeta `card` (0 cabecera, 1 tres filas, 2 dos filas).
pub(crate) fn general_card_top(card: usize, scroll: f32) -> f32 {
    let top = header_top(scroll);
    match card {
        0 => top,
        1 => top + GENERAL_HEADER_HEIGHT + measures::GROUP_GAP,
        _ => {
            top + GENERAL_HEADER_HEIGHT
                + measures::GROUP_GAP
                + 3.0 * measures::CONTENT_ROW_HEIGHT
                + measures::GROUP_GAP
        }
    }
}

/// Fila `row` (0-4) de la lista bajo el punto, con el desplazamiento aplicado.
/// `None` si cae en la cabecera o fuera de las tarjetas.
pub(crate) fn general_row_at(width: f32, scroll: f32, x: f32, y: f32) -> Option<usize> {
    if !(measures::CONTENT_LEFT..measures::CONTENT_LEFT + width).contains(&x) {
        return None;
    }
    let first = general_card_top(1, scroll);
    if (first..first + 3.0 * measures::CONTENT_ROW_HEIGHT).contains(&y) {
        let row = ((y - first) / measures::CONTENT_ROW_HEIGHT).floor() as usize;
        return Some(row.min(2));
    }
    let second = general_card_top(2, scroll);
    if (second..second + 2.0 * measures::CONTENT_ROW_HEIGHT).contains(&y) {
        let row = ((y - second) / measures::CONTENT_ROW_HEIGHT).floor() as usize;
        return Some(3 + row.min(1));
    }
    None
}

/// `true` si el punto cae en algún control de General (filas o botones).
pub(crate) fn general_control_at(
    width: f32,
    scroll: f32,
    general: crate::general::GeneralPage,
    x: f32,
    y: f32,
) -> bool {
    use crate::general::GeneralPage as Page;
    match general {
        Page::Root => general_row_at(width, scroll, x, y).is_some(),
        Page::About => about_button_at(width, scroll, x, y).is_some(),
        _ => false,
    }
}

// --- Ficha "Acerca de este PC" ---

/// Alto del héroe: dibujo del PC, modelo y línea gris.
pub(crate) const ABOUT_HERO_HEIGHT: f32 = 148.0;

/// Lado del dibujo del PC.
pub(crate) const ABOUT_PC_SIDE: f32 = 72.0;

/// Tamaño del nombre del modelo en grande.
pub(crate) const ABOUT_MODEL_SIZE: f32 = 20.0;

/// Alto de cada cabecera de sección ("Windows", "Pantalla"...).
pub(crate) const ABOUT_SECTION_TITLE: f32 = 26.0;

/// Tamaño del texto de cada cabecera de sección.
pub(crate) const ABOUT_SECTION_SIZE: f32 = 13.0;

/// Alto de cada botón de la ficha.
pub(crate) const ABOUT_BUTTON_HEIGHT: f32 = 32.0;

/// Radio de cada botón, concéntrico con las tarjetas.
pub(crate) const ABOUT_BUTTON_RADIUS: f32 = 16.0;

/// Aire entre la tarjeta y su botón.
pub(crate) const ABOUT_BUTTON_GAP: f32 = 8.0;

/// Filas de la tarjeta principal: Nombre, Procesador, Gráfica, Memoria y Serie.
pub(crate) fn about_main_rows() -> [&'static str; 5] {
    [
        "Nombre",
        "Procesador",
        "Gráfica",
        "Memoria",
        "Número de serie",
    ]
}

/// Altura total del contenido de la ficha, sin recortar.
pub(crate) fn about_content_height() -> f32 {
    ABOUT_HERO_HEIGHT
        + measures::GROUP_GAP
        + 5.0 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + ABOUT_SECTION_TITLE
        + 2.0 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + ABOUT_SECTION_TITLE
        + 2.0 * measures::CONTENT_ROW_HEIGHT
        + ABOUT_BUTTON_GAP
        + ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
        + ABOUT_SECTION_TITLE
        + measures::CONTENT_ROW_HEIGHT
        + ABOUT_BUTTON_GAP
        + ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
        + ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
}

/// Lo que se puede bajar con la rueda en esta ventana.
/// La lista cabe sin bajar; la ficha "Acerca" es la más alta.
pub(crate) fn general_scroll_max(window_height: f32) -> f32 {
    let visible = window_height - measures::CONTENT_TOP - 16.0;
    let needed = about_content_height();
    (needed - visible).max(0.0)
}

/// Borde superior de la ficha "Acerca" con el desplazamiento aplicado.
pub(crate) fn about_top(scroll: f32) -> f32 {
    measures::CONTENT_TOP - scroll
}

/// Botón de la ficha bajo el punto: 0 pantalla, 1 almacenamiento, 2 informe.
pub(crate) fn about_button_at(width: f32, scroll: f32, x: f32, y: f32) -> Option<u8> {
    let left = measures::CONTENT_LEFT;
    if !(left..left + width).contains(&x) {
        return None;
    }
    let top = about_top(scroll);
    // Botón "Ajustes de pantalla…": bajo su tarjeta de 2 filas.
    let display_button = top
        + ABOUT_HERO_HEIGHT
        + measures::GROUP_GAP
        + 5.0 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + ABOUT_SECTION_TITLE
        + 2.0 * measures::CONTENT_ROW_HEIGHT
        + measures::GROUP_GAP
        + ABOUT_SECTION_TITLE
        + 2.0 * measures::CONTENT_ROW_HEIGHT
        + ABOUT_BUTTON_GAP;
    if (display_button..display_button + ABOUT_BUTTON_HEIGHT).contains(&y) {
        return Some(0);
    }
    let storage_button = display_button
        + ABOUT_BUTTON_HEIGHT
        + measures::GROUP_GAP
        + ABOUT_SECTION_TITLE
        + measures::CONTENT_ROW_HEIGHT
        + ABOUT_BUTTON_GAP;
    if (storage_button..storage_button + ABOUT_BUTTON_HEIGHT).contains(&y) {
        return Some(1);
    }
    let report_button =
        storage_button + ABOUT_BUTTON_HEIGHT + measures::GROUP_GAP + ABOUT_BUTTON_HEIGHT;
    // El informe va centrado abajo: ocupa el centro de la tarjeta.
    let center = left + width / 2.0;
    let half = 110.0;
    if !(center - half..center + half).contains(&x) {
        return None;
    }
    if (report_button..report_button + ABOUT_BUTTON_HEIGHT).contains(&y) {
        return Some(2);
    }
    None
}
