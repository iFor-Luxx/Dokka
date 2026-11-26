use dokka_scene::{Rect, Thumbnail};

/// Mayor rectángulo con la proporción de la ventana que no pisa la esquina reservada:
/// se prueba encima de ella y a su izquierda, y gana el que dé más superficie.
pub(crate) fn fit_thumbnail(thumbnail: &Thumbnail, aspect: f32) -> Rect {
    let slot = thumbnail.bounds;
    let corner = thumbnail.reserved_corner;
    let above = fit(slot.width, slot.height - corner, aspect);
    let beside = fit(slot.width - corner, slot.height, aspect);

    let (size, area) = if above.0 * above.1 >= beside.0 * beside.1 {
        (above, (slot.width, slot.height - corner))
    } else {
        (beside, (slot.width - corner, slot.height))
    };
    // Centrado dentro de la zona elegida.
    Rect {
        x: slot.x + (area.0 - size.0) / 2.0,
        y: slot.y + (area.1 - size.1) / 2.0,
        width: size.0,
        height: size.1,
    }
}

fn fit(width: f32, height: f32, aspect: f32) -> (f32, f32) {
    if width / height > aspect {
        (height * aspect, height)
    } else {
        (width, width / aspect)
    }
}
