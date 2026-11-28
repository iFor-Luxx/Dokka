//! Insignia de estado: círculo verde con check o naranja con aviso.

use std::sync::Arc;

use dokka_scene::{Drawing, Scene, Shape};
use dokka_theme::Theme;

use super::chrome::{badge, check_drawing};

/// Círculo verde con check dibujado.
pub(super) fn badge_scene(theme: &Theme, scene: &mut Scene) {
    let drawing = check_drawing(theme.menu.highlighted_text);
    let (dot, icon) = badge(theme, theme.installer.switch_on, drawing);
    scene.dots.push(dot);
    scene.icons.push(icon);
}

/// Círculo naranja con "!" dibujado (raya y punto, nunca texto).
pub(super) fn warn_scene(theme: &Theme, scene: &mut Scene) {
    let white = theme.menu.highlighted_text;
    let drawing = Arc::new(Drawing {
        shapes: vec![
            Shape::Line {
                from: (50.0, 28.0),
                to: (50.0, 58.0),
                width: 10.0,
                color: white,
            },
            Shape::Circle {
                x: 50.0,
                y: 74.0,
                radius: 6.0,
                color: white,
            },
        ],
    });
    // Naranja de la insignia de ajustes, mismo tono cálido.
    let (dot, icon) = badge(theme, theme.settings.badge[6], drawing);
    scene.dots.push(dot);
    scene.icons.push(icon);
}
