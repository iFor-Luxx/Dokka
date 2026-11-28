//! Geometría del apartado "Almacenamiento": resumen, lista y botones.
//!
//! El resumen trae el disco, "X usados de Y" y la barra por categorías; la
//! lista trae las nueve categorías con su tamaño a la derecha; abajo van los
//! botones "Vaciar papelera" y "Ajustes de almacenamiento…". Lo que queda
//! fuera de la vista no se pinta: se lleva a tamaño cero para no reconstruir
//! capas al bajar con la rueda. Nunca hay tamaños negativos.

use dokka_scene::{Color, Drawing};
use dokka_theme::{Theme, settings as measures};

use super::super::storage_state::STORAGE_CATEGORIES;

/// Alturas de las filas de la tarjeta de resumen: disco, usado y barra.
pub(crate) fn storage_header_rows() -> [f32; 3] {
    [
        measures::CONTENT_ROW_HEIGHT,
        measures::CONTENT_ROW_HEIGHT,
        measures::STORAGE_BAR_ROW_HEIGHT,
    ]
}

/// Altura de la tarjeta de resumen.
fn header_height() -> f32 {
    storage_header_rows().iter().sum()
}

/// Alturas de la tarjeta de categorías: una fila por categoría.
pub(crate) fn storage_list_rows() -> usize {
    STORAGE_CATEGORIES
}

/// Posiciones de cada tramo del apartado con el desplazamiento aplicado.
pub(crate) struct StorageLayout {
    pub(crate) header: f32,
    pub(crate) hint: f32,
    pub(crate) list: f32,
    pub(crate) disks_title: f32,
    pub(crate) disks: f32,
    pub(crate) empty: f32,
    pub(crate) settings: f32,
    pub(crate) height: f32,
}

/// Calcula dónde empieza cada tramo; con 0 discos ese tramo no ocupa nada.
pub(crate) fn storage_layout(scroll: f32, disks: usize) -> StorageLayout {
    let header = measures::CONTENT_TOP - scroll;
    let hint = header + header_height() + measures::GROUP_GAP;
    let list = hint + measures::ABOUT_SECTION_TITLE_HEIGHT;
    let after_list = list + STORAGE_CATEGORIES as f32 * measures::CONTENT_ROW_HEIGHT;
    let (disks_title, disks, after_disks) = if disks == 0 {
        (after_list, after_list, after_list)
    } else {
        let title = after_list + measures::GROUP_GAP;
        let card = title + measures::ABOUT_SECTION_TITLE_HEIGHT;
        (
            title,
            card,
            card + disks as f32 * measures::CONTENT_ROW_HEIGHT,
        )
    };
    let empty = after_disks + measures::GROUP_GAP;
    let settings = empty + measures::ABOUT_BUTTON_HEIGHT + measures::ABOUT_BUTTON_GAP;
    let height = settings + measures::ABOUT_BUTTON_HEIGHT + measures::GROUP_GAP - header;
    StorageLayout {
        header,
        hint,
        list,
        disks_title,
        disks,
        empty,
        settings,
        height,
    }
}

/// Altura total del contenido, sin recortar.
pub(crate) fn storage_content_height(disks: usize) -> f32 {
    storage_layout(0.0, disks).height
}

/// Color del tramo y del cuadrito de la categoría `index`.
pub(crate) fn storage_category_color(theme: &Theme, index: usize) -> Color {
    theme.settings.badge[index % theme.settings.badge.len()]
}

/// Dibujo blanco de la categoría `index`.
pub(crate) fn storage_category_drawing(index: usize, color: Color) -> Drawing {
    match index {
        0 => dokka_icons::storage_apps_icon(color),
        1 => dokka_icons::storage_documents_icon(color),
        2 => dokka_icons::storage_images_icon(color),
        3 => dokka_icons::storage_music_icon(color),
        4 => dokka_icons::storage_videos_icon(color),
        5 => dokka_icons::storage_downloads_icon(color),
        6 => dokka_icons::storage_trash_icon(color),
        7 => dokka_icons::storage_windows_icon(color),
        _ => dokka_icons::storage_other_icon(color),
    }
}

/// Botón del apartado bajo el punto: 0 vaciar papelera, 1 ajustes.
pub(crate) fn storage_button_at(
    width: f32,
    scroll: f32,
    disks: usize,
    x: f32,
    y: f32,
) -> Option<u8> {
    let left = measures::CONTENT_LEFT;
    if !(left..left + width).contains(&x) {
        return None;
    }
    let layout = storage_layout(scroll, disks);
    let center = left + width / 2.0;
    let half = measures::ABOUT_BUTTON_WIDTH / 2.0;
    if !(center - half..center + half).contains(&x) {
        return None;
    }
    if (layout.empty..layout.empty + measures::ABOUT_BUTTON_HEIGHT).contains(&y) {
        return Some(0);
    }
    if (layout.settings..layout.settings + measures::ABOUT_BUTTON_HEIGHT).contains(&y) {
        return Some(1);
    }
    None
}
