use std::f32::consts::PI;

const HOP_SECONDS: f32 = 0.7;
/// Tope si lo esperado nunca ocurre (unos diez segundos).
const MAXIMUM_HOPS: f32 = 14.0;

/// Saltos repetidos mientras se espera algo.
#[derive(Debug, Clone, Copy)]
pub struct Bounce {
    elapsed: f32,
    end: f32,
}

impl Bounce {
    pub fn new() -> Self {
        Self {
            elapsed: 0.0,
            end: HOP_SECONDS * MAXIMUM_HOPS,
        }
    }

    /// Termina al aterrizar el salto en curso, nunca en el aire.
    pub fn finish(&mut self) {
        let landing = (self.elapsed / HOP_SECONDS).ceil().max(1.0) * HOP_SECONDS;
        self.end = self.end.min(landing);
    }

    /// Avanza el tiempo; indica si todavía sigue saltando.
    pub fn advance(&mut self, seconds: f32) -> bool {
        self.elapsed += seconds;
        self.elapsed < self.end
    }

    /// Altura actual para un salto de altura máxima `height`.
    pub fn lift(&self, height: f32) -> f32 {
        height * (PI * self.elapsed / HOP_SECONDS).sin().abs()
    }
}

impl Default for Bounce {
    fn default() -> Self {
        Self::new()
    }
}
