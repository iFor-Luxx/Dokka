//! Iconos del Centro de control desde la librería Lucide, por significado.
//!
//! Cada función crea un `Drawing` con un solo glifo que llena el lienzo
//! de 100 (centrado en 50, 50). El tamaño del glifo sale de
//! `dokka-theme/src/measures/control_center.rs` y es el lienzo completo,
//! así que la fuente efectiva iguala al lado con que se muestra el icono
//! (22 en los círculos); el color lo pone quien llama (oscuro de cristal
//! o azul Tahoe sobre círculo blanco). El icono de la barra usa la medida
//! de la barra superior.

use dokka_scene::{Color, Drawing, Shape};
use dokka_theme::{control_center as measures, menubar_tray};

const CENTER: f32 = 50.0;

/// El trazo de Lucide es fino: el mismo glifo se repite apenas desplazado en
/// ocho direcciones para engrosarlo, sin dibujar iconos propios.
fn glyph(icon: lucide_icons::Icon, size: f32, color: Color) -> Drawing {
    let straight = measures::BOLD_SPREAD;
    let diagonal = straight * std::f32::consts::FRAC_1_SQRT_2;
    let offsets = [
        (0.0, 0.0),
        (straight, 0.0),
        (-straight, 0.0),
        (0.0, straight),
        (0.0, -straight),
        (diagonal, diagonal),
        (diagonal, -diagonal),
        (-diagonal, diagonal),
        (-diagonal, -diagonal),
    ];
    Drawing {
        shapes: offsets
            .into_iter()
            .map(|(dx, dy)| Shape::Glyph {
                glyph: char::from(icon),
                x: CENTER + dx,
                y: CENTER + dy,
                size,
                color,
            })
            .collect(),
    }
}

/// Icono de la barra superior: deslizadores Lucide en blanco, a 20.
pub fn control_center_bar_icon(color: Color) -> Drawing {
    // Dos interruptores apilados, como en macOS: son dos glifos de Lucide.
    let toggle = |icon: lucide_icons::Icon, y: f32| Shape::Glyph {
        glyph: char::from(icon),
        x: 50.0,
        y,
        size: menubar_tray::TOGGLE_GLYPH_SIZE,
        color,
    };
    Drawing {
        shapes: vec![
            toggle(lucide_icons::Icon::ToggleLeft, menubar_tray::TOGGLE_TOP_Y),
            toggle(
                lucide_icons::Icon::ToggleRight,
                menubar_tray::TOGGLE_BOTTOM_Y,
            ),
        ],
    }
}

/// Red wifi con conexión.
pub fn control_center_wifi_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Wifi, measures::GLYPH_SIZE, color)
}

/// Red sin conexión o sin internet.
pub fn control_center_wifi_off_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::WifiOff, measures::GLYPH_SIZE, color)
}

/// Red por cable.
pub fn control_center_ethernet_icon(color: Color) -> Drawing {
    glyph(
        lucide_icons::Icon::EthernetPort,
        measures::GLYPH_SIZE,
        color,
    )
}

/// Otra clase de red.
pub fn control_center_globe_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Globe, measures::GLYPH_SIZE, color)
}

/// Volumen con sonido.
pub fn control_center_volume_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Volume2, measures::GLYPH_SIZE, color)
}

/// Volumen silenciado.
pub fn control_center_muted_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::VolumeX, measures::GLYPH_SIZE, color)
}

/// Botón de pista anterior.
pub fn control_center_previous_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::SkipBack, measures::GLYPH_SIZE, color)
}

/// Botón de reproducir.
pub fn control_center_play_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Play, measures::GLYPH_SIZE, color)
}

/// Botón de pausa.
pub fn control_center_pause_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Pause, measures::GLYPH_SIZE, color)
}

/// Botón de pista siguiente.
pub fn control_center_next_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::SkipForward, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Preferencias de Dokka.
pub fn control_center_preferences_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Settings, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Mostrar escritorio.
pub fn control_center_desktop_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::House, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Bloquear pantalla.
pub fn control_center_lock_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Lock, measures::GLYPH_SIZE, color)
}

/// Botón redondo del Administrador de tareas.
pub fn control_center_task_manager_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Activity, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Vista de tareas.
pub fn control_center_task_view_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::LayoutGrid, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Proyectar.
pub fn control_center_project_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::MonitorUp, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Calculadora.
pub fn control_center_calculator_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Calculator, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Recorte de pantalla.
pub fn control_center_screen_clip_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Scissors, measures::GLYPH_SIZE, color)
}

/// Botón redondo de Reloj.
pub fn control_center_clock_icon(color: Color) -> Drawing {
    glyph(lucide_icons::Icon::Clock, measures::GLYPH_SIZE, color)
}
