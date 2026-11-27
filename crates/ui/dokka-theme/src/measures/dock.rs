//! Medidas del dock.

/// Distancia entre el dock y el borde inferior del área de trabajo.
pub const DOCK_BOTTOM_MARGIN: f32 = 8.0;

/// Margen con el borde en vertical: el mismo que abajo.
pub const DOCK_EDGE_MARGIN: f32 = DOCK_BOTTOM_MARGIN;

/// Cápsula que queda al borde cuando el dock se oculta con una app
/// a pantalla completa: como el indicador de inicio del iPhone.
pub const HOME_LINE_LENGTH: f32 = 280.0;
pub const HOME_LINE_THICKNESS: f32 = 10.0;
/// Separación entre la línea y el borde de la pantalla.
pub const HOME_LINE_EDGE_GAP: f32 = 10.0;
/// Zona de toque de la línea: 320 a lo largo y desde el borde hasta
/// 24 hacia dentro, para llegar sin puntería fina hasta el píxel 0.
pub const HOME_LINE_TOUCH_LENGTH: f32 = 380.0;
pub const HOME_LINE_TOUCH_THICKNESS: f32 = 24.0;
/// Espera corta antes de volver a bajar tras salir del dock.
pub const HIDE_DELAY_SECONDS: f32 = 0.4;
/// Cuánto tarda en deslizarse fuera y en volver.
pub const HIDE_MOVE_SECONDS: f32 = 0.25;
pub const SHOW_MOVE_SECONDS: f32 = 0.18;

pub const HEIGHT: f32 = 80.0;
pub const SLOT: f32 = 52.0;
pub const GAP: f32 = 12.0;
pub const PADDING: f32 = 14.0;
pub const CORNER_RADIUS: f32 = 24.0;
pub const BORDER_WIDTH: f32 = 1.0;
pub const PLACEHOLDER_RADIUS: f32 = 12.0;

/// Ancho extra que ocupa la línea separadora y su aire.
pub const DIVIDER_SPACE: f32 = 29.0;
pub const DIVIDER_INSET: f32 = 14.0;
pub const DIVIDER_WIDTH: f32 = 1.0;

/// Tamaño de la insignia respecto al hueco, y qué parte queda dentro de él.
pub const BADGE_SIZE: f32 = 0.44;
pub const BADGE_INSIDE: f32 = 0.85;

pub const MAXIMUM_SCALE: f32 = 1.6;
/// Iconos afectados a cada lado del cursor.
pub const MAGNIFIED_NEIGHBOURS: f32 = 3.0;
pub const BOUNCE_HEIGHT: f32 = 26.0;

pub const DOT_DIAMETER: f32 = 5.0;
pub const DOT_FROM_BOTTOM: f32 = 10.0;
