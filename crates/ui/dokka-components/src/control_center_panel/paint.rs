//! Ayudas para pintar las piezas del Centro de control: cristal translúcido,
//! formas planas, iconos centrados y textos. Cada zona añade lo suyo a `Pieces`.

use std::sync::Arc;

use dokka_scene::{Color, Drawing, Icon, IconImage, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, control_center as measures, tray_glass as glass};

/// Lo que aporta una zona del panel: piezas, planos, textos e iconos.
#[derive(Debug, Clone, Default)]
pub(crate) struct Pieces {
    pub tiles: Vec<Panel>,
    pub flats: Vec<Panel>,
    pub texts: Vec<Text>,
    pub icons: Vec<Icon>,
}

impl Pieces {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

/// Pieza de cristal casi sin tinte con su borde fino y brillante.
pub(crate) fn tile(theme: &Theme, bounds: Rect, radius: f32) -> Panel {
    tile_tinted(theme, bounds, radius, theme.tray.tile_translucent)
}

/// Pieza redonda blanca (círculo activo) con el mismo borde luminoso.
pub(crate) fn tile_tinted(theme: &Theme, bounds: Rect, radius: f32, tint: Color) -> Panel {
    Panel {
        bounds,
        corner_radius: clip(radius, bounds),
        border_width: glass::TILE_BORDER_WIDTH,
        tint,
        border: theme.tray.tile_border,
    }
}

/// Forma plana sin borde: círculo interior, carátula o barra.
pub(crate) fn flat(bounds: Rect, radius: f32, color: Color) -> Panel {
    Panel {
        bounds,
        corner_radius: clip(radius, bounds),
        border_width: 0.0,
        tint: color,
        border: color,
    }
}

/// Recorta el radio a la mitad del lado menor; nunca negativo.
pub(crate) fn clip(radius: f32, bounds: Rect) -> f32 {
    radius
        .min(bounds.width / 2.0)
        .min(bounds.height / 2.0)
        .max(0.0)
}

/// Icono centrado en su zona, con el dibujo ya construido.
pub(crate) fn picture(theme: &Theme, drawing: &Arc<Drawing>, area: Rect, side: f32) -> Icon {
    Icon {
        x: area.x + (area.width - side) / 2.0,
        y: area.y + (area.height - side) / 2.0,
        side,
        image: Some(IconImage::Drawing(Arc::clone(drawing))),
        placeholder: theme.tray.circle_icon,
        placeholder_radius: 0.0,
    }
}

pub(crate) fn title(theme: &Theme, x: f32, center_y: f32, content: String) -> Text {
    Text {
        x,
        center_y,
        content,
        size: glass::TITLE_SIZE,
        is_bold: true,
        color: theme.tray.text,
        align: TextAlign::Left,
    }
}

pub(crate) fn subtitle(theme: &Theme, x: f32, center_y: f32, content: String) -> Text {
    Text {
        x,
        center_y,
        content,
        size: glass::SUBTITLE_SIZE,
        is_bold: false,
        color: theme.tray.secondary,
        align: TextAlign::Left,
    }
}

pub(crate) fn shorten(content: &str) -> String {
    if content.chars().count() <= measures::TEXT_CHARACTERS {
        return content.to_owned();
    }
    let kept: String = content
        .chars()
        .take(measures::TEXT_CHARACTERS - 1)
        .collect();
    format!("{}…", kept.trim_end())
}
