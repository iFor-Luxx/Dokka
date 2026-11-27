//! Qué filas trae cada sección con ajustes y dónde está cada control.
//!
//! Las tarjetas usan el componente `settings_group`; el deslizador es el
//! componente `slider` y el interruptor el `switch`. La geometría vive aquí
//! para que el dibujo, el golpe y el arrastre usen los mismos rectángulos.
//! El Dock agrupa tamaño, ampliación y cantidad en una tarjeta y las vistas
//! previas en otra; la barra superior trae sus siete filas en una sola. Los
//! valores van de 0 a 1 en la barra y se convierten a los rangos de
//! `dokka_model`.

use dokka_components::settings_group::row_bounds;
use dokka_components::slider::slider_track_bounds;
use dokka_components::switch::switch_touch;
use dokka_model::{DOCK_ICON_SCALE_RANGE, DOCK_MAGNIFICATION_RANGE};
use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::sections::{SECTION_DOCK, SECTION_MENUBAR, SECTION_TASKBAR};

/// Deslizador arrastrado, si hay alguno a medias.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SliderTarget {
    DockScale,
    DockStrength,
}

/// Interruptor del Dock: ampliación o vistas previas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DockSwitch {
    Magnification,
    Previews,
}

/// Dónde empieza la primera tarjeta del contenido.
pub(crate) fn content_top() -> f32 {
    measures::CONTENT_TOP
}

/// Alturas de cada fila de la tarjeta `card` en la sección.
pub(crate) fn card_heights(section: usize, card: usize) -> Vec<f32> {
    if section == SECTION_DOCK {
        if card == 0 {
            vec![
                measures::CONTENT_SLIDER_ROW_HEIGHT,
                measures::CONTENT_ROW_HEIGHT,
                measures::CONTENT_SLIDER_ROW_HEIGHT,
            ]
        } else {
            vec![measures::CONTENT_ROW_HEIGHT]
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
            vec!["Vistas previas de ventanas"],
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

/// De 0-1 al rango del tamaño de iconos, limitado.
pub(crate) fn icon_scale_from(value: f32) -> f32 {
    let (minimum, maximum) = DOCK_ICON_SCALE_RANGE;
    (minimum + value.clamp(0.0, 1.0) * (maximum - minimum)).clamp(minimum, maximum)
}

/// Del tamaño de iconos a 0-1 para la barra.
pub(crate) fn icon_scale_to(scale: f32) -> f32 {
    let (minimum, maximum) = DOCK_ICON_SCALE_RANGE;
    if maximum <= minimum {
        return 0.0;
    }
    ((scale - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
}

/// De 0-1 al rango de la lupa, limitado.
pub(crate) fn magnification_from(value: f32) -> f32 {
    let (minimum, maximum) = DOCK_MAGNIFICATION_RANGE;
    (minimum + value.clamp(0.0, 1.0) * (maximum - minimum)).clamp(minimum, maximum)
}

/// De la lupa a 0-1 para la barra.
pub(crate) fn magnification_to(strength: f32) -> f32 {
    let (minimum, maximum) = DOCK_MAGNIFICATION_RANGE;
    if maximum <= minimum {
        return 0.0;
    }
    ((strength - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
}

/// `true` si la sección trae tarjetas con ajustes reales.
pub(crate) fn has_content(section: usize) -> bool {
    section == SECTION_DOCK || section == SECTION_MENUBAR || section == SECTION_TASKBAR
}

/// Interruptor bajo el punto en la sección del Dock, si hay alguno.
pub(crate) fn dock_switch_at(section: usize, x: f32, y: f32) -> Option<DockSwitch> {
    if section != SECTION_DOCK {
        return None;
    }
    let tops = card_tops(section);
    let first = tops.first()?;
    let heights = card_heights(section, 0);
    let touch = content_switch(*first, &heights, 1);
    if (touch.x..touch.x + touch.width).contains(&x)
        && (touch.y..touch.y + touch.height).contains(&y)
    {
        return Some(DockSwitch::Magnification);
    }
    let second = tops.get(1)?;
    let preview_heights = card_heights(section, 1);
    let touch = content_switch(*second, &preview_heights, 0);
    if (touch.x..touch.x + touch.width).contains(&x)
        && (touch.y..touch.y + touch.height).contains(&y)
    {
        return Some(DockSwitch::Previews);
    }
    None
}

/// Interruptor bajo el punto en la sección de la barra superior.
pub(crate) fn menubar_switch_at(section: usize, x: f32, y: f32) -> Option<usize> {
    if section != SECTION_MENUBAR {
        return None;
    }
    let tops = card_tops(section);
    let top = tops.into_iter().next()?;
    let heights = card_heights(section, 0);
    (0..7).find(|index| {
        let touch = content_switch(top, &heights, *index);
        (touch.x..touch.x + touch.width).contains(&x)
            && (touch.y..touch.y + touch.height).contains(&y)
    })
}

/// Interruptor bajo el punto en la sección de la barra de tareas.
pub(crate) fn taskbar_switch_at(section: usize, x: f32, y: f32) -> Option<usize> {
    if section != SECTION_TASKBAR {
        return None;
    }
    let tops = card_tops(section);
    let top = tops.into_iter().next()?;
    let heights = card_heights(section, 0);
    let touch = content_switch(top, &heights, 0);
    if (touch.x..touch.x + touch.width).contains(&x)
        && (touch.y..touch.y + touch.height).contains(&y)
    {
        Some(0)
    } else {
        None
    }
}

/// Deslizador bajo el punto en la sección del Dock.
pub(crate) fn dock_slider_at(section: usize, x: f32, y: f32) -> Option<SliderTarget> {
    if section != SECTION_DOCK {
        return None;
    }
    let tops = card_tops(section);
    let top = tops.first()?;
    let heights = card_heights(section, 0);
    let matches = |index: usize| {
        let area = content_slider_area(*top, &heights, index);
        (area.x..area.x + area.width).contains(&x) && (area.y..area.y + area.height).contains(&y)
    };
    if matches(0) {
        Some(SliderTarget::DockScale)
    } else if matches(2) {
        Some(SliderTarget::DockStrength)
    } else {
        None
    }
}

/// `true` si el punto cae en algún control del contenido (para no arrastrar
/// la ventana desde ahí).
pub(crate) fn content_control_at(section: usize, x: f32, y: f32) -> bool {
    dock_switch_at(section, x, y).is_some()
        || menubar_switch_at(section, x, y).is_some()
        || taskbar_switch_at(section, x, y).is_some()
        || dock_slider_at(section, x, y).is_some()
}
