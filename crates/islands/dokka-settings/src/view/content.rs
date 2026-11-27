//! Contenido de la ventana: tarjetas Tahoe con ajustes reales.
//!
//! Cada tarjeta es un plano claro con filas separadas por líneas finas,
//! etiqueta a la izquierda y control a la derecha. Los deslizadores usan el
//! componente `slider` (pista fina, relleno y tirador en cápsula, con
//! "Pequeño" y "Grande" debajo) y los interruptores el componente `switch`
//! (pista con tirador alargado). Todo son planos: el estilo y la cantidad no
//! cambian al arrastrar, así `SceneLayers` solo mueve sin reconstruir. La
//! tarjeta "Próximamente" sigue para las secciones sin ajustes.

use dokka_components::settings_group::{
    SettingsGroupProps, group_dividers, group_labels, group_panel,
};
use dokka_components::slider::{SliderColors, SliderProps, slider_panels, slider_track_bounds};
use dokka_components::switch::{SwitchProps, switch_panels, switch_touch};
use dokka_scene::{Divider, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{
    SECTION_DOCK, SECTION_MENUBAR, SECTION_TASKBAR, card_heights, card_tops, content_row,
    content_slider_area, icon_scale_to, magnification_to, section_cards,
};
use super::super::pointer::display_preferences;
use super::super::state::Settings;

/// Todo lo que pinta el contenido: planos, líneas y textos, sin puntos.
pub(crate) struct ContentScene {
    pub flats: Vec<Panel>,
    pub dividers: Vec<Divider>,
    pub texts: Vec<Text>,
}

/// Tarjetas con ajustes o la tarjeta gris "Próximamente".
pub(crate) fn content_scene(theme: &Theme, settings: &Settings) -> ContentScene {
    let selected = settings.selected();
    if !super::super::logic::has_content(selected) {
        return coming_soon(theme);
    }
    if selected == SECTION_DOCK {
        dock_content(theme, settings)
    } else if selected == SECTION_MENUBAR {
        menubar_content(theme, settings)
    } else if selected == SECTION_TASKBAR {
        taskbar_content(theme, settings)
    } else {
        coming_soon(theme)
    }
}

/// Tarjeta gris centrada de las secciones todavía sin ajustes.
fn coming_soon(theme: &Theme) -> ContentScene {
    let (card_x, card_top, card_width) = measures::card_bounds();
    let panel = Panel {
        bounds: Rect {
            x: card_x,
            y: card_top,
            width: card_width,
            height: measures::CARD_HEIGHT,
        },
        corner_radius: measures::CARD_RADIUS,
        border_width: measures::CARD_BORDER_WIDTH,
        tint: theme.settings.card,
        border: theme.settings.card_border,
    };
    let text = Text {
        x: card_x + card_width / 2.0,
        center_y: card_top + measures::CARD_HEIGHT / 2.0,
        content: measures::CARD_TEXT.to_owned(),
        size: measures::CARD_TEXT_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Center,
    };
    ContentScene {
        flats: vec![panel],
        dividers: Vec::new(),
        texts: vec![text],
    }
}

/// Colores del deslizador de Preferencias, claros y neutros como la tarjeta.
fn slider_colors(theme: &Theme) -> SliderColors {
    SliderColors {
        track: theme.settings.slider_track,
        fill: theme.settings.slider_fill,
        muted: theme.settings.slider_muted,
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

/// Planos de un deslizador en su fila, con sus textos bajo la barra.
fn slider_pieces(
    theme: &Theme,
    top: f32,
    heights: &[f32],
    index: usize,
    value: f32,
    active: bool,
) -> (Vec<Panel>, Vec<Text>) {
    let area = content_slider_area(top, heights, index);
    let track = slider_track_bounds(area);
    let props = SliderProps { value, active };
    let panels = slider_panels(track, &props, &slider_colors(theme));
    let captions = slider_captions(theme, area, track);
    (panels.into_iter().collect(), captions)
}

/// Planos de un interruptor en su fila: pista y tirador en cápsula.
fn switch_pieces(theme: &Theme, top: f32, heights: &[f32], index: usize, on: bool) -> Vec<Panel> {
    let row = content_row(top, heights, index);
    let touch = switch_touch(row);
    let props = SwitchProps { on };
    switch_panels(theme, touch, &props).into_iter().collect()
}

/// Etiquetas, líneas y tarjeta de un grupo que empieza en `top`.
fn group_pieces(
    theme: &Theme,
    top: f32,
    labels: &[&'static str],
    heights: &[f32],
    dim: &[bool],
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let owned: Vec<String> = labels.iter().map(|label| label.to_string()).collect();
    let props = SettingsGroupProps {
        top,
        rows: owned,
        heights: heights.to_vec(),
    };
    let panel = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);
    let texts = group_labels(theme, &props, dim);
    (panel, dividers, texts)
}

/// Contenido del Dock: tamaño, ampliación y cantidad en una tarjeta y las
/// vistas previas en otra.
fn dock_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let shown = display_preferences(settings.values());
    let tops = card_tops(SECTION_DOCK);
    let cards = section_cards(SECTION_DOCK);
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut texts = Vec::new();
    let Some(first_top) = tops.first().copied() else {
        return coming_soon(theme);
    };
    let Some(first_labels) = cards.first() else {
        return coming_soon(theme);
    };
    let first_heights = card_heights(SECTION_DOCK, 0);
    let strength_dim = !shown.dock.magnification;
    let dim = vec![false, false, strength_dim];
    let (panel, mut card_dividers, mut card_texts) =
        group_pieces(theme, first_top, first_labels, &first_heights, &dim);
    flats.push(panel);
    dividers.append(&mut card_dividers);
    texts.append(&mut card_texts);
    let (mut panels, mut captions) = slider_pieces(
        theme,
        first_top,
        &first_heights,
        0,
        icon_scale_to(shown.dock.icon_scale),
        true,
    );
    flats.append(&mut panels);
    texts.append(&mut captions);
    flats.append(&mut switch_pieces(
        theme,
        first_top,
        &first_heights,
        1,
        shown.dock.magnification,
    ));
    let (mut panels, mut captions) = slider_pieces(
        theme,
        first_top,
        &first_heights,
        2,
        magnification_to(shown.dock.magnification_strength),
        !strength_dim,
    );
    flats.append(&mut panels);
    texts.append(&mut captions);
    if let (Some(second_top), Some(second_labels)) = (tops.get(1).copied(), cards.get(1)) {
        let second_heights = card_heights(SECTION_DOCK, 1);
        let (panel, mut card_dividers, mut card_texts) =
            group_pieces(theme, second_top, second_labels, &second_heights, &[false]);
        flats.push(panel);
        dividers.append(&mut card_dividers);
        texts.append(&mut card_texts);
        flats.append(&mut switch_pieces(
            theme,
            second_top,
            &second_heights,
            0,
            shown.dock.previews,
        ));
    }
    ContentScene {
        flats,
        dividers,
        texts,
    }
}

/// Contenido de la barra superior: un interruptor por icono.
fn menubar_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let shown = display_preferences(settings.values());
    let tops = card_tops(SECTION_MENUBAR);
    let Some(top) = tops.into_iter().next() else {
        return coming_soon(theme);
    };
    let cards = section_cards(SECTION_MENUBAR);
    let Some(labels) = cards.into_iter().next() else {
        return coming_soon(theme);
    };
    let heights = card_heights(SECTION_MENUBAR, 0);
    let dim = vec![false; labels.len()];
    let (panel, dividers, texts) = group_pieces(theme, top, &labels, &heights, &dim);
    let mut flats = vec![panel];
    let states = [
        shown.menubar.volume,
        shown.menubar.network,
        shown.menubar.music,
        shown.menubar.language,
        shown.menubar.search,
        shown.menubar.control_center,
        shown.menubar.background_apps,
    ];
    for (index, on) in states.iter().enumerate() {
        flats.append(&mut switch_pieces(theme, top, &heights, index, *on));
    }
    ContentScene {
        flats,
        dividers,
        texts,
    }
}

/// Contenido de la barra de tareas: ocultar sola la barra de Windows.
fn taskbar_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let tops = card_tops(SECTION_TASKBAR);
    let Some(top) = tops.into_iter().next() else {
        return coming_soon(theme);
    };
    let cards = section_cards(SECTION_TASKBAR);
    let Some(labels) = cards.into_iter().next() else {
        return coming_soon(theme);
    };
    let heights = card_heights(SECTION_TASKBAR, 0);
    let (panel, dividers, texts) = group_pieces(theme, top, &labels, &heights, &[false]);
    let mut flats = vec![panel];
    flats.append(&mut switch_pieces(
        theme,
        top,
        &heights,
        0,
        settings.taskbar_auto(),
    ));
    ContentScene {
        flats,
        dividers,
        texts,
    }
}
