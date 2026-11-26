use dokka_components::search_field::search_contains_at;
use dokka_components::settings_list::search_bounds;
use dokka_components::window_controls::traffic_at;
use dokka_theme::settings as measures;

/// `true` si el punto cae en un chevrón de la cabecera (atrás o adelante).
fn nav_at(x: f32, y: f32) -> bool {
    let back = measures::NAV_LEFT..measures::NAV_LEFT + measures::NAV_SIDE;
    let forward =
        measures::NAV_LEFT + measures::NAV_SIDE..measures::NAV_LEFT + measures::NAV_SIDE * 2.0;
    (measures::NAV_TOP..measures::NAV_TOP + measures::NAV_SIDE).contains(&y)
        && (back.contains(&x) || forward.contains(&x))
}

/// `true` si desde el punto se puede arrastrar la ventana: la franja superior
/// quitando los semáforos, los chevrones, la casilla y las filas.
pub fn drag_zone(x: f32, y: f32) -> bool {
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
