//! Iconos de interfaz desde la librería Lucide, por significado.
//!
//! Cada función crea un `Drawing` con un solo glifo que llena el lienzo
//! de 100 (centrado en 50, 50). El tamaño del glifo sale de `dokka-theme`
//! y es el lienzo completo, así que la fuente efectiva iguala al lado con
//! que se muestra el icono (24); el color lo pone quien llama
//! (texto normal o blanco de la fila resaltada).

use dokka_scene::{Color, DRAWING_SIDE, Drawing, Shape};
use dokka_theme::{launchpad, menu, menubar, nav_pill, popup, settings};

const CENTER: f32 = 50.0;

fn glyph(icon: lucide_icons::Icon, size: f32, color: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Glyph {
            glyph: char::from(icon),
            x: CENTER,
            y: CENTER,
            size,
            color,
        }],
    }
}

/// Chevrón de las filas con submenú.
pub fn submenu_chevron_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::ChevronRight,
        menu::CHEVRON_GLYPH_SIZE,
        color,
    )
}

/// Marca de Dokka en la barra superior: icono Lucide a 20, sin dibujo propio.
pub fn menubar_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Dock, menubar::GLYPH_SIZE, color)
}

/// Marca de la opción elegida.
pub fn checkmark_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Check, menu::CHECK_GLYPH_SIZE, color)
}

/// Chevrón izquierdo de la píldora de navegación.
pub fn nav_previous_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::ChevronLeft, nav_pill::GLYPH_SIZE, color)
}

/// Chevrón derecho de la píldora de navegación.
pub fn nav_next_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::ChevronRight,
        nav_pill::GLYPH_SIZE,
        color,
    )
}

/// Lupa de la casilla de búsqueda del Launchpad.
pub fn search_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Search,
        launchpad::SEARCH_GLYPH_SIZE,
        color,
    )
}

/// Círculo liso de un semáforo, sin símbolo: en color con foco y en gris
/// sin él. Llena el lienzo para pintarse nítido a su diámetro real.
pub fn traffic_circle(circle: Color) -> Drawing {
    Drawing {
        shapes: vec![Shape::Circle {
            x: CENTER,
            y: CENTER,
            radius: CENTER,
            color: circle,
        }],
    }
}

/// Semáforo completo: círculo con su símbolo oscuro centrado al pasar el
/// cursor. El trazo de Lucide es muy fino a este tamaño: el mismo glifo se
/// repite apenas desplazado en cruz para engrosarlo sin dibujar uno propio.
fn traffic_button(circle: Color, icon: lucide_icons::Icon, symbol: Color) -> Drawing {
    let to_canvas = DRAWING_SIDE / settings::TRAFFIC_DIAMETER;
    let spread = settings::TRAFFIC_GLYPH_SPREAD * to_canvas;
    let glyphs = [
        (0.0, 0.0),
        (-spread, 0.0),
        (spread, 0.0),
        (0.0, -spread),
        (0.0, spread),
    ]
    .into_iter()
    .map(|(dx, dy)| Shape::Glyph {
        glyph: char::from(icon),
        x: CENTER + dx,
        y: CENTER + dy,
        size: settings::TRAFFIC_GLYPH_SIDE * to_canvas,
        color: symbol,
    });
    let mut shapes = vec![Shape::Circle {
        x: CENTER,
        y: CENTER,
        radius: CENTER,
        color: circle,
    }];
    shapes.extend(glyphs);
    Drawing { shapes }
}

/// Círculo rojo con su × oscura al pasar el cursor.
pub fn traffic_close_button(circle: Color, symbol: Color) -> Drawing {
    traffic_button(circle, lucide_icons::Icon::X, symbol)
}

/// Círculo amarillo con su − oscura al pasar el cursor.
pub fn traffic_minimize_button(circle: Color, symbol: Color) -> Drawing {
    traffic_button(circle, lucide_icons::Icon::Minus, symbol)
}

/// Círculo verde con sus flechas diagonales (pantalla completa) al pasar el cursor.
pub fn traffic_zoom_button(circle: Color, symbol: Color) -> Drawing {
    traffic_button(circle, lucide_icons::Icon::Maximize2, symbol)
}

/// Icono blanco de la sección General sobre su cuadrado de color.
pub fn settings_general_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Settings,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Dock sobre su cuadrado de color.
pub fn settings_dock_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Dock, settings::ROW_GLYPH_SIZE, color)
}

/// Icono blanco de la sección Launchpad sobre su cuadrado de color.
pub fn settings_launchpad_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::LayoutGrid,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Barra superior sobre su cuadrado de color.
pub fn settings_menubar_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::PanelTop,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Apariencia sobre su cuadrado de color.
pub fn settings_appearance_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Palette, settings::ROW_GLYPH_SIZE, color)
}

/// Icono blanco de la sección Barra de tareas sobre su cuadrado de color.
pub fn settings_taskbar_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::PanelBottom,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Atajos sobre su cuadrado de color.
pub fn settings_shortcuts_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::Keyboard,
        settings::ROW_GLYPH_SIZE,
        color,
    )
}

/// Icono blanco de la sección Acerca de sobre su cuadrado de color.
pub fn settings_about_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Info, settings::ROW_GLYPH_SIZE, color)
}

/// Agarre de las filas reordenables de la barra superior: seis puntos
/// verticales en color secundario, a tamaño de la fila.
pub fn settings_grip_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::GripVertical,
        settings::REORDER_GRIP_GLYPH_SIZE,
        color,
    )
}

/// Botón de pista anterior.
pub fn media_previous_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::SkipBack, popup::MEDIA_GLYPH_SIZE, color)
}

/// Botón de reproducir.
pub fn media_play_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Play, popup::MEDIA_GLYPH_SIZE, color)
}

/// Botón de pausa.
pub fn media_pause_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Pause, popup::MEDIA_GLYPH_SIZE, color)
}

/// Botón de pista siguiente.
pub fn media_next_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::SkipForward,
        popup::MEDIA_GLYPH_SIZE,
        color,
    )
}

/// Botón de cerrar la ventana (X).
pub fn preview_close_icon(glyph_color: Color, background: Color, border: Color) -> Drawing {
    window_button(lucide_icons::Icon::X, glyph_color, background, border)
}

/// Botón de fijar la ventana (chincheta).
pub fn preview_pin_icon(glyph_color: Color, background: Color, border: Color) -> Drawing {
    window_button(lucide_icons::Icon::Pin, glyph_color, background, border)
}

/// Botón de ventana fijada: misma chincheta tachada para desfijar.
pub fn preview_pinned_icon(glyph_color: Color, background: Color, border: Color) -> Drawing {
    window_button(lucide_icons::Icon::PinOff, glyph_color, background, border)
}

/// Círculo claro con borde y glifo centrado de 20 en un botón de 28.
fn window_button(
    icon: lucide_icons::Icon,
    glyph_color: Color,
    background: Color,
    border: Color,
) -> Drawing {
    Drawing {
        shapes: vec![
            Shape::Circle {
                x: CENTER,
                y: CENTER,
                radius: CENTER,
                color: border,
            },
            Shape::Circle {
                x: CENTER,
                y: CENTER,
                radius: CENTER - 6.0,
                color: background,
            },
            Shape::Glyph {
                glyph: char::from(icon),
                x: CENTER,
                y: CENTER,
                size: popup::PREVIEW_BUTTON_GLYPH_SIZE,
                color: glyph_color,
            },
        ],
    }
}
