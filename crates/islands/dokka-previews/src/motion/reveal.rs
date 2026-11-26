//! Progreso de aparición de un panel: 0 oculto, 1 en reposo.

use dokka_animation::ease_out_cubic;

/// Sube al entrar y baja al salir; si vuelve el cursor, retoma desde donde está.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Reveal {
    progress: f32,
    leaving: bool,
}

impl Reveal {
    pub(crate) fn hidden() -> Self {
        Self {
            progress: 0.0,
            leaving: false,
        }
    }

    pub(crate) fn start_exit(&mut self) {
        self.leaving = true;
    }

    pub(crate) fn cancel_exit(&mut self) {
        self.leaving = false;
    }

    pub(crate) fn is_leaving(&self) -> bool {
        self.leaving
    }

    /// `true` mientras entra o sale.
    pub(crate) fn is_animating(&self) -> bool {
        if self.leaving {
            self.progress > 0.0
        } else {
            self.progress < 1.0
        }
    }

    /// `true` cuando la salida llegó al final.
    pub(crate) fn has_exited(&self) -> bool {
        self.leaving && self.progress <= 0.0
    }

    pub(crate) fn advance(&mut self, seconds: f32, enter_seconds: f32, exit_seconds: f32) {
        if self.leaving {
            if exit_seconds > 0.0 {
                self.progress = (self.progress - seconds / exit_seconds).max(0.0);
            } else {
                self.progress = 0.0;
            }
        } else if enter_seconds > 0.0 {
            self.progress = (self.progress + seconds / enter_seconds).min(1.0);
        } else {
            self.progress = 1.0;
        }
    }

    /// Progreso suavizado para desplazar y fundir.
    pub(crate) fn eased(&self) -> f32 {
        ease_out_cubic(self.progress)
    }
}
