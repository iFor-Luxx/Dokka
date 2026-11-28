//! Galería dibujada: cristal claro con la columna, las secciones al mismo
//! tamaño que el centro, el pie fijo y la sombra bajo la ventana.
//!
//! Cada control lo dibuja la vista previa del centro (`preview_panels`), no
//! otro dibujo. Los que ya están en el centro llevan un velo claro y el
//! nombre en gris, sin responder (el golpe los devuelve igual y la isla los
//! deja sin acción). Las piezas fijas (casilla, líneas, pie) viven en
//! `chrome`.

use std::sync::Arc;

use dokka_scene::{Divider, Drawing, Icon, IconImage, Overlay, Panel, Rect, Text, TextAlign};
use dokka_theme::{Theme, control_gallery as measures};

use super::super::control_center_panel::{ControlCenterIcons, preview_panels};
use super::categories::{CATEGORY_COUNT, category_name, item_name};
use super::chrome::{
    accept_label, accept_tile, capped, column_divider, footer_divider, hint_text, search_cursor,
    search_icon, search_text, search_tile,
};
use super::layout::{category_row, grid_view, list_view, place_sections, search_box};
use super::props::ControlGalleryProps;

/// Ventana de la galería sobre el cristal claro del tema.
pub fn gallery_panel(
    theme: &Theme,
    icons: &ControlCenterIcons,
    magnifier: &Arc<Drawing>,
    badges: &[Arc<Drawing>],
    gallery: Rect,
    props: &ControlGalleryProps,
    list_scroll: f32,
    grid_scroll: f32,
    cursor_visible: bool,
) -> Overlay {
    let search = search_box(gallery);
    let with_brightness = props.center.brightness.is_some();
    let placed = place_sections(
        gallery,
        props.selected,
        &props.query,
        with_brightness,
        grid_scroll,
    );
    let mut tiles = vec![search_tile(theme, search), accept_tile(theme, gallery)];
    let mut flats = Vec::new();
    let mut texts = vec![search_text(theme, search, &props.query), hint_text(theme, gallery)];
    texts.push(accept_label(theme, gallery));
    let mut overlay_icons = vec![search_icon(theme, magnifier, search)];
    let mut dividers = vec![
        column_divider(theme, gallery),
        footer_divider(theme, gallery),
        search_cursor(theme, search, &props.query, cursor_visible),
    ];
    add_categories(
        theme,
        gallery,
        badges,
        props.selected,
        list_scroll,
        &mut tiles,
        &mut flats,
        &mut texts,
        &mut overlay_icons,
    );
    add_sections(
        theme,
        icons,
        gallery,
        props,
        &placed,
        &mut tiles,
        &mut flats,
        &mut texts,
        &mut overlay_icons,
        &mut dividers,
    );
    Overlay {
        panel: Panel {
            bounds: gallery,
            corner_radius: capped(gallery, measures::CORNER_RADIUS),
            border_width: measures::BORDER_WIDTH,
            tint: theme.tray.panel,
            border: theme.tray.panel_border,
        },
        tiles,
        panel_shadow: Some(dokka_scene::Shadow {
            color: theme.tray.shadow,
            blur: measures::SHADOW_BLUR,
            offset: (0.0, measures::SHADOW_OFFSET_Y),
        }),
        tile_shadow: None,
        pill: None,
        highlight: None,
        flat_panels: flats,
        dividers,
        dots: Vec::new(),
        pointer: None,
        texts,
        icons: overlay_icons,
        thumbnails: Vec::new(),
        controls: None,
        opacity: 1.0,
        scale: 1.0,
        origin: (0.0, 0.0),
    }
}

