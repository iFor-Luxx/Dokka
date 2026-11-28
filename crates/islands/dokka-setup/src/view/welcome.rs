//! Bienvenida y confirmación de desinstalar.

use dokka_scene::Scene;
use dokka_theme::{Theme, installer as measures};

use super::super::Setup;
use super::base_scene;
use super::chrome::{link_text, logo_placeholder, primary_button, round_button, title_texts};

const TITLE_TOP: f32 = 200.0;

/// Bienvenida: logo, título, botón Instalar y enlace Opciones.
pub fn scene(theme: &Theme, setup: &Setup) -> Scene {
    let mut scene = base_scene(theme);
    let (logo, logo_text) = logo_placeholder(theme, measures::LOGO_TOP, measures::LOGO_SIDE);
    scene.flat_panels.push(logo);
    scene.texts.push(logo_text);
    scene.texts.extend(title_texts(
        theme,
        "Dokka",
        "Tu escritorio, a tu manera.",
        TITLE_TOP,
    ));
    let (round, round_icon) = round_button(theme, setup, false);
    scene.flat_panels.push(round);
    scene.icons.push(round_icon);
    let (button, label) = primary_button(theme, setup, "Instalar", false);
    scene.flat_panels.push(button);
    scene.texts.push(label);
    scene.texts.push(link_text(theme, "Opciones"));
    scene
}

/// Confirmación de desinstalar: logo menor, casilla y botón rojo.
pub fn uninstall_scene(theme: &Theme, setup: &Setup) -> Scene {
    let mut scene = base_scene(theme);
    let (logo, logo_text) = logo_placeholder(theme, 70.0, measures::UNINSTALL_LOGO_SIDE);
    scene.flat_panels.push(logo);
    scene.texts.push(logo_text);
    scene.texts.extend(title_texts(
        theme,
        "¿Desinstalar Dokka?",
        "Windows volverá a verse como antes.",
        178.0,
    ));
    check_row(theme, setup, &mut scene);
    let (button, label) = primary_button(theme, setup, "Desinstalar", true);
    scene.flat_panels.push(button);
    scene.texts.push(label);
    scene.texts.push(link_text(theme, "Cancelar"));
    scene
}

/// Fila centrada con la casilla de "Borrar también mis ajustes".
fn check_row(theme: &Theme, setup: &Setup, scene: &mut Scene) {
    let on = setup.remove_settings();
    let y = measures::CHECK_TOP;
    let half = 150.0;
    let box_side = measures::CHECK_SIDE;
    let box_x = measures::center_x() - half;
    scene.flat_panels.push(dokka_scene::Panel {
        bounds: dokka_scene::Rect {
            x: box_x,
            y: y + (measures::CHECK_TEXT_SIZE - box_side) / 2.0,
            width: box_side,
            height: box_side,
        },
        corner_radius: measures::CHECK_RADIUS,
        border_width: 1.0,
        tint: theme.installer.card,
        border: theme.installer.card_border,
    });
    if on {
        let check = super::chrome::check_drawing(theme.installer.accent);
        scene.icons.push(dokka_scene::Icon {
            x: box_x + 2.0,
            y: y + (measures::CHECK_TEXT_SIZE - box_side) / 2.0 + 2.0,
            side: box_side - 4.0,
            image: Some(dokka_scene::IconImage::Drawing(check)),
            placeholder: theme.tray.clear,
            placeholder_radius: 0.0,
        });
    }
    scene.texts.push(dokka_scene::Text {
        x: box_x + box_side + measures::CHECK_GAP,
        center_y: y + measures::CHECK_TEXT_SIZE / 2.0,
        content: "Borrar también mis ajustes".to_owned(),
        size: measures::CHECK_TEXT_SIZE,
        is_bold: false,
        color: theme.installer.text,
        align: dokka_scene::TextAlign::Left,
    });
}
