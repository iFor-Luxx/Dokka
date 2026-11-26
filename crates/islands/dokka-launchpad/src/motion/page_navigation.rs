//! Cambio de página del Launchpad: rueda, puntos y flechas.
//!
//! La rueda y las flechas no encadenan: mientras la cuadrícula se mueve se
//! ignoran los avisos nuevos. Fuera de los extremos la cuadrícula cede hacia
//! ese lado y vuelve, sin cambiar de página.

use dokka_scene::Size;

use crate::Launchpad;

impl Launchpad {
    /// La rueda cambia de página: hacia arriba retrocede, hacia abajo avanza.
    pub fn scroll(&mut self, screen: Size, delta: i16) {
        if delta > 0 {
            self.step_to(screen, -1);
        } else if delta < 0 {
            self.step_to(screen, 1);
        }
    }

    /// Flecha izquierda: retrocede o cede a la derecha en la primera página.
    pub fn step_backward(&mut self, screen: Size) {
        self.step_to(screen, -1);
    }

    /// Flecha derecha: avanza o cede a la izquierda en la última página.
    pub fn step_forward(&mut self, screen: Size) {
        self.step_to(screen, 1);
    }

    /// Va a esta página si existe, deslizándose desde la actual.
    pub fn go_to_page(&mut self, screen: Size, page: usize) {
        if self.motion.is_moving() {
            return;
        }
        let from = self.current_page(screen);
        let target = page.min(self.page_count(screen) - 1);
        if target == from {
            return;
        }
        self.page = target;
        self.highlighted = None;
        self.motion.start_slide(from, target);
    }

    /// Avanza la transición y el zoom; indica si algo sigue moviéndose.
    pub fn advance(&mut self, seconds: f32) -> bool {
        let paging = self.motion.advance(seconds);
        let zooming = self.zoom.advance(seconds);
        paging || zooming
    }

    /// `true` mientras la cuadrícula se desliza o cede.
    pub fn is_moving(&self) -> bool {
        self.motion.is_moving()
    }

    /// Desplazamiento horizontal actual de la cuadrícula.
    pub fn motion_offset(&self, screen_width: f32) -> f32 {
        self.motion.offset(screen_width)
    }

    fn step_to(&mut self, screen: Size, direction: i32) {
        if self.motion.is_moving() {
            return;
        }
        let count = self.page_count(screen);
        let current = self.current_page(screen);
        let target = current as i32 + direction;
        if target < 0 {
            self.motion.start_nudge(1.0);
        } else if target >= count as i32 {
            self.motion.start_nudge(-1.0);
        } else {
            self.page = target as usize;
            self.motion.start_slide(current, target as usize);
        }
        self.highlighted = None;
    }
}
