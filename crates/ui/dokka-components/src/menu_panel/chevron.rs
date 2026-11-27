use std::sync::Arc;

use dokka_icons::submenu_chevron_icon;
use dokka_scene::{Drawing, Icon, IconImage, Rect};
use dokka_theme::{Theme, menu as measures};

use super::MenuRow;

/// Chevrones ya construidos con los colores de un tema. Se reutilizan entre fotogramas:
/// la plataforma solo repinta cuando recibe otro dibujo.
#[derive(Debug)]
pub struct MenuChevrons {
    normal: Arc<Drawing>,
    highlighted: Arc<Drawing>,
}

impl MenuChevrons {
    pub fn new(theme: &Theme) -> Self {
        Self {
            normal: Arc::new(submenu_chevron_icon(theme.menu.text)),
            highlighted: Arc::new(submenu_chevron_icon(theme.menu.highlighted_text)),
        }
    }
}

/// Iconos de las filas con submenú: centrados en su fila y en la columna derecha,
/// con el mismo aire que la columna de iconos de la izquierda.
pub(super) fn chevron_icons(
    theme: &Theme,
    chevrons: &MenuChevrons,
    bounds: Rect,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Vec<Icon> {
    let mut top = bounds.y + measures::PADDING;
    let mut icons = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        if let MenuRow::Item(item) = row {
            if item.has_submenu {
                let drawing = if highlighted == Some(index) {
                    &chevrons.highlighted
                } else {
                    &chevrons.normal
                };
                icons.push(Icon {
                    x: bounds.x + bounds.width - measures::PADDING - measures::ARROW_WIDTH
                        + (measures::ARROW_WIDTH - measures::CHEVRON_SIDE) / 2.0,
                    y: top + (measures::ROW_HEIGHT - measures::CHEVRON_SIDE) / 2.0,
                    side: measures::CHEVRON_SIDE,
                    image: Some(IconImage::Drawing(Arc::clone(drawing))),
                    placeholder: theme.menu.background,
                    placeholder_radius: 0.0,
                });
            }
            top += measures::ROW_HEIGHT;
        } else {
            top += measures::SEPARATOR_HEIGHT;
        }
    }
    icons
}
