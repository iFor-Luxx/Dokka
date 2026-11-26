//! Medidas y colores del dock, en unidades lógicas.

use dokka_scene::Color;

/// Distancia entre el dock y el borde inferior del área de trabajo.
pub const DOCK_BOTTOM_MARGIN: f32 = 8.0;

pub const HEIGHT: f32 = 80.0;
pub const SLOT: f32 = 52.0;
pub const GAP: f32 = 12.0;
pub const PADDING: f32 = 14.0;
pub const CORNER_RADIUS: f32 = 24.0;
pub const TINT: Color = Color::rgba(255, 255, 255, 40);
pub const BORDER: Color = Color::rgba(255, 255, 255, 96);
pub const PLACEHOLDER: Color = Color::rgba(255, 255, 255, 80);
pub const PLACEHOLDER_RADIUS: f32 = 12.0;

/// Ancho extra que ocupa la línea separadora y su aire.
pub const DIVIDER_SPACE: f32 = 13.0;
pub const DIVIDER_INSET: f32 = 14.0;
pub const DIVIDER_COLOR: Color = Color::rgba(255, 255, 255, 110);

/// Tamaño de la insignia respecto al hueco, y qué parte queda dentro de él.
pub const BADGE_SIZE: f32 = 0.44;
pub const BADGE_INSIDE: f32 = 0.85;

pub const MAXIMUM_SCALE: f32 = 1.6;
/// Iconos afectados a cada lado del cursor.
pub const MAGNIFIED_NEIGHBOURS: f32 = 3.0;
pub const BOUNCE_HEIGHT: f32 = 26.0;

pub const DOT_DIAMETER: f32 = 5.0;
pub const DOT_FROM_BOTTOM: f32 = 10.0;
pub const DOT_COLOR: Color = Color::rgba(255, 255, 255, 220);
