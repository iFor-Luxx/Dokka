//! Contenido de la barra superior: filas reordenables con agarre.
//!
//! Cada fila trae un agarre a la izquierda (seis puntos verticales en color
//! secundario), su nombre y su interruptor. La fila arrastrada sigue al
//! cursor en vertical, resaltada en plano; las demás se apartan dejando el
//! hueco donde caerá, sin salirse de la tarjeta. Debajo queda el texto
//! pequeño que explica el arrastre. Los planos van antes que iconos y textos
//! para que el resaltado nunca tape el texto de las demás filas.
//!
//! La cantidad y el estilo no cambian mientras se arrastra: tarjeta, un
//! resaltado siempre presente (de tamaño cero en reposo), dos planos por
//! interruptor, seis líneas, siete agarres, siete nombres y el texto de ayuda.
//! El orden de la lista es fijo (el base al pulsar): solo se mueven los
//! huecos, así `SceneLayers` no reconstruye.

use std::sync::Arc;

use dokka_components::settings_group::{SettingsGroupProps, group_dividers, group_panel};
use dokka_components::switch::{SwitchProps, switch_panels, switch_touch};
use dokka_model::MenubarItem;
use dokka_scene::{Drawing, Icon, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, settings as measures};

use super::super::logic::{
    SECTION_MENUBAR, card_heights, float_top, menubar_bottom, menubar_label, menubar_on,
    menubar_top,
};
use super::super::reorder_pointer::display_order;
use super::super::state::Settings;
use super::content::{ContentScene, coming_soon};

/// Texto bajo la tarjeta que explica el arrastre.
const HINT: &str = "Arrastra para cambiar el orden. La fila de arriba queda a la izquierda.";

/// Contenido de la barra superior: tarjeta reordenable y su ayuda.
pub(crate) fn menubar_content(
    theme: &Theme,
    settings: &Settings,
    grip: &Arc<Drawing>,
) -> ContentScene {
    let width = settings.content_width();
    let values = settings.values();
    let Some(top) = menubar_top() else {
        return coming_soon(theme, width);
    };
    let bottom = menubar_bottom(top);
    let shown = super::super::pointer::display_preferences(values);
    let base = values.preferences.menubar.order;
    let display = display_order(values);
    let active = values.reorder.is_some_and(|drag| drag.is_active());
    let dragged = values.reorder.map(|drag| drag.item());
    let floating = values
        .reorder
        .filter(|drag| drag.is_active())
        .map(|drag| float_top(top, drag.current_y()));

    let heights = card_heights(SECTION_MENUBAR, 0);
    let owned: Vec<String> = base
        .iter()
        .map(|item| menubar_label(*item).to_owned())
        .collect();
    let props = SettingsGroupProps {
        top,
        rows: owned,
        heights: heights.clone(),
        width,
    };
    let card = group_panel(theme, &props);
    let dividers = group_dividers(theme, &props);

    // Resaltado de la fila arrastrada: siempre existe para no reconstruir;
    // en reposo es de tamaño cero con el mismo estilo.
    let highlight = if let Some(y) = floating {
        Panel {
            bounds: Rect {
                x: measures::CONTENT_LEFT,
                y,
                width,
                height: measures::CONTENT_ROW_HEIGHT,
            },
            corner_radius: measures::REORDER_HIGHLIGHT_RADIUS,
            border_width: 0.0,
            tint: theme.settings.hover,
            border: theme.settings.hover,
        }
    } else {
        Panel {
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            corner_radius: measures::REORDER_HIGHLIGHT_RADIUS,
            border_width: 0.0,
            tint: theme.settings.hover,
            border: theme.settings.hover,
        }
    };

    let mut flats = vec![card, highlight];
    let mut texts = Vec::with_capacity(8);
    let mut icons = Vec::with_capacity(7);
    // Orden fijo (el de fábrica) para que el estilo de cada plano no cambie al
    // mover ni al soltar: solo se mueven los huecos.
    for item in MenubarItem::FACTORY_ORDER {
        let position = display.iter().position(|known| *known == item).unwrap_or(0);
        let visual = if active && Some(item) == dragged {
            floating.unwrap_or(top + position as f32 * measures::CONTENT_ROW_HEIGHT)
        } else {
            top + position as f32 * measures::CONTENT_ROW_HEIGHT
        };
        let row = Rect {
            x: measures::CONTENT_LEFT,
            y: visual,
            width,
            height: measures::CONTENT_ROW_HEIGHT,
        };
        // Agarre a la izquierda en color secundario.
        icons.push(Icon {
            x: row.x + measures::CONTENT_PADDING,
            y: visual + (measures::CONTENT_ROW_HEIGHT - measures::REORDER_GRIP_SIDE) / 2.0,
            side: measures::REORDER_GRIP_SIDE,
            image: Some(dokka_scene::IconImage::Drawing(Arc::clone(grip))),
            placeholder: theme.settings.card_text,
            placeholder_radius: 0.0,
        });
        texts.push(Text {
            x: row.x
                + measures::CONTENT_PADDING
                + measures::REORDER_GRIP_SIDE
                + measures::REORDER_GRIP_GAP,
            center_y: visual + measures::CONTENT_ROW_HEIGHT / 2.0,
            content: menubar_label(item).to_owned(),
            size: measures::CONTENT_LABEL_SIZE,
            is_bold: false,
            color: theme.settings.row_text,
            align: TextAlign::Left,
        });
        let touch = switch_touch(row);
        let panels = switch_panels(
            theme,
            touch,
            &SwitchProps {
                on: menubar_on(&shown.menubar, item),
            },
        );
        flats.extend(panels);
    }
    texts.push(Text {
        x: measures::CONTENT_LEFT + measures::CONTENT_PADDING,
        center_y: bottom + measures::REORDER_HINT_GAP + measures::REORDER_HINT_SIZE / 2.0,
        content: HINT.to_owned(),
        size: measures::REORDER_HINT_SIZE,
        is_bold: false,
        color: theme.settings.card_text,
        align: TextAlign::Left,
    });
    // Las líneas separan los huecos fijos de la tarjeta.
    ContentScene {
        flats,
        dividers,
        icons,
        texts,
    }
}
