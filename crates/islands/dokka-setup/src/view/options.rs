//! Pantalla de opciones: tarjeta con dos interruptores.

use dokka_components::switch::switch_panels;
use dokka_scene::{Panel, Rect, Scene, Text, TextAlign};
use dokka_theme::{Theme, installer as measures};

use super::super::Setup;
use super::base_scene;
use super::chrome::{primary_button, round_button};

const TITLE_TOP: f32 = 96.0;

/// Opciones: flecha de volver, tarjeta con dos filas e interruptor verde.
pub fn scene(theme: &Theme, setup: &Setup) -> Scene {
    let mut scene = base_scene(theme);
    let (round, icon) = round_button(theme, setup, true);
    scene.flat_panels.push(round);
    scene.icons.push(icon);
    scene.texts.push(Text {
        x: measures::center_x(),
        center_y: TITLE_TOP + measures::TITLE_SIZE / 2.0,
        content: "Opciones".to_owned(),
        size: 32.0,
        is_bold: true,
        color: theme.installer.text,
        align: TextAlign::Center,
    });
    scene.texts.push(Text {
        x: measures::center_x(),
        center_y: TITLE_TOP + 32.0 + 10.0 + measures::SUBTITLE_SIZE / 2.0,
        content: "Podrás cambiarlas cuando quieras.".to_owned(),
        size: measures::SUBTITLE_SIZE,
        is_bold: false,
        color: theme.installer.secondary,
        align: TextAlign::Center,
    });
    card(theme, setup, &mut scene);
    let (button, label) = primary_button(theme, setup, "Listo", false);
    scene.flat_panels.push(button);
    scene.texts.push(label);
    scene
}

/// Tarjeta translúcida con las dos filas de 58 y su interruptor.
fn card(theme: &Theme, setup: &Setup, scene: &mut Scene) {
    let (x, y, w, h) = measures::card_rect();
    scene.flat_panels.push(Panel {
        bounds: Rect {
            x,
            y,
            width: w,
            height: h,
        },
        corner_radius: measures::CARD_RADIUS,
        border_width: measures::CARD_BORDER_WIDTH,
        tint: theme.installer.card,
        border: theme.installer.card_border,
    });
    row(
        theme,
        setup,
        scene,
        0,
        "Abrir al iniciar Windows",
        setup.start_with_windows(),
    );
    row(
        theme,
        setup,
        scene,
        1,
        "Crear acceso directo",
        setup.desktop_shortcut(),
    );
    scene.dividers.push(dokka_scene::Divider {
        bounds: Rect {
            x: x + measures::ROW_PADDING,
            y: y + measures::ROW_HEIGHT,
            width: w - measures::ROW_PADDING * 2.0,
            height: 1.0,
        },
        color: theme.installer.card_border,
    });
}

/// Una fila con su etiqueta y su interruptor a la derecha.
fn row(theme: &Theme, setup: &Setup, scene: &mut Scene, index: usize, label: &str, on: bool) {
    let (x, y, w, _) = measures::card_rect();
    let row = Rect {
        x,
        y: y + index as f32 * measures::ROW_HEIGHT,
        width: w,
        height: measures::ROW_HEIGHT,
    };
    scene.texts.push(Text {
        x: x + measures::ROW_PADDING,
        center_y: row.y + row.height / 2.0,
        content: label.to_owned(),
        size: measures::ROW_TEXT_SIZE,
        is_bold: false,
        color: theme.installer.text,
        align: TextAlign::Left,
    });
    let touch = Rect {
        x: row.x + row.width - measures::ROW_PADDING - dokka_theme::settings::SWITCH_WIDTH,
        y: row.y + (row.height - dokka_theme::settings::SWITCH_HEIGHT) / 2.0,
        width: dokka_theme::settings::SWITCH_WIDTH,
        height: dokka_theme::settings::SWITCH_HEIGHT,
    };
    let props = dokka_components::switch::SwitchProps { on };
    // La pista usa el verde del instalador cuando está encendido.
    let mut panels = switch_panels(theme, touch, &props);
    if on {
        panels[0].tint = theme.installer.switch_on;
        panels[0].border = theme.installer.switch_on;
    } else {
        panels[0].tint = theme.installer.switch_off;
        panels[0].border = theme.installer.switch_off;
    }
    scene.flat_panels.extend(panels);
}
