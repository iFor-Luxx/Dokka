use dokka_components::nav_pill::nav_pill_contains;
use dokka_components::search_field::search_contains_at;
use dokka_components::settings_list::search_bounds;
use dokka_components::window_controls::traffic_at;
use dokka_scene::Rect;
use dokka_theme::settings as measures;

use super::content_hit::content_control_at;
use super::general::general_control_at;
use super::sections::{SECTION_DOCK, SECTION_GENERAL, SECTION_MENUBAR, SECTION_TASKBAR};

/// `true` si el punto cae en la píldora de la cabecera.
fn nav_at(x: f32, y: f32) -> bool {
    nav_pill_contains(
        Rect {
            x: measures::NAV_LEFT,
            y: measures::NAV_TOP,
            width: measures::NAV_PILL_WIDTH,
            height: measures::NAV_PILL_HEIGHT,
        },
        x,
        y,
    )
}

/// `true` si desde el punto se puede arrastrar la ventana: la franja superior
/// quitando los semáforos, la píldora, la casilla, las filas y los controles
/// del contenido (deslizadores, interruptores, filas de General y botones
/// nunca mueven la ventana). `disks` dice cuántos otros discos trae hoy
/// "Almacenamiento", para no moverla desde sus botones.
pub fn drag_zone(
    x: f32,
    y: f32,
    selected: usize,
    width: f32,
    general: crate::general::GeneralPage,
    scroll: f32,
    disks: usize,
) -> bool {
    if content_control_at(selected, width, x, y) {
        return false;
    }
    if selected == SECTION_GENERAL && general_control_at(width, scroll, general, x, y, disks) {
        return false;
    }
    // Los controles de las demás secciones no se ven, pero se comprueban
    // igual por si la sección cambia a mitad de un arrastre.
    for section in [SECTION_DOCK, SECTION_MENUBAR, SECTION_TASKBAR] {
        if section != selected && content_control_at(section, width, x, y) {
            return false;
        }
    }
    if !(0.0..measures::DRAG_HEIGHT).contains(&y) {
        return false;
    }
    if traffic_at(x, y).is_some() || nav_at(x, y) {
        return false;
    }
    if search_contains_at(search_bounds(), x, y) {
        return false;
    }
    // Las filas empiezan bajo la zona arrastrable, pero se comprueba igual.
    if dokka_components::settings_list::row_at(x, y).is_some() {
        return false;
    }
    true
}
