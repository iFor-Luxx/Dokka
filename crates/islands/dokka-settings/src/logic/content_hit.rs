//! Golpe de los controles del contenido: qué hay bajo el punto.
//!
//! Usa la misma geometría que el dibujo para que el clic, el hover y el
//! arrastre caigan donde se ve cada control.

use super::content::{card_tops, content_segmented, content_slider_area, content_switch};
use super::general::{about_button_at as about_hit, general_row_at as general_hit};
use super::sections::{SECTION_DOCK, SECTION_GENERAL, SECTION_MENUBAR, SECTION_TASKBAR};

/// Deslizador arrastrado, si hay alguno a medias.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SliderTarget {
    DockScale,
    DockStrength,
}

/// Interruptor del Dock, en su tarjeta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DockSwitch {
    Magnification,
    AutoHide,
    AnimateLaunch,
    ShowIndicators,
    Previews,
}

/// Interruptor bajo el punto en la sección del Dock, si hay alguno.
pub(crate) fn dock_switch_at(section: usize, width: f32, x: f32, y: f32) -> Option<DockSwitch> {
    if section != SECTION_DOCK {
        return None;
    }
    let tops = card_tops(section);
    let first = tops.first()?;
    let heights = super::content::card_heights(section, 0);
    if in_touch(&content_switch(*first, &heights, 1, width), x, y) {
        return Some(DockSwitch::Magnification);
    }
    let second = tops.get(1)?;
    let mid = super::content::card_heights(section, 1);
    if in_touch(&content_switch(*second, &mid, 1, width), x, y) {
        return Some(DockSwitch::AutoHide);
    }
    let third = tops.get(2)?;
    let last = super::content::card_heights(section, 2);
    if in_touch(&content_switch(*third, &last, 0, width), x, y) {
        return Some(DockSwitch::AnimateLaunch);
    }
    if in_touch(&content_switch(*third, &last, 1, width), x, y) {
        return Some(DockSwitch::ShowIndicators);
    }
    if in_touch(&content_switch(*third, &last, 2, width), x, y) {
        return Some(DockSwitch::Previews);
    }
    None
}

/// Interruptor bajo el punto en la sección de la barra superior.
pub(crate) fn menubar_switch_at(section: usize, width: f32, x: f32, y: f32) -> Option<usize> {
    if section != SECTION_MENUBAR {
        return None;
    }
    let tops = card_tops(section);
    let top = tops.into_iter().next()?;
    let heights = super::content::card_heights(section, 0);
    (0..7).find(|index| {
        let touch = content_switch(top, &heights, *index, width);
        in_touch(&touch, x, y)
    })
}

/// Interruptor bajo el punto en la sección de la barra de tareas.
pub(crate) fn taskbar_switch_at(section: usize, width: f32, x: f32, y: f32) -> Option<usize> {
    if section != SECTION_TASKBAR {
        return None;
    }
    let tops = card_tops(section);
    let top = tops.into_iter().next()?;
    let heights = super::content::card_heights(section, 0);
    let touch = content_switch(top, &heights, 0, width);
    if in_touch(&touch, x, y) {
        Some(0)
    } else {
        None
    }
}

/// Deslizador bajo el punto en la sección del Dock.
pub(crate) fn dock_slider_at(section: usize, width: f32, x: f32, y: f32) -> Option<SliderTarget> {
    if section != SECTION_DOCK {
        return None;
    }
    let tops = card_tops(section);
    let top = tops.first()?;
    let heights = super::content::card_heights(section, 0);
    let matches = |index: usize| {
        let area = content_slider_area(*top, &heights, index, width);
        in_touch(&area, x, y)
    };
    if matches(0) {
        Some(SliderTarget::DockScale)
    } else if matches(2) {
        Some(SliderTarget::DockStrength)
    } else {
        None
    }
}

/// Segmento de posición bajo el punto en la sección del Dock.
pub(crate) fn dock_segment_at(section: usize, width: f32, x: f32, y: f32) -> Option<usize> {
    if section != SECTION_DOCK {
        return None;
    }
    let tops = card_tops(section);
    let top = tops.get(1)?;
    let heights = super::content::card_heights(section, 1);
    let touch = content_segmented(*top, &heights, 0, width);
    dokka_components::segmented_control::segmented_at(touch, 3, x, y)
}

/// Fila de General bajo el punto, con su desplazamiento aplicado.
pub(crate) fn general_row_at(
    section: usize,
    general: crate::general::GeneralPage,
    width: f32,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<usize> {
    use crate::general::GeneralPage as Page;
    if section != SECTION_GENERAL || general != Page::Root {
        return None;
    }
    general_hit(width, scroll, x, y)
}

/// Botón de "Acerca de este PC" bajo el punto, con su desplazamiento aplicado.
pub(crate) fn about_button_at(
    section: usize,
    general: crate::general::GeneralPage,
    width: f32,
    scroll: f32,
    x: f32,
    y: f32,
) -> Option<u8> {
    use crate::general::GeneralPage as Page;
    if section != SECTION_GENERAL || general != Page::About {
        return None;
    }
    about_hit(width, scroll, x, y)
}

/// `true` si el punto cae en algún control del contenido (para no arrastrar
/// la ventana desde ahí).
pub(crate) fn content_control_at(section: usize, width: f32, x: f32, y: f32) -> bool {
    dock_switch_at(section, width, x, y).is_some()
        || menubar_switch_at(section, width, x, y).is_some()
        || taskbar_switch_at(section, width, x, y).is_some()
        || dock_slider_at(section, width, x, y).is_some()
        || dock_segment_at(section, width, x, y).is_some()
}

fn in_touch(touch: &dokka_scene::Rect, x: f32, y: f32) -> bool {
    (touch.x..touch.x + touch.width).contains(&x) && (touch.y..touch.y + touch.height).contains(&y)
}
