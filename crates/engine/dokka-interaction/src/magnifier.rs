/// Lupa sobre un eje: crece lo cercano al cursor y aparta a los vecinos.
#[derive(Debug, Clone, Copy)]
pub struct Magnifier {
    pub cursor: f32,
    /// Distancia a partir de la cual no hay efecto.
    pub range: f32,
    pub maximum_scale: f32,
    /// Intensidad de 0 (apagada) a 1 (completa).
    pub strength: f32,
}

impl Magnifier {
    /// Escala de un elemento centrado en `position`.
    pub fn scale_at(&self, position: f32) -> f32 {
        let distance = ((position - self.cursor) / self.range).abs().min(1.0);
        // Curva suave: vale 1 bajo el cursor y 0 en el límite, sin saltos.
        let weight = 1.0 - distance * distance * (3.0 - 2.0 * distance);
        1.0 + (self.maximum_scale - 1.0) * self.strength * weight
    }

    /// Cuánto se aparta `position`. Es la suma acumulada del crecimiento,
    /// por eso los vecinos nunca se enciman.
    pub fn shift_at(&self, position: f32) -> f32 {
        let u = ((position - self.cursor) / self.range).clamp(-1.0, 1.0);
        let area = u - u.powi(3) + u.powi(3) * u.abs() / 2.0;
        (self.maximum_scale - 1.0) * self.strength * self.range * area
    }
}
