//! Iconos provisionales de la fila derecha de la barra (Lucide, blancos).
//! Los paneles reales traerán sus propios dibujos si los necesitan;
//! estos solo marcan el sitio hasta que lleguen sus componentes.

use std::sync::Arc;

use dokka_scene::{Color, DRAWING_SIDE, Drawing, Shape};
use dokka_theme::{Theme, menubar_tray as measures};

fn glyph(icon: lucide_icons::Icon, color: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Glyph {
            glyph: char::from(icon),
            x: 50.0,
            y: 50.0,
            size: measures::GLYPH_SIZE,
            color,
        }],
    }
}

/// Dibujos de la fila derecha, creados una vez. Blancos de la barra.
#[derive(Debug, Clone)]
pub struct TrayIcons {
    pub control: Arc<Drawing>,
    pub search: Arc<Drawing>,
    pub music: Arc<Drawing>,
    pub network: Arc<Drawing>,
    pub volume: Arc<Drawing>,
}

impl TrayIcons {
    pub fn new(theme: &Theme) -> Self {
        let color = theme.menubar.text;
        Self {
            control: Arc::new(glyph(lucide_icons::Icon::SlidersHorizontal, color)),
            search: Arc::new(glyph(lucide_icons::Icon::Search, color)),
            music: Arc::new(glyph(lucide_icons::Icon::Music, color)),
            network: Arc::new(glyph(lucide_icons::Icon::Wifi, color)),
            volume: Arc::new(glyph(lucide_icons::Icon::Volume2, color)),
        }
    }
}

/// Lado efectivo del lienzo Lucide (para medidas que lo necesiten).
pub const TRAY_GLYPH_SIDE: f32 = DRAWING_SIDE;
