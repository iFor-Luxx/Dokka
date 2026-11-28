//! Pantalla de trabajo: logo, título, barra fina y paso en pequeño.

use dokka_scene::{Panel, Rect, Scene, Text, TextAlign};
use dokka_theme::{Theme, installer as measures};

use super::super::{Setup, step_text};
use super::base_scene;
use super::chrome::{logo_placeholder, title_texts};

/// Instalando o desinstalando: sin botón de cerrar y misma barra.
pub fn scene(theme: &Theme, setup: &Setup, uninstall: bool) -> Scene {
    let mut scene = base_scene(theme);
    let (logo, logo_text) = logo_placeholder(theme, measures::LOGO_TOP, measures::LOGO_SIDE);
    scene.flat_panels.push(logo);
    scene.texts.push(logo_text);
    let title = if uninstall {
        "Desinstalando Dokka"
    } else {
        "Instalando Dokka"
    };
    scene
        .texts
        .extend(title_texts(theme, title, "Solo tardará un momento.", 200.0));
    bar(theme, setup, &mut scene);
    step_line(theme, setup, &mut scene);
    scene
}

/// Barra fina de 248 × 6 con el relleno según la fracción.
fn bar(theme: &Theme, setup: &Setup, scene: &mut Scene) {
    let (x, y, w, h) = measures::bar_rect();
    scene.flat_panels.push(Panel {
        bounds: Rect {
            x,
            y,
            width: w,
            height: h,
        },
        corner_radius: measures::BAR_RADIUS,
        border_width: 0.0,
        tint: theme.installer.card,
        border: theme.installer.card,
    });
    let fraction = setup.step().map_or(0.0, |(_, f)| f);
    let fill = (w * fraction).clamp(0.0, w);
    // Al menos un círculo: el radio nunca supera el ancho y Windows no rechaza la forma.
    let fill = if fill > 0.0 { fill.max(h).min(w) } else { 0.0 };
    if fill > 0.0 {
        scene.flat_panels.push(Panel {
            bounds: Rect {
                x,
                y,
                width: fill,
                height: h,
            },
            corner_radius: measures::BAR_RADIUS,
            border_width: 0.0,
            tint: theme.installer.accent,
            border: theme.installer.accent,
        });
    }
}

/// Paso en pequeño bajo la barra ("Copiando archivos…").
fn step_line(theme: &Theme, setup: &Setup, scene: &mut Scene) {
    let (_, top, _, height) = measures::bar_rect();
    let content = setup.step().map_or("Empezando…".to_owned(), |(step, _)| {
        step_text(step).to_owned()
    });
    scene.texts.push(Text {
        x: measures::center_x(),
        center_y: top + height + measures::STEP_GAP + measures::STEP_TEXT_SIZE / 2.0,
        content,
        size: measures::STEP_TEXT_SIZE,
        is_bold: false,
        color: theme.installer.secondary,
        align: TextAlign::Center,
    });
}