/// Filas de la columna con su cuadrito de color y su nombre.
fn add_categories(
    theme: &Theme,
    gallery: Rect,
    badges: &[Arc<Drawing>],
    selected: usize,
    list_scroll: f32,
    tiles: &mut Vec<Panel>,
    flats: &mut Vec<Panel>,
    texts: &mut Vec<Text>,
    icons: &mut Vec<Icon>,
) {
    let view = list_view(gallery);
    for index in 0..CATEGORY_COUNT {
        let row = category_row(gallery, index, list_scroll);
        if row.y + row.height < view.y || row.y > view.y + view.height {
            continue;
        }
        if index == selected {
            tiles.push(Panel {
                bounds: row,
                corner_radius: measures::ROW_RADIUS,
                border_width: 0.0,
                tint: theme.settings.hover,
                border: theme.settings.hover,
            });
        }
        let badge = Rect {
            x: row.x + measures::PADDING,
            y: row.y + (row.height - measures::BADGE_SIDE) / 2.0,
            width: measures::BADGE_SIDE,
            height: measures::BADGE_SIDE,
        };
        let tint = theme.settings.badge[index % theme.settings.badge.len()];
        flats.push(Panel {
            bounds: badge,
            corner_radius: measures::BADGE_RADIUS,
            border_width: 0.0,
            tint,
            border: tint,
        });
        if let Some(drawing) = badges.get(index) {
            icons.push(Icon {
                x: badge.x + (badge.width - measures::BADGE_ICON_SIDE) / 2.0,
                y: badge.y + (badge.height - measures::BADGE_ICON_SIDE) / 2.0,
                side: measures::BADGE_ICON_SIDE,
                image: Some(IconImage::Drawing(Arc::clone(drawing))),
                placeholder: theme.menu.highlighted_text,
                placeholder_radius: 0.0,
            });
        }
        texts.push(Text {
            x: badge.x + badge.width + measures::BADGE_TEXT_GAP,
            center_y: row.y + row.height / 2.0,
            content: category_name(index).to_owned(),
            size: measures::ROW_TEXT_SIZE,
            is_bold: index == selected,
            color: theme.tray.text,
            align: TextAlign::Left,
        });
    }
}

/// Secciones con cada control igual que en el centro y su nombre debajo.
fn add_sections(
    theme: &Theme,
    icons: &ControlCenterIcons,
    gallery: Rect,
    props: &ControlGalleryProps,
    placed: &[super::layout::PlacedSection],
    tiles: &mut Vec<Panel>,
    flats: &mut Vec<Panel>,
    texts: &mut Vec<Text>,
    overlay_icons: &mut Vec<Icon>,
    dividers: &mut Vec<Divider>,
) {
    let view = grid_view(gallery);
    for section in placed {
        if section.title.y + section.title.height >= view.y
            && section.title.y <= view.y + view.height
        {
            texts.push(Text {
                x: section.title.x,
                center_y: section.title.y + measures::SECTION_TITLE_HEIGHT / 2.0 - 2.0,
                content: category_name(section.category).to_owned(),
                size: measures::SECTION_TITLE_SIZE,
                is_bold: true,
                color: theme.tray.secondary,
                align: TextAlign::Left,
            });
            dividers.push(Divider {
                bounds: Rect {
                    x: section.title.x,
                    y: section.title.y + measures::SECTION_TITLE_HEIGHT
                        - measures::DIVIDER_HEIGHT,
                    width: section.title.width,
                    height: measures::DIVIDER_HEIGHT,
                },
                color: theme.tray.separator,
            });
        }
        for cell in &section.items {
            let (mut tile, mut flat, mut text, mut icon) = preview_panels(
                theme,
                icons,
                cell.item,
                &props.center,
                (cell.tile.x, cell.tile.y),
            );
            let dimmed = props.center.hidden.contains(cell.item);
            if dimmed && let Some(first) = tile.first() {
                flat.push(Panel {
                    bounds: first.bounds,
                    corner_radius: first.corner_radius,
                    border_width: 0.0,
                    tint: theme.menubar.hover,
                    border: theme.menubar.hover,
                });
            }
            tiles.append(&mut tile);
            flats.append(&mut flat);
            texts.append(&mut text);
            overlay_icons.append(&mut icon);
            texts.push(Text {
                x: cell.tile.x + cell.tile.width / 2.0,
                center_y: cell.label_y,
                content: item_name(cell.item).to_owned(),
                size: measures::LABEL_TEXT_SIZE,
                is_bold: false,
                color: if dimmed {
                    theme.tray.secondary
                } else {
                    theme.tray.text
                },
                align: TextAlign::Center,
            });
        }
    }
}
