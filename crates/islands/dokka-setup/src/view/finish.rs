//! Pantallas finales: círculo verde con check y botón.

use dokka_scene::Scene;
use dokka_theme::Theme;

use super::super::Setup;
use super::badge::badge_scene;
use super::base_scene;
use super::chrome::{primary_button, title_texts};

const TITLE_TOP: f32 = 200.0;

/// Todo listo o desinstalado: check y un solo botón.
pub(super) fn scene(theme: &Theme, setup: &Setup, uninstall: bool) -> Scene {
    let mut scene = base_scene(theme);
    badge_scene(theme, &mut scene);
    if uninstall {
        scene.texts.extend(title_texts(
            theme,
            "Dokka se desinstaló",
            "Los últimos archivos se borrarán al reiniciar.",
            TITLE_TOP,
        ));
        let (button, label) = primary_button(theme, setup, "Cerrar", false);
        scene.flat_panels.push(button);
        scene.texts.push(label);
    } else {
        scene.texts.extend(title_texts(
            theme,
            "Todo listo",
            "Dokka ya está en tu PC.",
            TITLE_TOP,
        ));
        let (button, label) = primary_button(theme, setup, "Abrir Dokka", false);
        scene.flat_panels.push(button);
        scene.texts.push(label);
    }
    scene
}
