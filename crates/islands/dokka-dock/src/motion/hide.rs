//! Auto-ocultado: se desliza fuera al perder el cursor y vuelve al tocar el borde.

use dokka_theme::dock as measures;

/// Espera y deslizamiento del auto-ocultado.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Hide {
    enabled: bool,
    /// 0 visible, 1 escondido.
    progress: f32,
    /// Tiempo desde que se perdió el cursor sin nada abierto.
    away: Option<f32>,
    cursor_inside: bool,
    busy: bool,
}

impl Hide {
    pub(crate) fn new() -> Self {
        Self {
            enabled: false,
            progress: 0.0,
            away: None,
            cursor_inside: false,
            busy: false,
        }
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.progress = 0.0;
            self.away = None;
        }
    }

    pub(crate) fn set_cursor_inside(&mut self, inside: bool) {
        self.cursor_inside = inside;
        if inside {
            self.away = None;
        }
    }

    pub(crate) fn set_busy(&mut self, busy: bool) {
        let was_busy = self.busy;
        self.busy = busy;
        if busy {
            self.away = None;
        } else if was_busy && !self.cursor_inside {
            // Acaba de cerrarse algo con el cursor fuera: empieza la espera.
            self.away = Some(0.0);
        }
    }

    pub(crate) fn show_now(&mut self) {
        self.progress = 0.0;
        self.away = None;
    }

    pub(crate) fn progress(&self) -> f32 {
        if self.enabled { self.progress } else { 0.0 }
    }

    pub(crate) fn is_hidden(&self) -> bool {
        self.enabled && self.progress >= 0.999
    }

    pub(crate) fn is_animating(&self) -> bool {
        if !self.enabled {
            return false;
        }
        if self.away.is_some() {
            return true;
        }
        let target = self.target();
        (self.progress - target).abs() > 0.001
    }

    fn target(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        if self.cursor_inside || self.busy {
            0.0
        } else {
            1.0
        }
    }

    /// Avanza la espera y el deslizamiento; indica si queda movimiento.
    pub(crate) fn advance(&mut self, seconds: f32) -> bool {
        if !self.enabled {
            return false;
        }
        let target = self.target();
        if target > 0.5 && self.progress < 0.5 {
            // Fuera del dock y sin nada abierto: espera con retardo.
            let waited = match self.away {
                Some(waited) => waited + seconds,
                None => 0.0,
            };
            self.away = Some(waited);
            if waited < measures::HIDE_DELAY_SECONDS {
                return true;
            }
        } else if target < 0.5 {
            self.away = None;
        }
        let span = if target > 0.5 {
            measures::HIDE_MOVE_SECONDS
        } else {
            measures::SHOW_MOVE_SECONDS
        };
        if span <= 0.0 {
            self.progress = target;
            return false;
        }
        let step = seconds / span;
        if self.progress < target {
            self.progress = (self.progress + step).min(target);
        } else if self.progress > target {
            self.progress = (self.progress - step).max(target);
            if self.progress <= 0.0 {
                self.away = None;
            }
        } else {
            self.away = None;
        }
        self.is_animating()
    }

    /// Desplazamiento suavizado de 0 (visible) a 1 (escondido).
    pub(crate) fn eased(&self) -> f32 {
        let progress = self.progress().clamp(0.0, 1.0);
        progress * progress * (3.0 - 2.0 * progress)
    }
}
