//! Contenido de la ventana: tarjetas Tahoe con ajustes reales.
//!
//! Cada tarjeta es un plano claro con filas separadas por líneas finas,
//! etiqueta a la izquierda y control a la derecha. Los ayudantes de aquí
//! (deslizadores, interruptores y grupos) los usan las tarjetas del Dock y
//! del resto; la tarjeta "Próximamente" sigue para las secciones sin ajustes.

use dokka_components::settings_group::{
    SettingsGroupProps, group_dividers, group_labels, group_panel,
};
use dokka_components::slider::{SliderColors, SliderProps, slider_panels, slider_track_bounds};
use dokka_components::switch::{SwitchProps, switch_panels, switch_touch};
use dokka_scene::{Divider, Icon, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{SECTION_DOCK, SECTION_GENERAL, SECTION_MENUBAR, SECTION_TASKBAR};
use super::super::state::Settings;

/// Todo lo que pinta el contenido: planos, líneas, iconos y textos.
pub(crate) struct ContentScene {
    pub flats: Vec<Panel>,
    pub dividers: Vec<Divider>,
    pub icons: Vec<Icon>,
    pub texts: Vec<Text>,
}

/// Tarjetas con ajustes o la tarjeta gris "Próximamente".
pub(crate) fn content_scene(
    theme: &Theme,
    settings: &mut Settings,
    grip: &std::sync::Arc<dokka_scene::Drawing>,
) -> ContentScene {
    let selected = settings.selected();
    if !super::super::logic::has_content(selected) {
        return coming_soon(theme, settings.content_width());
    }
    if selected == SECTION_DOCK {
        super::dock_content::dock_content(theme, settings)
    } else if selected == SECTION_MENUBAR {
        super::menubar_content::menubar_content(theme, settings, grip)
    } else if selected == SECTION_TASKBAR {
        super::other_content::taskbar_content(theme, settings)
    } else if selected == SECTION_GENERAL {
        general_scene(theme, settings)
    } else {
        coming_soon(theme, settings.content_width())
    }
}

/// Contenido de General según su apartado: lista, ficha o próximamente.
fn general_scene(theme: &Theme, settings: &mut Settings) -> ContentScene {
    use super::super::general::GeneralPage as Page;
    match settings.general() {
        Page::Root => {
            let mut badges = Vec::new();
            let mut glyphs = Vec::new();
            for row in 0..super::super::logic::GENERAL_ROWS {
                badges.push(settings.general_badge_drawing(theme, row));
                glyphs.push(settings.general_row_drawing(theme, row));
            }
            let chevron = settings.chevron_drawing(theme);
            let gear = settings.gear_drawing(theme);
            super::general_content::general_content(
                theme, settings, &badges, &glyphs, &chevron, &gear,
            )
        }
        Page::About => {
            let pc = std::sync::Arc::new(super::about_pc::pc_drawing(theme));
            super::about_content::about_content(theme, settings, &pc)
        }
        Page::Storage => {
            let mut badges = Vec::new();
            let mut glyphs = Vec::new();
            for index in 0..super::super::storage_state::STORAGE_CATEGORIES {
                badges.push(settings.storage_badge_drawing(theme, index));
                glyphs.push(settings.storage_row_drawing(theme, index));
            }
            super::storage_content::storage_content(theme, settings, &badges, &glyphs)
        }
        Page::Update => {
            let badge = settings.general_badge_drawing(theme, 1);
            let glyph = settings.general_row_drawing(theme, 1);
            super::update_content::update_content(theme, settings, &badge, &glyph)
        }
        _ => coming_soon(theme, settings.content_width()),
    }
}

/// Tarjeta gris centrada de las secciones todavía sin ajustes.
pub(crate) fn coming_soon(theme: &Theme, width: f32) -> ContentScene {
    let panel = Panel {
        bounds: Rect {
            x: measures::CONTENT_LEFT,
            y: measures::CARD_TOP,
            width,
            height: measures::CARD_HEIGHT,
        },
        corner_radius: measures::CARD_RADIUS,
        border_width: measures::CARD_BORDER_WIDTH,
        tint: theme.settings.card,
        border: theme.settings.card_border,
    };
    let text = Text {
        x: measures::CONTENT_LEFT + width / 2.0,
        center_y: measures::CARD_TOP + measures::CARD_HEIGHT / 2.0,
        content: measures::CARD_TEXT.to_owned(),
        size: measures::CARD_TEXT_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Center,
    };
    ContentScene {
        flats: vec![panel],
        dividers: Vec::new(),
        icons: Vec::new(),
        texts: vec![text],
    }
}

/// Colores del deslizador de Preferencias, claros y neutros como la tarjeta.
pub(crate) fn slider_colors(theme: &Theme) -> SliderColors {
    SliderColors {
        track: theme.settings.slider_track,
        fill: theme.settings.slider_fill,
        muted: theme.settings.slider_muted,
        tick: theme.settings.slider_tick,
        knob: theme.settings.switch_knob,
        knob_border: theme.settings.card_border,
    }
}

/// "Pequeño" bajo el extremo izquierdo y "Grande" bajo el derecho.
fn slider_captions(theme: &Theme, area: Rect, track: Rect) -> Vec<Text> {
    let middle =
        track.y + track.height + measures::SLIDER_CAPTION_GAP + measures::SLIDER_CAPTION_SIZE / 2.0;
    vec![
        Text {
            x: area.x,
            center_y: middle,
            content: "Pequeño".to_owned(),
            size: measures::SLIDER_CAPTION_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Left,
        },
        Text {
            x: area.x + area.width - measures::SLIDER_CAPTION_RIGHT_WIDTH,
            center_y: middle,
            content: "Grande".to_owned(),
            size: measures::SLIDER_CAPTION_SIZE,
            is_bold: false,
            color: theme.settings.card_text,
            align: TextAlign::Left,
        },
    ]
}

/// Planos de un deslizador en su fila, con sus textos bajo la barra. El
/// relleno va desde la fábrica hasta el valor, con su marca.
pub(crate) fn slider_pieces(
    theme: &Theme,
    top: f32,
    heights: &[f32],
    index: usize,
    props: &SliderProps,
    width: f32,
) -> (Vec<Panel>, Vec<Text>) {
    let area = super::super::logic::content_slider_area(top, heights, index, width);
    let track = slider_track_bounds(area);
    let panels = slider_panels(track, props, &slider_colors(theme));
    let captions = slider_captions(theme, area, track);
    (panels.into_iter().collect(), captions)
}

/// Planos de un interruptor en su fila: pista y tirador en cápsula.
pub(crate) fn switch_pieces(
    theme: &Theme,
    top: f32,
    heights: &[f32],
    index: usize,
    on: bool,
    width: f32,
) -> Vec<Panel> {
    let row = super::super::logic::content_row(top, heights, index, width);
    let touch = switch_touch(row);
    let props = SwitchProps { on };
    switch_panels(theme, touch, &props).into_iter().collect()
}

/// Etiquetas, líneas y tarjeta de un grupo que empieza en `top`.
pub(crate) fn group_pieces(
    theme: &Theme,
    top: f32,
    labels: &[&'static str],
    heights: &[f32],
    dim: &[bool],
    width: f32,
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let owned: Vec<String> = labels.iter().map(|label| label.to_string()).collect();
    let props = SettingsGroupProps {
        top,
        rows: owned,
        heights: heights.to_vec(),
        width,
    };
    let panel = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);
    let texts = group_labels(theme, &props, dim);
    (panel, dividers, texts)
}
