use std::sync::Arc;

use dokka_icons::checkmark_icon;
use dokka_scene::{Drawing, Icon, IconImage, Rect};
use dokka_theme::{Theme, menu as measures};

use super::MenuRow;

/// Marcas de opción elegida ya construidas con los colores de un tema.
/// Se reutilizan entre fotogramas: la plataforma solo repinta
/// cuando recibe otro dibujo.
#[derive(Debug)]
pub struct MenuChecks {
    normal: Arc<Drawing>,
    highlighted: Arc<Drawing>,
}

impl MenuChecks {
    pub fn new(theme: &Theme) -> Self {
        Self {
            normal: Arc::new(checkmark_icon(theme.menu.text)),
            highlighted: Arc::new(checkmark_icon(theme.menu.highlighted_text)),
        }
    }
}

/// Iconos de las filas elegidas: centrados en su fila y en la columna de marca.
pub(super) fn check_icons(
    theme: &Theme,
    checks: &MenuChecks,
    bounds: Rect,
    rows: &[MenuRow],
    highlighted: Option<usize>,
) -> Vec<Icon> {
    let mut top = bounds.y + measures::PADDING;
    let mut icons = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        if let MenuRow::Item(item) = row {
            if item.checked {
                let drawing = if highlighted == Some(index) {
                    &checks.highlighted
                } else {
                    &checks.normal
                };
                icons.push(Icon {
                    x: bounds.x
                        + measures::PADDING
                        + measures::TEXT_INSET
                        + (measures::CHECK_WIDTH - measures::CHECK_SIDE) / 2.0,
                    y: top + (measures::ROW_HEIGHT - measures::CHECK_SIDE) / 2.0,
                    side: measures::CHECK_SIDE,
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
