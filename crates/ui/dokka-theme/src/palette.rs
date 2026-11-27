//! Colores base. Es el ÚNICO lugar de Dokka con valores de color.

use dokka_scene::Color;

pub(crate) const fn white(alpha: u8) -> Color {
    Color::rgba(255, 255, 255, alpha)
}

pub(crate) const SNOW: Color = Color::rgba(250, 250, 250, 255);
pub(crate) const INK: Color = Color::rgba(30, 32, 36, 255);
pub(crate) const CHARCOAL: Color = Color::rgba(40, 40, 44, 255);
pub(crate) const MIST: Color = Color::rgba(236, 240, 236, 170);
pub(crate) const MIST_STRONG: Color = Color::rgba(240, 244, 240, 105);
/// Tinte neutro translúcido de la etiqueta flotante, como el resto de Dokka.
pub(crate) const TOOLTIP_TINT: Color = Color::rgba(232, 238, 232, 175);
/// Borde luminoso de la etiqueta flotante.
pub(crate) const TOOLTIP_BORDER: Color = Color::rgba(255, 255, 255, 160);

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

/// Cápsula de navegación: gris muy claro.
pub(crate) const NAV_PILL_BACKGROUND: Color = Color::rgba(230, 232, 238, 235);
/// Borde fino y luminoso de la cápsula de navegación.
pub(crate) const NAV_PILL_BORDER: Color = white(180);
/// Línea vertical del centro de la píldora.
pub(crate) const NAV_PILL_DIVIDER: Color = Color::rgba(0, 0, 0, 35);
/// Chevrón no disponible de la píldora.
pub(crate) const NAV_PILL_ICON_DISABLED: Color = Color::rgba(30, 32, 36, 110);
/// Resaltado suave de la mitad bajo el cursor.
pub(crate) const NAV_PILL_HOVER: Color = Color::rgba(0, 0, 0, 25);

/// Fondo del Launchpad: velo negro muy leve que deja ver el escritorio.
pub(crate) const LAUNCHPAD_BACKGROUND: Color = Color::rgba(0, 0, 0, 50);
/// Baldosa clara donde apoya el glifo de cada app en el Launchpad.
pub(crate) const APP_TILE: Color = Color::rgba(231, 234, 239, 255);
/// Texto claro de los nombres bajo los iconos.
pub(crate) const LAUNCHPAD_TEXT: Color = Color::rgba(250, 250, 250, 255);
/// Resaltado suave detrás de la app bajo el cursor.
pub(crate) const LAUNCHPAD_HIGHLIGHT: Color = Color::rgba(255, 255, 255, 38);
/// Casilla de búsqueda: fondo claro apenas visible.
pub(crate) const LAUNCHPAD_SEARCH_BACKGROUND: Color = Color::rgba(255, 255, 255, 30);
/// Borde fino blanco translúcido de la casilla.
pub(crate) const LAUNCHPAD_SEARCH_BORDER: Color = Color::rgba(255, 255, 255, 110);
/// Texto y lupa de la casilla, claros para leerse sobre el fondo vivo.
pub(crate) const LAUNCHPAD_SEARCH_TEXT: Color = Color::rgba(255, 255, 255, 190);
/// Marcador "Buscar" junto al cursor: el mismo blanco, más tenue.
pub(crate) const LAUNCHPAD_SEARCH_PLACEHOLDER: Color = Color::rgba(255, 255, 255, 120);
/// Punto de la página actual: blanco.
pub(crate) const LAUNCHPAD_DOT_ACTIVE: Color = Color::rgba(255, 255, 255, 255);
/// Fondo claro de la ventana de Preferencias.
pub(crate) const SETTINGS_WINDOW: Color = Color::rgba(246, 246, 248, 255);
/// Barra lateral translúcida sobre la ventana.
pub(crate) const SETTINGS_SIDEBAR: Color = Color::rgba(232, 232, 238, 190);
/// Borde fino y luminoso del panel flotante de la barra lateral.
pub(crate) const SETTINGS_SIDEBAR_BORDER: Color = white(170);
/// Casilla de búsqueda: gris apenas visible.
pub(crate) const SETTINGS_SEARCH_BACKGROUND: Color = Color::rgba(0, 0, 0, 18);
/// Borde fino de la casilla de búsqueda.
pub(crate) const SETTINGS_SEARCH_BORDER: Color = Color::rgba(0, 0, 0, 30);
/// Resaltado suave al pasar sobre una fila.
pub(crate) const SETTINGS_HOVER: Color = Color::rgba(0, 0, 0, 22);
/// Tarjeta gris muy suave del contenido, todavía sin ajustes.
pub(crate) const SETTINGS_CARD: Color = Color::rgba(0, 0, 0, 12);
/// Línea bajo la cabecera del contenido.
pub(crate) const SETTINGS_DIVIDER: Color = Color::rgba(0, 0, 0, 28);
/// Semáforos apagados cuando Preferencias no es la ventana activa.
pub(crate) const SETTINGS_TRAFFIC_INACTIVE: Color = Color::rgba(206, 206, 210, 255);
/// Símbolo oscuro sobre el semáforo rojo al pasar el cursor.
pub(crate) const TRAFFIC_CLOSE_SYMBOL: Color = Color::rgba(143, 29, 29, 255);
/// Símbolo oscuro sobre el semáforo amarillo al pasar el cursor.
pub(crate) const TRAFFIC_MINIMIZE_SYMBOL: Color = Color::rgba(155, 98, 10, 255);
/// Símbolo oscuro sobre el semáforo verde al pasar el cursor.
pub(crate) const TRAFFIC_ZOOM_SYMBOL: Color = Color::rgba(23, 125, 43, 255);
/// Puntos de las demás páginas.
pub(crate) const LAUNCHPAD_DOT_INACTIVE: Color = Color::rgba(255, 255, 255, 110);
