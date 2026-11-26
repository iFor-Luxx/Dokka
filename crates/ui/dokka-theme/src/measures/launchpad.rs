//! Medidas del Launchpad a pantalla completa.
//!
//! Todo es proporcional al tamaño de pantalla: las razones se midieron sobre
//! 2560×1440 lógicas (centros de columna 470–2090, icono 92, primera fila en
//! y≈285, casilla en y≈125 y puntos en y≈1013).

/// Columnas fijas: se llenan de izquierda a derecha, sin centrar las que sobran.
pub const COLUMNS: usize = 7;
/// Ancho del icono respecto al ancho de pantalla (92 sobre 2560).
pub const ICON_WIDTH_RATIO: f32 = 92.0 / 2560.0;
/// Distancia entre centros de columna respecto al ancho (270 sobre 2560).
pub const COLUMN_PITCH_RATIO: f32 = 270.0 / 2560.0;
/// Centro de la primera columna respecto al ancho (470 sobre 2560).
pub const FIRST_CENTER_RATIO: f32 = 470.0 / 2560.0;
/// Borde superior de la primera fila respecto al alto (285 sobre 1440).
pub const TOP_RATIO: f32 = 285.0 / 1440.0;
/// Separación entre filas respecto al alto (48 sobre 1440).
pub const ROW_GAP_HEIGHT_RATIO: f32 = 48.0 / 1440.0;
/// Reserva entre la última fila y los puntos, respecto al alto (48 sobre 1440).
pub const GRID_BOTTOM_MARGIN_RATIO: f32 = 48.0 / 1440.0;
/// Lado para el que se definió el radio de la baldosa.
pub const TILE_REFERENCE_SIDE: f32 = 90.0;
/// Radio de la baldosa uniforme, para `TILE_REFERENCE_SIDE`.
pub const TILE_RADIUS: f32 = 20.0;
/// Parte del lado que ocupa el glifo sobre la baldosa.
pub const TILE_CONTENT_SCALE: f32 = 0.66;
/// Aire entre el icono y su nombre.
pub const TEXT_GAP: f32 = 6.0;
pub const TEXT_SIZE: f32 = 13.0;
/// Alto aproximado de una línea de texto.
pub const TEXT_HEIGHT: f32 = 18.0;
pub const HIGHLIGHT_RADIUS: f32 = 16.0;
/// Aire del resaltado alrededor del icono y su nombre.
pub const HIGHLIGHT_PADDING: f32 = 12.0;
/// Nombres largos recortados con "…".
pub const MAX_NAME_CHARACTERS: usize = 18;

/// Ancho de la casilla de búsqueda respecto al ancho (340 sobre 2560).
pub const SEARCH_WIDTH_RATIO: f32 = 340.0 / 2560.0;
/// Alto de la casilla respecto al alto (34 sobre 1440).
pub const SEARCH_HEIGHT_RATIO: f32 = 34.0 / 1440.0;
/// Borde superior de la casilla respecto al alto (125 sobre 1440).
pub const SEARCH_TOP_RATIO: f32 = 125.0 / 1440.0;
/// Tamaño del texto escrito y del marcador "Buscar".
pub const SEARCH_TEXT_SIZE: f32 = 15.0;
/// Grosor del borde de la casilla.
pub const SEARCH_BORDER_WIDTH: f32 = 1.0;
/// Lado de la lupa, en unidades lógicas.
pub const SEARCH_ICON_SIDE: f32 = 20.0;
/// Tamaño del glifo Lucide dentro del lienzo de 100 para la lupa.
pub const SEARCH_GLYPH_SIZE: f32 = 100.0;
/// Aire entre el borde de la casilla y la lupa.
pub const SEARCH_ICON_INSET: f32 = 14.0;
/// Texto que se muestra cuando no se ha escrito nada.
pub const SEARCH_PLACEHOLDER: &str = "Buscar";
/// Mitad del ancho aproximado de "Buscar" a su tamaño nuevo: centra el grupo de lupa y texto.
pub const SEARCH_PLACEHOLDER_HALF_WIDTH: f32 = 24.0;
/// Aire entre la lupa y el texto de la casilla.
pub const SEARCH_MAGNIFIER_GAP: f32 = 10.0;
/// Distancia del borde izquierdo al texto escrito: lupa más aire a ambos lados.
pub const SEARCH_TEXT_INSET: f32 = SEARCH_ICON_INSET + SEARCH_ICON_SIDE + SEARCH_MAGNIFIER_GAP;
/// Cuántos caracteres guarda la búsqueda.
pub const SEARCH_MAX_CHARACTERS: usize = 32;
/// Mensaje centrado cuando el filtro no encuentra nada.
pub const EMPTY_TEXT: &str = "Sin resultados";
/// Tamaño del mensaje de búsqueda vacía.
pub const EMPTY_TEXT_SIZE: f32 = 15.0;

