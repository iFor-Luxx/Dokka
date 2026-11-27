//! Contenido de la ventana: tarjetas Tahoe con ajustes reales.
//!
//! Cada tarjeta es un plano claro con filas separadas por líneas finas,
//! etiqueta a la izquierda y control a la derecha. Los deslizadores usan el
//! componente `slider` y los interruptores el componente `switch`. La tarjeta
//! "Próximamente" sigue para las secciones sin ajustes.

use dokka_components::settings_group::{
    SettingsGroupProps, group_dividers, group_labels, group_panel,
};
use dokka_components::slider::{SliderColors, SliderProps, slider_panels, slider_track_bounds};
use dokka_components::switch::{SwitchProps, switch_dot, switch_panels, switch_touch};
use dokka_scene::{Divider, Dot, Panel, Rect, Text};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{
    SECTION_DOCK, SECTION_MENUBAR, SECTION_TASKBAR, card_tops, content_row,
    content_slider_area, has_content, icon_scale_to, magnification_to, section_cards,
};
use super::super::pointer::display_preferences;
use super::super::state::Settings;

/// Todo lo que pinta el contenido: planos, líneas, puntos y textos.
pub(crate) struct ContentScene {
    pub flats: Vec<Panel>,
    pub dividers: Vec<Divider>,
    pub dots: Vec<Dot>,
    pub texts: Vec<Text>,
}

/// Tarjetas con ajustes o la tarjeta gris "Próximamente".
pub(crate) fn content_scene(theme: &Theme, settings: &Settings) -> ContentScene {
    let selected = settings.selected();
    if !has_content(selected) {
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
    use dokka_scene::{TextAlign as Align};
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
        align: Align::Center,
    };
    ContentScene {
        flats: vec![panel],
        dividers: Vec::new(),
        dots: Vec::new(),
        texts: vec![text],
    }
}

/// Colores del deslizador de Preferencias, claros y neutros como la tarjeta.
fn slider_colors(theme: &Theme) -> SliderColors {
    SliderColors {
        track: theme.settings.slider_track,
        fill: theme.settings.slider_fill,
        muted: theme.settings.slider_muted,
    }
}

/// Tirador redondo al final del relleno, como en el volumen.
fn slider_knob(track: Rect, value: f32, active: bool, theme: &Theme) -> Dot {
    let clamped = value.clamp(0.0, 1.0);
    let fill = track.width * clamped;
    let diameter = measures::SWITCH_KNOB.min(track.height + 8.0).max(0.0);
    let center = (track.x + fill).clamp(track.x, track.x + track.width);
    Dot {
        x: center - diameter / 2.0,
        y: track.y + (track.height - diameter) / 2.0,
        diameter: diameter.max(0.0),
        color: if active {
            theme.settings.slider_fill
        } else {
            theme.settings.card_text
        },
    }
}

/// Planos y punto de un deslizador en su fila.
fn slider_pieces(
    theme: &Theme,
    top: f32,
    index: usize,
    value: f32,
    active: bool,
) -> (Vec<Panel>, Dot) {
    let area = content_slider_area(top, index);
    let track = slider_track_bounds(area);
    let props = SliderProps { value, active };
    let panels = slider_panels(track, &props, &slider_colors(theme));
    let knob = slider_knob(track, value, active, theme);
    (panels.into_iter().collect(), knob)
}

/// Planos y punto de un interruptor en su fila.
fn switch_pieces(theme: &Theme, top: f32, index: usize, on: bool) -> (Vec<Panel>, Dot) {
    let row = content_row(top, index);
    let touch = switch_touch(row);
    let props = SwitchProps { on };
    let panels = switch_panels(theme, touch, &props);
    let knob = switch_dot(theme, touch, &props);
    (panels.into_iter().collect(), knob)
}

/// Etiquetas, líneas y tarjeta de un grupo que empieza en `top`.
fn group_pieces(
    theme: &Theme,
    top: f32,
    labels: &[&'static str],
    dim: &[bool],
) -> (Panel, Vec<Divider>, Vec<Text>) {
    let owned: Vec<String> = labels.iter().map(|label| label.to_string()).collect();
    let props = SettingsGroupProps {
        top,
        rows: owned,
    };
    let panel = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);
    let texts = group_labels(theme, &props, dim);
    (panel, dividers, texts)
}

/// Contenido del Dock: tamaño, ampliación, cantidad y vistas previas.
fn dock_content(theme: &Theme, settings: &Settings) -> ContentScene {
    let shown = display_preferences(settings.values());
    let tops = card_tops(SECTION_DOCK);
    let cards = section_cards(SECTION_DOCK);
    let mut flats = Vec::new();
    let mut dividers = Vec::new();
    let mut dots = Vec::new();
    let mut texts = Vec::new();
    let strength_dim = !shown.dock.magnification;
    for (card, (top, labels)) in tops.iter().zip(cards.iter()).enumerate() {
        let dim = if card == 2 { vec![strength_dim] } else { vec![false] };
        let (panel, mut card_dividers, mut card_texts) = group_pieces(theme, *top, labels, &dim);
        flats.push(panel);
        dividers.append(&mut card_dividers);
        texts.append(&mut card_texts);
        match card {
            0 => {
                let (mut panels, knob) = slider_pieces(
                    theme,
                    *top,
                    0,
                    icon_scale_to(shown.dock.icon_scale),
                    true,
                );
                flats.append(&mut panels);
                dots.push(knob);
            }
            1 => {
                let (mut panels, knob) =
                    switch_pieces(theme, *top, 0, shown.dock.magnification);
                flats.append(&mut panels);
                dots.push(knob);
            }
            2 => {
                let (mut panels, knob) = slider_pieces(
                    theme,
                    *top,
                    0,
                    magnification_to(shown.dock.magnification_strength),
                    !strength_dim,
                );
                flats.append(&mut panels);
                dots.push(knob);
            }
            _ => {
                let (mut panels, knob) = switch_pieces(theme, *top, 0, shown.dock.previews);
                flats.append(&mut panels);
                dots.push(knob);
            }
        }
    }
    ContentScene {
        flats,
        dividers,
        dots,
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
    let dim = vec![false; labels.len()];
    let (panel, dividers, texts) = group_pieces(theme, top, &labels, &dim);
    let mut flats = vec![panel];
    let mut dots = Vec::new();
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
        let (mut panels, knob) = switch_pieces(theme, top, index, *on);
        flats.append(&mut panels);
        dots.push(knob);
    }
    ContentScene {
        flats,
        dividers,
        dots,
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
    let (panel, dividers, texts) = group_pieces(theme, top, &labels, &[false]);
    let mut flats = vec![panel];
    let (mut panels, knob) = switch_pieces(theme, top, 0, settings.taskbar_auto());
    flats.append(&mut panels);
    ContentScene {
        flats,
        dividers,
        dots: vec![knob],
        texts,
    }
}
