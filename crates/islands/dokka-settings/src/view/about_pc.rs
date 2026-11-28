//! Dibujos propios de la ficha: iMac e icono redondo de olas.
//!
//! El iMac es pantalla fina con marco claro, mentón de color y pie en cuña;
//! el icono de macOS son tres olas azules propias estilo Tahoe. Sin
//! logotipos de ninguna marca: solo formas con colores del tema.

use dokka_scene::{Drawing, Icon, Shape};
use dokka_theme::Theme;

/// iMac de pantalla fina: marco claro, pantalla oscura, mentón de color y
/// pie en cuña con base. Todo queda dentro del lienzo de 100.
pub(crate) fn pc_drawing(theme: &Theme) -> Drawing {
    let frame = theme.icons.imac_frame;
    let screen = theme.icons.imac_screen;
    let chin = theme.icons.imac_chin;
    let stand = theme.icons.imac_stand;
    Drawing {
        shapes: vec![
            // Pie en cuña detrás del monitor.
            Shape::Triangle {
                points: [(42.0, 84.0), (58.0, 84.0), (50.0, 60.0)],
                color: stand,
            },
            // Monitor: marco claro fino.
            Shape::RoundedRect {
                x: 8.0,
                y: 6.0,
                width: 84.0,
                height: 54.0,
                radius: 8.0,
                color: frame,
            },
            // Pantalla oscura arriba.
            Shape::RoundedRect {
                x: 12.0,
                y: 10.0,
                width: 76.0,
                height: 36.0,
                radius: 4.0,
                color: screen,
            },
            // Mentón de color abajo, con labio de marco.
            Shape::RoundedRect {
                x: 12.0,
                y: 46.0,
                width: 76.0,
                height: 10.0,
                radius: 3.0,
                color: chin,
            },
            // Base del pie.
            Shape::RoundedRect {
                x: 30.0,
                y: 84.0,
                width: 40.0,
                height: 6.0,
                radius: 3.0,
                color: stand,
            },
        ],
    }
}

/// Icono redondo de olas azules: tres capas propias de clara a oscura.
pub(crate) fn macos_drawing(theme: &Theme) -> Drawing {
    Drawing {
        shapes: vec![
            Shape::Circle {
                x: 50.0,
                y: 50.0,
                radius: 48.0,
                color: theme.icons.tahoe_light,
            },
            Shape::Circle {
                x: 50.0,
                y: 62.0,
                radius: 30.0,
                color: theme.icons.tahoe_mid,
            },
            Shape::Circle {
                x: 50.0,
                y: 74.0,
                radius: 22.0,
                color: theme.icons.tahoe_deep,
            },
        ],
    }
}

/// Icono oculto fuera de la vista, para no reconstruir capas.
pub(crate) fn hidden_icon(theme: &Theme) -> Icon {
    Icon {
        x: -1000.0,
        y: -1000.0,
        side: 16.0,
        image: None,
        placeholder: theme.tray.clear,
        placeholder_radius: 0.0,
    }
}
