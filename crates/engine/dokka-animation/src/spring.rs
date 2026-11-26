const SETTLE_DISTANCE: f32 = 0.001;
const SETTLE_VELOCITY: f32 = 0.01;

/// Resorte críticamente amortiguado: llega al destino sin pasarse.
#[derive(Debug, Clone, Copy)]
pub struct Spring {
    value: f32,
    velocity: f32,
    target: f32,
    stiffness: f32,
}

impl Spring {
    /// `stiffness` es la frecuencia angular: a más valor, más rápido.
    pub fn new(value: f32, stiffness: f32) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            stiffness,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn is_settled(&self) -> bool {
        self.value == self.target && self.velocity == 0.0
    }

    /// Usa la solución exacta, así el resultado no depende de los fotogramas por segundo.
    pub fn advance(&mut self, seconds: f32) {
        let offset = self.value - self.target;
        let decay = (-self.stiffness * seconds).exp();
        let momentum = self.velocity + self.stiffness * offset;
        let next_offset = (offset + momentum * seconds) * decay;
        self.velocity = (self.velocity - self.stiffness * momentum * seconds) * decay;
        self.value = self.target + next_offset;

        if next_offset.abs() < SETTLE_DISTANCE && self.velocity.abs() < SETTLE_VELOCITY {
            self.value = self.target;
            self.velocity = 0.0;
        }
    }
}
