/// Frenada suave al final: empieza rápido y se detiene sin pasarse.
/// `t` es el progreso lineal de 0 a 1.
pub fn ease_out_cubic(t: f32) -> f32 {
    let remaining = (1.0 - t).clamp(0.0, 1.0);
    1.0 - remaining * remaining * remaining
}

/// Rebote muy leve al llegar: se pasa un poco y vuelve, sin vibrar.
/// `t` es el progreso lineal de 0 a 1.
pub fn ease_out_back(t: f32) -> f32 {
    let clamped = t.clamp(0.0, 1.0);
    // Constante suave: menor que la clásica 1.70158 para que apenas se pase.
    const SOFTNESS: f32 = 1.2;
    let distance = clamped - 1.0;
    1.0 + (SOFTNESS + 1.0) * distance * distance * distance + SOFTNESS * distance * distance
}
