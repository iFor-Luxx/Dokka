use std::sync::Arc;

use dokka_icons::{traffic_close_icon, traffic_minimize_icon, traffic_zoom_icon};
use dokka_scene::{Color, Drawing, Icon, IconImage};
use dokka_theme::{Theme, settings as measures};

/// Función que dibuja el símbolo de un semáforo en un color.
type TrafficIcon = fn(Color) -> Drawing;

const GLYPHS: [TrafficIcon; 3] = [traffic_close_icon, traffic_minimize_icon, traffic_zoom_icon];

/// Símbolos de los semáforos ya construidos con los colores del tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta cuando recibe
/// otro dibujo. Sin tema en caliente, basta con crearlos una vez. Ocultos
/// comparten un dibujo vacío que pinta transparente.
#[derive(Debug, Clone)]
pub struct TrafficGlyphs {
    shown: [Arc<Drawing>; 3],
    hidden: Arc<Drawing>,
}

impl TrafficGlyphs {
    pub fn new(theme: &Theme) -> Self {
        let [close, minimize, zoom] = theme.settings.traffic_symbol;
        let colors = [close, minimize, zoom];
        Self {
            shown: [
                Arc::new(GLYPHS[0](colors[0])),
                Arc::new(GLYPHS[1](colors[1])),
                Arc::new(GLYPHS[2](colors[2])),
            ],
            hidden: Arc::new(Drawing { shapes: Vec::new() }),
        }
    }
}

/// Los tres símbolos centrados en sus círculos. Siempre hay tres, con el
/// mismo estilo: ocultos pintan vacío para no reconstruir capas al pasar el
/// cursor, como el cursor de la casilla de búsqueda.
pub fn traffic_glyph_icons(theme: &Theme, visible: bool, glyphs: &TrafficGlyphs) -> Vec<Icon> {
    let inset = (measures::TRAFFIC_DIAMETER - measures::TRAFFIC_GLYPH_SIDE) / 2.0;
    glyphs
        .shown
        .iter()
        .enumerate()
        .map(|(index, drawing)| {
            let image = if visible {
                Arc::clone(drawing)
            } else {
                Arc::clone(&glyphs.hidden)
            };
            Icon {
                x: measures::TRAFFIC_LEFT + index as f32 * measures::TRAFFIC_PITCH + inset,
                y: measures::TRAFFIC_TOP + inset,
                side: measures::TRAFFIC_GLYPH_SIDE,
                image: Some(IconImage::Drawing(image)),
                placeholder: theme.settings.traffic[index],
                placeholder_radius: 0.0,
            }
        })
        .collect()
}
