//! Galería de controles del modo editar, estilo macOS Tahoe.
//!
//! Ventana flotante de cristal claro (~760 x 900, radio 26) que se encoge en
//! pantallas bajas. Columna izquierda de 245 con la casilla y las categorías;
//! a la derecha las secciones con cada control al mismo tamaño que en el
//! centro y su nombre debajo; abajo fijos la línea, el aviso y "Aceptar".
//! Radios concéntricos: el interior recorta la separación al exterior.

use dokka_scene::{Rect, Size};

use super::menubar as bar;

/// Ancho fijo de la ventana (~760).
pub const WIDTH: f32 = 760.0;
/// Alto de la ventana (~900); se encoge en pantallas bajas.
pub const HEIGHT: f32 = 900.0;
/// Esquinas muy redondeadas de la ventana.
pub const CORNER_RADIUS: f32 = 26.0;
/// Ancho del borde fino y luminoso.
pub const BORDER_WIDTH: f32 = 1.0;
/// Aire exterior de la ventana al borde de la pantalla.
pub const MARGIN: f32 = 16.0;
/// Hueco para no tapar el centro al desplazar a la izquierda.
pub const CENTER_GAP: f32 = 16.0;

/// Ancho de la columna izquierda con la casilla y las categorías.
pub const LEFT_WIDTH: f32 = 245.0;
/// Línea fina que separa la columna y el pie.
pub const DIVIDER_HEIGHT: f32 = 1.0;
/// Aire interior de la ventana.
pub const PADDING: f32 = 16.0;

/// Alto de la casilla redondeada de búsqueda.
pub const SEARCH_HEIGHT: f32 = 36.0;
/// Radio de la casilla, concéntrico con la ventana.
pub const SEARCH_RADIUS: f32 = 18.0;
/// Lado de la lupa dentro de la casilla.
pub const SEARCH_ICON_SIDE: f32 = 16.0;
/// Aire entre el borde de la casilla y la lupa.
pub const SEARCH_ICON_INSET: f32 = 12.0;
/// Aire entre la lupa y el texto.
pub const SEARCH_TEXT_GAP: f32 = 8.0;
/// Texto de la casilla.
pub const SEARCH_TEXT_SIZE: f32 = 14.0;
/// Aire entre la última letra y el cursor.
pub const SEARCH_CURSOR_GAP: f32 = 2.0;
/// Ancho y alto del cursor vertical.
pub const SEARCH_CURSOR_WIDTH: f32 = 2.0;
pub const SEARCH_CURSOR_HEIGHT: f32 = 18.0;
/// Texto con la casilla vacía.
pub const SEARCH_PLACEHOLDER: &str = "Buscar controles";

/// Alto de cada fila de categoría.
pub const ROW_HEIGHT: f32 = 44.0;
/// Radio del fondo gris de la elegida, concéntrico.
pub const ROW_RADIUS: f32 = 10.0;
/// Margen lateral del fondo de la elegida.
pub const ROW_MARGIN: f32 = 8.0;
/// Lado del cuadrito redondeado de color.
pub const BADGE_SIDE: f32 = 28.0;
/// Esquinas del cuadrito.
pub const BADGE_RADIUS: f32 = 8.0;
/// Lado del dibujo blanco sobre el cuadrito.
pub const BADGE_ICON_SIDE: f32 = 16.0;
/// Aire entre el cuadrito y el nombre.
pub const BADGE_TEXT_GAP: f32 = 10.0;
/// Nombre de la categoría.
pub const ROW_TEXT_SIZE: f32 = 14.0;

/// Título pequeño gris de cada sección.
pub const SECTION_TITLE_SIZE: f32 = 12.0;
/// Alto reservado al título con su línea.
pub const SECTION_TITLE_HEIGHT: f32 = 22.0;
/// Hueco entre secciones y entre piezas y etiquetas.
pub const SECTION_GAP: f32 = 16.0;
pub const LABEL_GAP: f32 = 6.0;
/// Nombre centrado bajo cada control.
pub const LABEL_TEXT_SIZE: f32 = 12.0;
/// Alto reservado a cada nombre (una línea).
pub const LABEL_HEIGHT: f32 = 16.0;

/// Alto del pie fijo con la línea, el aviso y el botón.
pub const FOOTER_HEIGHT: f32 = 88.0;
/// Aviso del pie.
pub const HINT_TEXT_SIZE: f32 = 13.0;
/// Botón azul redondeado "Aceptar".
pub const ACCEPT_WIDTH: f32 = 120.0;
pub const ACCEPT_HEIGHT: f32 = 36.0;
pub const ACCEPT_RADIUS: f32 = 18.0;
pub const ACCEPT_TEXT_SIZE: f32 = 14.0;

/// Sombra suave bajo la ventana.
pub const SHADOW_BLUR: f32 = 28.0;
pub const SHADOW_OFFSET_Y: f32 = 8.0;
/// Aire para no recortar la sombra en las zonas activas.
pub const SHADOW_MARGIN: f32 = 40.0;
/// Paso de la rueda en cada giro.
pub const SCROLL_STEP: f32 = 48.0;

/// Tamaño de la ventana para este monitor: se encoge en pantallas bajas.
pub fn gallery_size(screen_height: f32) -> Size {
    let free = screen_height - bar::HEIGHT - MARGIN * 2.0;
    Size {
        width: WIDTH,
        height: HEIGHT.min(free.max(0.0)),
    }
}

/// Ventana centrada sin tapar el centro de arriba a la derecha: si no cabe,
/// se desplaza a la izquierda dejando un hueco.
pub fn gallery_rect(bar_width: f32, screen_height: f32, control: Option<Rect>) -> Rect {
    let size = gallery_size(screen_height);
    let mut x = ((bar_width - size.width) / 2.0).max(0.0);
    let free = (screen_height - bar::HEIGHT - size.height).max(0.0);
    let mut y = bar::HEIGHT + bar::MENU_GAP + free / 2.0;
    y = y.min(screen_height - size.height - MARGIN).max(bar::HEIGHT);
    if let Some(center) = control {
        let right = x + size.width;
        let gap = center.x - CENTER_GAP;
        if right > gap && y < center.y + center.height + CENTER_GAP {
            x = (gap - size.width).max(0.0);
        }
    }
    Rect {
        x,
        y,
        width: size.width,
        height: size.height,
    }
}

/// Zona derecha con las secciones.
pub fn grid_box(gallery: Rect) -> Rect {
    Rect {
        x: gallery.x + LEFT_WIDTH,
        y: gallery.y,
        width: gallery.width - LEFT_WIDTH,
        height: gallery.height - FOOTER_HEIGHT,
    }
}
