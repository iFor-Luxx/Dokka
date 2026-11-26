//! Colores base. Es el ÚNICO lugar de Dokka con valores de color.

use dokka_scene::Color;

pub(crate) const fn white(alpha: u8) -> Color {
    Color::rgba(255, 255, 255, alpha)
}

pub(crate) const SNOW: Color = Color::rgba(250, 250, 250, 255);
pub(crate) const INK: Color = Color::rgba(30, 32, 36, 255);
pub(crate) const CHARCOAL: Color = Color::rgba(40, 40, 44, 255);
pub(crate) const MIST: Color = Color::rgba(236, 240, 236, 170);
pub(crate) const MIST_SOLID: Color = Color::rgba(232, 238, 232, 235);
pub(crate) const MIST_STRONG: Color = Color::rgba(240, 244, 240, 105);

pub(crate) const RED: Color = Color::rgba(235, 70, 70, 255);
pub(crate) const ORANGE: Color = Color::rgba(250, 140, 40, 255);
pub(crate) const YELLOW: Color = Color::rgba(250, 200, 40, 255);
pub(crate) const GREEN: Color = Color::rgba(76, 201, 90, 255);
pub(crate) const TEAL: Color = Color::rgba(40, 200, 200, 255);
pub(crate) const BLUE: Color = Color::rgba(32, 150, 243, 255);
pub(crate) const SKY: Color = Color::rgba(188, 224, 252, 255);
pub(crate) const PURPLE: Color = Color::rgba(160, 90, 220, 255);
pub(crate) const PINK: Color = Color::rgba(240, 80, 130, 255);

pub(crate) const SILVER: Color = Color::rgba(226, 229, 234, 255);
pub(crate) const SILVER_SOFT: Color = Color::rgba(222, 226, 232, 235);
pub(crate) const STEEL: Color = Color::rgba(160, 166, 176, 255);
pub(crate) const STEEL_SOFT: Color = Color::rgba(150, 156, 168, 200);
pub(crate) const GRAY: Color = Color::rgba(140, 145, 155, 255);

/// Azul de selección tipo macOS para el menú.
pub(crate) const MACOS_BLUE: Color = Color::rgba(10, 100, 215, 255);

/// Fondo translúcido del menú del clic derecho, como el calendario.
pub(crate) const MENU_BACKGROUND: Color = Color::rgba(242, 242, 242, 175);
/// Borde sutil del menú del clic derecho.
pub(crate) const MENU_BORDER: Color = Color::rgba(0, 0, 0, 38);
/// Línea separadora del menú del clic derecho.
pub(crate) const MENU_SEPARATOR: Color = Color::rgba(0, 0, 0, 35);