/// Línea de los puntos respecto al alto (1013 sobre 1440).
pub const DOTS_CENTER_Y_RATIO: f32 = 1013.0 / 1440.0;
/// Diámetro de cada punto respecto al ancho (8 sobre 2560).
pub const DOT_DIAMETER_RATIO: f32 = 8.0 / 2560.0;
/// Distancia entre centros de puntos respecto al ancho (24 sobre 2560).
pub const DOT_PITCH_RATIO: f32 = 24.0 / 2560.0;
/// Lado del cuadrado que responde al clic en un punto.
pub const DOT_HIT_SIDE: f32 = 24.0;
/// Diámetro del aro claro tras el punto actual respecto al ancho (11 sobre 2560):
/// lo hace algo mayor sin cambiar el tamaño de los puntos.
pub const DOT_ACTIVE_HALO_RATIO: f32 = 11.0 / 2560.0;
/// Cuánto tarda la cuadrícula en deslizarse a otra página, en segundos.
pub const PAGE_SLIDE_SECONDS: f32 = 0.3;
/// Cuánto cede la cuadrícula al intentar pasar de la primera o la última página
/// respecto al ancho (40 sobre 2560).
pub const PAGE_NUDGE_RATIO: f32 = 40.0 / 2560.0;

/// Lado del icono de app para este ancho de pantalla.
pub fn icon_side(screen_width: f32) -> f32 {
    screen_width * ICON_WIDTH_RATIO
}

/// Distancia entre centros de columna para este ancho.
pub fn column_pitch(screen_width: f32) -> f32 {
    screen_width * COLUMN_PITCH_RATIO
}

/// Centro de la primera columna para este ancho.
pub fn first_center_x(screen_width: f32) -> f32 {
    screen_width * FIRST_CENTER_RATIO
}

/// Borde superior de la primera fila para este alto.
pub fn grid_top(screen_height: f32) -> f32 {
    screen_height * TOP_RATIO
}

/// Separación entre filas para este alto.
pub fn row_gap(screen_height: f32) -> f32 {
    screen_height * ROW_GAP_HEIGHT_RATIO
}

/// Alto de una fila: icono, aire y nombre.
pub fn row_height(screen_width: f32) -> f32 {
    icon_side(screen_width) + TEXT_GAP + TEXT_HEIGHT
}

/// Línea central de los puntos de página para este alto.
pub fn dots_center_y(screen_height: f32) -> f32 {
    screen_height * DOTS_CENTER_Y_RATIO
}

/// Diámetro de los puntos de página para este ancho.
pub fn dot_diameter(screen_width: f32) -> f32 {
    screen_width * DOT_DIAMETER_RATIO
}

/// Distancia entre centros de puntos para este ancho.
pub fn dot_pitch(screen_width: f32) -> f32 {
    screen_width * DOT_PITCH_RATIO
}

/// Ancho de la casilla de búsqueda para este ancho.
pub fn search_width(screen_width: f32) -> f32 {
    screen_width * SEARCH_WIDTH_RATIO
}

/// Alto de la casilla de búsqueda para este alto.
pub fn search_height(screen_height: f32) -> f32 {
    screen_height * SEARCH_HEIGHT_RATIO
}

/// Borde superior de la casilla para este alto.
pub fn search_top(screen_height: f32) -> f32 {
    screen_height * SEARCH_TOP_RATIO
}

/// Centro vertical del mensaje de búsqueda vacía: a media cuadrícula.
pub fn empty_center_y(screen_height: f32) -> f32 {
    (grid_top(screen_height) + dots_center_y(screen_height)) / 2.0
}

/// Lado del aro claro tras el punto actual para este ancho.
pub fn dot_halo_diameter(screen_width: f32) -> f32 {
    screen_width * DOT_ACTIVE_HALO_RATIO
}

/// Cuánto cede la cuadrícula al intentar salir de los extremos para este ancho.
pub fn page_nudge_distance(screen_width: f32) -> f32 {
    screen_width * PAGE_NUDGE_RATIO
}

/// Radio de la casilla: un cuarto de su alto.
pub fn search_radius(screen_height: f32) -> f32 {
    search_height(screen_height) / 4.0
}
