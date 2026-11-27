use std::sync::Arc;

use dokka_icons::{
    traffic_circle, traffic_close_button, traffic_minimize_button, traffic_zoom_button,
};
use dokka_scene::{Drawing, Icon, IconImage};
use dokka_theme::{Theme, settings as measures};

/// Botones completos de los semáforos: cada uno es un solo dibujo con su
/// círculo de color y, con el cursor encima, su símbolo oscuro. Sin foco, los
/// tres comparten el mismo gris. Se crean una vez por estado y se reutiliza
/// el mismo `Arc`: la plataforma solo repinta cuando cambia el dibujo, sin
/// reconstruir nada al pasar el cursor ni al perder el foco.
#[derive(Debug, Clone)]
pub struct TrafficButtons {
    normal: [Arc<Drawing>; 3],
    hovered: [Arc<Drawing>; 3],
    inactive: Arc<Drawing>,
}

impl TrafficButtons {
    pub fn new(theme: &Theme) -> Self {
        let [red, yellow, green] = theme.settings.traffic;
        let [close, minimize, zoom] = theme.settings.traffic_symbol;
        Self {
            normal: [
                Arc::new(traffic_circle(red)),
                Arc::new(traffic_circle(yellow)),
                Arc::new(traffic_circle(green)),
            ],
            hovered: [
                Arc::new(traffic_close_button(red, close)),
                Arc::new(traffic_minimize_button(yellow, minimize)),
                Arc::new(traffic_zoom_button(green, zoom)),
            ],
            inactive: Arc::new(traffic_circle(theme.settings.traffic_inactive)),
        }
    }

    fn drawing(&self, index: usize, focused: bool, hovered: bool) -> &Arc<Drawing> {
        if !focused {
            &self.inactive
        } else if hovered {
            match self.hovered.get(index) {
                Some(drawing) => drawing,
                None => &self.inactive,
            }
        } else {
            match self.normal.get(index) {
                Some(drawing) => drawing,
                None => &self.inactive,
            }
        }
    }
}

/// Los tres botones a su diámetro y posición reales. Siempre hay tres, con
/// imagen propia: al pasar el cursor solo cambia el dibujo, como el cursor
/// de la casilla de búsqueda.
pub fn traffic_icons(
    theme: &Theme,
    drawings: &TrafficButtons,
    focused: bool,
    hovered: bool,
) -> Vec<Icon> {
    (0..3)
        .map(|index| Icon {
            x: measures::TRAFFIC_LEFT + index as f32 * measures::TRAFFIC_PITCH,
            y: measures::TRAFFIC_TOP,
            side: measures::TRAFFIC_DIAMETER,
            image: Some(IconImage::Drawing(Arc::clone(
                drawings.drawing(index, focused, hovered),
            ))),
            placeholder: if focused {
                theme.settings.traffic[index]
            } else {
                theme.settings.traffic_inactive
            },
            placeholder_radius: 0.0,
        })
        .collect()
}
