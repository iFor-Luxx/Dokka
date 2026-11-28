//! Pantalla de error: círculo naranja, frase del fallo y reintento.

use dokka_scene::{Scene, Text, TextAlign};
use dokka_theme::{Theme, installer as measures};

use super::super::Setup;
use super::badge::warn_scene;
use super::base_scene;
use super::chrome::{link_text, primary_button, round_button};

const TITLE_TOP: f32 = 200.0;

/// Error al instalar o desinstalar, con su frase y botón Reintentar.
pub fn scene(theme: &Theme, setup: &Setup, uninstall: bool) -> Scene {
    let mut scene = base_scene(theme);
    warn_scene(theme, &mut scene);
    let title = if setup.error_title().is_empty() {
        if uninstall {
            "No se pudo desinstalar"
        } else {
            "No se pudo instalar"
        }
    } else {
        setup.error_title()
    };
    scene.texts.push(Text {
        x: measures::center_x(),
        center_y: TITLE_TOP + measures::TITLE_SIZE / 2.0,
        content: title.to_owned(),
        size: 36.0,
        is_bold: true,
        color: theme.installer.text,
        align: TextAlign::Center,
    });
    scene.texts.push(Text {
        x: measures::center_x(),
        center_y: TITLE_TOP + 36.0 + 10.0 + measures::SUBTITLE_SIZE / 2.0,
        content: setup.error_detail().to_owned(),
        size: 15.0,
        is_bold: false,
        color: theme.installer.secondary,
        align: TextAlign::Center,
    });
    let (round, icon) = round_button(theme, setup, false);
    scene.flat_panels.push(round);
    scene.icons.push(icon);
    let (button, label) = primary_button(theme, setup, "Reintentar", false);
    scene.flat_panels.push(button);
    scene.texts.push(label);
    scene.texts.push(link_text(theme, "Salir"));
    scene
}
