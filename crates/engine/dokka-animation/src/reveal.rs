//! Progreso de aparición de un panel: 0 oculto, 1 en reposo.

use crate::{ease_out_back, ease_out_cubic};

/// Sube al entrar y baja al salir; si vuelve el cursor, retoma desde donde está.
#[derive(Debug, Clone, Copy)]
pub struct Reveal {
    progress: f32,
    leaving: bool,
}

impl Reveal {
    pub fn hidden() -> Self {
        Self {
            progress: 0.0,
            leaving: false,
        }
    }

    /// Ya visible y en reposo: para empezar un fundido de salida.
    pub fn visible() -> Self {
        Self {
            progress: 1.0,
            leaving: false,
        }
    }

    /// Cuánto se ve, de 0 (oculto) a 1 (reposo).
    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn start_exit(&mut self) {
        self.leaving = true;
    }

    pub fn cancel_exit(&mut self) {
        self.leaving = false;
    }

    pub fn is_leaving(&self) -> bool {
        self.leaving
    }

    /// `true` mientras entra o sale.
    pub fn is_animating(&self) -> bool {
        if self.leaving {
            self.progress > 0.0
        } else {
            self.progress < 1.0
        }
    }

    /// `true` cuando la salida llegó al final.
    pub fn has_exited(&self) -> bool {
        self.leaving && self.progress <= 0.0
    }

    pub fn advance(&mut self, seconds: f32, enter_seconds: f32, exit_seconds: f32) {
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

    /// Movimiento suavizado con un rebote muy leve al llegar.
    /// Vale 0 al empezar y 1 en reposo; se pasa un poco por encima de 1.
    pub fn motion(&self) -> f32 {
        ease_out_back(self.progress)
    }

    /// Opacidad que llega a 1 en la primera mitad de la entrada.
    /// Al salir se queda opaco un momento y luego se desvanece acelerando.
    pub fn fade(&self, span: f32) -> f32 {
        if span <= 0.0 {
            return if self.progress > 0.0 { 1.0 } else { 0.0 };
        }
        ease_out_cubic((self.progress / span).clamp(0.0, 1.0))
    }

    /// Escala actual: crece desde el icono al entrar y se encoge al salir.
    /// Si el cursor vuelve a mitad de salida, continúa desde el progreso actual.
    pub fn scale(&self, enter_start: f32, exit_end: f32) -> f32 {
        let start = if self.leaving { exit_end } else { enter_start };
        start + (1.0 - start) * self.motion()
    }

    /// Cuánto baja el panel respecto al reposo. En el rebote es negativo.
    pub fn drop(&self, enter_travel: f32, exit_travel: f32) -> f32 {
        let travel = if self.leaving {
            exit_travel
        } else {
            enter_travel
        };
        (1.0 - self.motion()) * travel
    }
}
