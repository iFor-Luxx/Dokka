//! Qué filas trae cada sección con ajustes y dónde está cada control.
//!
//! Las tarjetas usan el componente `settings_group`; el deslizador es el
//! componente `slider`, el segmentado el `segmented_control` y el interruptor
//! el `switch`. La geometría vive aquí para que el dibujo, el golpe y el
//! arrastre usen los mismos rectángulos. El Dock agrupa tamaño, ampliación y
//! cantidad en una tarjeta, posición y ocultado en otra y tres hábitos en la
//! última; la barra superior trae sus siete filas en una sola. Los valores
//! van de 0 a 1 en la barra y se convierten a los rangos de `dokka_model`.

use dokka_components::segmented_control::segmented_touch;
use dokka_components::settings_group::row_bounds;
use dokka_components::slider::slider_track_bounds;
use dokka_components::switch::switch_touch;
use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::sections::{SECTION_DOCK, SECTION_MENUBAR, SECTION_TASKBAR};

/// Dónde empieza la primera tarjeta del contenido.
pub(crate) fn content_top() -> f32 {
    measures::CONTENT_TOP
}

/// Alturas de cada fila de la tarjeta `card` en la sección.
pub(crate) fn card_heights(section: usize, card: usize) -> Vec<f32> {
    if section == SECTION_DOCK {
        match card {
            0 => vec![
                measures::CONTENT_SLIDER_ROW_HEIGHT,
                measures::CONTENT_ROW_HEIGHT,
                measures::CONTENT_SLIDER_ROW_HEIGHT,
            ],
            1 => vec![measures::CONTENT_ROW_HEIGHT, measures::CONTENT_ROW_HEIGHT],
            _ => vec![
                measures::CONTENT_ROW_HEIGHT,
                measures::CONTENT_ROW_HEIGHT,
                measures::CONTENT_ROW_HEIGHT,
            ],
        }
    } else if section == SECTION_MENUBAR {
        vec![measures::CONTENT_ROW_HEIGHT; 7]
    } else if section == SECTION_TASKBAR {
        vec![measures::CONTENT_ROW_HEIGHT]
    } else {
        Vec::new()
    }
}

/// Altura de una tarjeta con estas filas.
pub(crate) fn card_height_for(heights: &[f32]) -> f32 {
    heights.iter().sum()
}

/// Tarjetas de la sección: cada una con sus etiquetas.
pub(crate) fn section_cards(section: usize) -> Vec<Vec<&'static str>> {
    if section == SECTION_DOCK {
        vec![
            vec![
                "Tamaño de los iconos",
                "Ampliación",
                "Cantidad de ampliación",
            ],
            vec!["Posición en pantalla", "Ocultar automáticamente"],
            vec![
                "Animar al abrir apps",
                "Mostrar indicadores de apps abiertas",
                "Vistas previas de ventanas",
            ],
        ]
    } else if section == SECTION_MENUBAR {
        vec![vec![
            "Volumen",
            "Red",
            "Música",
            "Idioma",
            "Buscar",
            "Centro de control",
            "Apps en segundo plano",
        ]]
    } else if section == SECTION_TASKBAR {
        vec![vec!["Ocultar automáticamente la barra de Windows"]]
    } else {
        Vec::new()
    }
}

/// Borde superior de cada tarjeta de la sección, en orden.
pub(crate) fn card_tops(section: usize) -> Vec<f32> {
    let cards = section_cards(section);
    let mut tops = Vec::with_capacity(cards.len());
    let mut top = content_top();
    for (card, _) in cards.iter().enumerate() {
        tops.push(top);
        top += card_height_for(&card_heights(section, card)) + measures::GROUP_GAP;
    }
    tops
}

/// Fila `index` de la tarjeta que empieza en `top` con estas alturas.
pub(crate) fn content_row(top: f32, heights: &[f32], index: usize) -> Rect {
    row_bounds(top, heights, index)
}

/// Zona de toque del interruptor de esa fila.
pub(crate) fn content_switch(top: f32, heights: &[f32], index: usize) -> Rect {
    switch_touch(content_row(top, heights, index))
}

/// Zona del segmentado de esa fila, con la misma geometría del dibujo.
pub(crate) fn content_segmented(top: f32, heights: &[f32], index: usize) -> Rect {
    segmented_touch(content_row(top, heights, index))
}

/// Zona del deslizador de esa fila, a la derecha con el mismo aire que el
/// texto a la izquierda. El área deja medio tirador a cada lado en la pista.
pub(crate) fn content_slider_area(top: f32, heights: &[f32], index: usize) -> Rect {
    let row = content_row(top, heights, index);
    Rect {
        x: row.x + row.width - measures::CONTENT_PADDING - measures::SLIDER_WIDTH,
        y: row.y + measures::SLIDER_AREA_TOP,
        width: measures::SLIDER_WIDTH,
        height: measures::SLIDER_TOUCH_HEIGHT,
    }
}

/// Barra fina del deslizador dentro de su zona de toque.
pub(crate) fn content_slider_track(top: f32, heights: &[f32], index: usize) -> Rect {
    slider_track_bounds(content_slider_area(top, heights, index))
}

/// `true` si la sección trae tarjetas con ajustes reales.
pub(crate) fn has_content(section: usize) -> bool {
    section == SECTION_DOCK || section == SECTION_MENUBAR || section == SECTION_TASKBAR
}
