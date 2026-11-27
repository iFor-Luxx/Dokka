//! Colocación de la ventana del dock: tamaño según borde y encogido en vertical.

use std::error::Error;

use dokka_dock::{DOCK_BOTTOM_MARGIN, DOCK_EDGE_MARGIN};
use dokka_win_surface::{Surface, SurfaceOptions};

use crate::dock_session::DockSession;

impl DockSession {
    /// Ajusta la ventana al número de iconos y a su borde.
    /// En caliente recoloca y redimensiona para la nueva posición.
    /// En vertical encoge si hay muchas apps para caber bajo la barra superior;
    /// si algo no puede calcularse, abajo y visible (falla sola).
    pub(crate) fn resize(&mut self, surface: &mut Surface) -> Result<(), Box<dyn Error>> {
        let position = self.preferences.dock.position;
        // Con una app ocupando, el dock baja hasta el monitor completo para
        // que la línea quede al borde; si no, usa el área de trabajo.
        let over_taskbar =
            self.taskbar_hidden || (self.preferences.dock.auto_hide && self.dock.is_occluded());
        self.fit_to_work_area(surface, position, over_taskbar);
        let size = self.dock.surface_size(self.state.apps());
        // Con una app ocupando y auto-ocultado, la ventana toca el borde
        // (margen 0): el píxel 0 sigue dentro de la ventana durante toda
        // la subida y el cursor no se pierde en el hueco. Sin oclusión,
        // el margen habitual para despegar el dock del borde.
        let margin = if self.preferences.dock.auto_hide && self.dock.is_occluded() {
            0.0
        } else {
            match position {
                dokka_model::DockPosition::Bottom => DOCK_BOTTOM_MARGIN,
                _ => DOCK_EDGE_MARGIN,
            }
        };
        let placed = surface.resize(SurfaceOptions {
            width: size.width,
            height: size.height,
            bottom_margin: margin,
            over_taskbar,
            position,
            hide: self.dock.hide_eased(),
        });
        if placed.is_err() && !matches!(position, dokka_model::DockPosition::Bottom) {
            // Falla sola: abajo y visible.
            self.dock.set_fit(1.0);
            let size = self.dock.surface_size(self.state.apps());
            surface.resize(SurfaceOptions {
                width: size.width,
                height: size.height,
                bottom_margin: DOCK_BOTTOM_MARGIN,
                over_taskbar,
                position: dokka_model::DockPosition::Bottom,
                hide: 0.0,
            })?;
            return Ok(());
        }
        placed?;
        Ok(())
    }

    /// Reposiciona la ventana durante el auto-ocultado sin cambiar su tamaño.
    /// Solo se llama mientras anima: en reposo no hay sondeo.
    pub(crate) fn apply_hide(&mut self, surface: &mut Surface) -> Result<(), Box<dyn Error>> {
        self.resize(surface)
    }

    fn fit_to_work_area(
        &mut self,
        surface: &Surface,
        position: dokka_model::DockPosition,
        over_taskbar: bool,
    ) {
        if !matches!(
            position,
            dokka_model::DockPosition::Left | dokka_model::DockPosition::Right
        ) {
            self.dock.set_fit(1.0);
            return;
        }
        let [_, work_height] = surface.work_area_logical(over_taskbar);
        if work_height <= 0.0 {
            self.dock.set_fit(1.0);
            return;
        }
        self.dock.set_fit(1.0);
        let apps = self.state.apps();
        let needed = self.dock.surface_size(apps).height;
        if needed <= work_height {
            return;
        }
        // Margen mínimo arriba y abajo: el del menú, que no encoge.
        let side = dokka_theme::menu::WIDTH + dokka_theme::menu::SUBMENU_GAP;
        let bar_needed = needed - side * 2.0;
        let bar_max = (work_height - side * 2.0).max(0.0);
        if bar_needed <= 0.0 {
            return;
        }
        self.dock.set_fit(bar_max / bar_needed);
    }
}
