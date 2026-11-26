//! Apertura y cierre del Launchpad con el zoom clásico hacia el usuario.
//!
//! La apertura entra desde lejos encogiendo y apareciendo, frenando; el cierre
//! crece hacia el usuario mientras todo se desvanece. Si se reabre a mitad del
//! cierre, continúa desde donde está, sin saltos.

use dokka_animation::ease_out_cubic;
use dokka_theme::launchpad as measures;

/// Progreso del zoom: 0 en reposo, 1 lejos (grande y transparente).
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ZoomMotion {
    progress: f32,
    closing: bool,
}

impl ZoomMotion {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Empieza a abrirse desde lejos.
    pub(crate) fn open(&mut self) {
        self.progress = 1.0;
        self.closing = false;
    }

    /// Empieza a cerrarse desde donde está.
    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
    }

    /// Vuelve a abrirse desde donde está, cancelando el cierre.
    pub(crate) fn reopen(&mut self) {
        self.closing = false;
    }

    /// `true` mientras se abre o se cierra.
    pub(crate) fn is_zooming(&self) -> bool {
        if self.closing {
            self.progress < 1.0
        } else {
            self.progress > 0.0
        }
    }

    /// `true` mientras el cierre está en curso.
    pub(crate) fn is_closing(&self) -> bool {
        self.closing
    }

    /// `true` cuando el cierre llegó al final y se puede ocultar la ventana.
    pub(crate) fn close_finished(&self) -> bool {
        self.closing && self.progress >= 1.0
    }

    /// Avanza el tiempo; indica si el zoom sigue en curso.
    pub(crate) fn advance(&mut self, seconds: f32) -> bool {
        let step = seconds.max(0.0);
        if self.closing {
            if measures::ZOOM_CLOSE_SECONDS > 0.0 {
                self.progress = (self.progress + step / measures::ZOOM_CLOSE_SECONDS).min(1.0);
            } else {
                self.progress = 1.0;
            }
        } else if measures::ZOOM_OPEN_SECONDS > 0.0 {
            self.progress = (self.progress - step / measures::ZOOM_OPEN_SECONDS).max(0.0);
        } else {
            self.progress = 0.0;
        }
        self.is_zooming()
    }

    /// Cuánto se ve, de 0 (lejos) a 1 (reposo).
    pub(crate) fn opacity(&self) -> f32 {
        ease_out_cubic(1.0 - self.progress)
    }

    /// Escala del contenido: crece hacia el usuario al cerrar.
    pub(crate) fn scale(&self) -> f32 {
        1.0 + (measures::ZOOM_FAR_SCALE - 1.0) * ease_out_cubic(self.progress)
    }
}
