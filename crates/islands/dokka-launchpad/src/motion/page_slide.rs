//! Deslizamiento de la cuadrícula y tope elástico en los extremos.
//!
//! Guarda de dónde a dónde va la página y cuánto lleva. La escena dibuja
//! siempre la página destino y la desplaza hasta su sitio: la cantidad de
//! iconos y textos no cambia durante el deslizamiento y las capas solo se
//! actualizan, sin reconstruirse en cada fotograma.

use std::f32::consts::PI;

use dokka_animation::ease_out_cubic;
use dokka_theme::launchpad as measures;

/// Transición en curso: deslizamiento entre páginas o cesión en un extremo.
#[derive(Debug, Clone, Copy)]
enum SlideKind {
    Idle,
    Slide {
        from: usize,
        to: usize,
        elapsed: f32,
    },
    Nudge {
        /// Hacia dónde cede: +1 a la derecha, -1 a la izquierda.
        side: f32,
        elapsed: f32,
    },
}

/// Cuánto se ha movido la cuadrícula y hacia dónde va.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PageMotion {
    kind: SlideKind,
}

impl PageMotion {
    pub(crate) fn new() -> Self {
        Self {
            kind: SlideKind::Idle,
        }
    }

    /// `true` mientras la cuadrícula se desliza o cede.
    pub(crate) fn is_moving(&self) -> bool {
        !matches!(self.kind, SlideKind::Idle)
    }

    /// Empieza a deslizarse de una página a otra vecina o lejana.
    pub(crate) fn start_slide(&mut self, from: usize, to: usize) {
        if from == to {
            return;
        }
        self.kind = SlideKind::Slide {
            from,
            to,
            elapsed: 0.0,
        };
    }

    /// Cede un poco hacia ese lado y vuelve, sin cambiar de página.
    pub(crate) fn start_nudge(&mut self, side: f32) {
        self.kind = SlideKind::Nudge {
            side: side.signum(),
            elapsed: 0.0,
        };
    }

    /// Avanza el tiempo; indica si la transición sigue en curso.
    pub(crate) fn advance(&mut self, seconds: f32) -> bool {
        let elapsed = match &mut self.kind {
            SlideKind::Idle => return false,
            SlideKind::Slide { elapsed, .. } | SlideKind::Nudge { elapsed, .. } => {
                *elapsed += seconds.max(0.0);
                *elapsed
            }
        };
        if elapsed >= measures::PAGE_SLIDE_SECONDS {
            self.kind = SlideKind::Idle;
            return false;
        }
        true
    }

    /// Desplazamiento horizontal de la cuadrícula para este ancho.
    pub(crate) fn offset(&self, screen_width: f32) -> f32 {
        match self.kind {
            SlideKind::Idle => 0.0,
            SlideKind::Slide { from, to, elapsed } => {
                let progress = (elapsed / measures::PAGE_SLIDE_SECONDS).clamp(0.0, 1.0);
                let distance = screen_width * (1.0 - ease_out_cubic(progress));
                if to > from { distance } else { -distance }
            }
            SlideKind::Nudge { side, elapsed } => {
                let progress = (elapsed / measures::PAGE_SLIDE_SECONDS).clamp(0.0, 1.0);
                side * (PI * progress).sin() * measures::page_nudge_distance(screen_width)
            }
        }
    }
}

impl Default for PageMotion {
    fn default() -> Self {
        Self::new()
    }
}
