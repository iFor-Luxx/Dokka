/// Frenada suave al final: empieza rápido y se detiene sin pasarse.
/// `t` es el progreso lineal de 0 a 1.
pub fn ease_out_cubic(t: f32) -> f32 {
    let remaining = (1.0 - t).clamp(0.0, 1.0);
    1.0 - remaining * remaining * remaining
}
