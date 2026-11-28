//! Vista de la isla: una `Scene` por pantalla.

mod badge;
mod chrome;
mod error;
mod finish;
mod options;
mod progress;
mod welcome;

use dokka_scene::{Scene, Shadow};
use dokka_theme::{Theme, installer as measures};

use super::Setup;
use super::state::Screen;

/// Escena de la ventana: fondo transparente (la foto la pone la plataforma
/// bajo todo), contenido centrado y desplazado por el aire de la sombra en
/// este único sitio. El fundido al cambiar de pantalla va en `opacity`.
pub fn setup_scene(theme: &Theme, setup: &mut Setup) -> Scene {
    let mut scene = match setup.screen() {
        Screen::Welcome => welcome::scene(theme, setup),
        Screen::Options => options::scene(theme, setup),
        Screen::Installing => progress::scene(theme, setup, false),
        Screen::Done => finish::scene(theme, setup, false),
        Screen::Error => error::scene(theme, setup, false),
        Screen::UninstallConfirm => welcome::uninstall_scene(theme, setup),
        Screen::Uninstalling => progress::scene(theme, setup, true),
        Screen::UninstallDone => finish::scene(theme, setup, true),
        Screen::UninstallError => error::scene(theme, setup, true),
    };
    scene.opacity = setup.fade().clamp(0.0, 1.0);
    scene.scale = 1.0;
    scene.blur = 0.0;
    let (ox, oy) = measures::content_origin();
    shift_scene(&mut scene, ox, oy);
    scene
}

/// Fondo común: transparente para ver la foto, con borde y sombra suaves.
pub(super) fn base_scene(theme: &Theme) -> Scene {
    Scene {
        panel: dokka_scene::Panel {
            bounds: dokka_scene::Rect {
                x: 0.0,
                y: 0.0,
                width: measures::WIDTH,
                height: measures::HEIGHT,
            },
            corner_radius: measures::CORNER_RADIUS,
            border_width: measures::BORDER_WIDTH,
            tint: theme.tray.clear,
            border: theme.installer.card_border,
        },
        panel_shadow: Some(Shadow {
            color: theme.tray.shadow,
            blur: measures::SHADOW_BLUR,
            offset: (0.0, measures::SHADOW_OFFSET_Y),
        }),
        panels: Vec::new(),
        flat_panels: Vec::new(),
        highlight: None,
        dividers: Vec::new(),
        cursors: Vec::new(),
        icons: Vec::new(),
        dots: Vec::new(),
        texts: Vec::new(),
        thumbnails: Vec::new(),
        labels: Vec::new(),
        opacity: 1.0,
        scale: 1.0,
        blur: 0.0,
        header_veil: None,
        overlay: None,
        submenu: None,
    }
}

/// Desplaza toda la escena por el aire de la sombra.
fn shift_scene(scene: &mut Scene, dx: f32, dy: f32) {
    scene.panel.bounds.x += dx;
    scene.panel.bounds.y += dy;
    for panel in scene.panels.iter_mut().chain(scene.flat_panels.iter_mut()) {
        panel.bounds.x += dx;
        panel.bounds.y += dy;
    }
    if let Some(highlight) = &mut scene.highlight {
        highlight.bounds.x += dx;
        highlight.bounds.y += dy;
    }
    for divider in &mut scene.dividers {
        divider.bounds.x += dx;
        divider.bounds.y += dy;
    }
    for cursor in &mut scene.cursors {
        cursor.text_x += dx;
        cursor.max_x += dx;
        cursor.center_y += dy;
    }
    for icon in &mut scene.icons {
        icon.x += dx;
        icon.y += dy;
    }
    for dot in &mut scene.dots {
        dot.x += dx;
        dot.y += dy;
    }
    for text in &mut scene.texts {
        text.x += dx;
        text.center_y += dy;
    }
}
