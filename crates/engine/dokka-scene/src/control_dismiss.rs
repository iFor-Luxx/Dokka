//! Salida del centro de control hacia su icono, descrita una sola vez.
//!
//! La isla decide cuándo empieza (al cerrarse por inactividad, clic fuera,
//! Escape o repetir su icono) y cuándo termina (cierre real tras el tope
//! total). La plataforma crea y lanza las animaciones de Composition con
//! estos datos, sin que Dokka recalcule cada fotograma.

/// Destino y tiempos de la salida en bloque, en unidades lógicas y segundos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlDismiss {
    /// Punto del icono del centro en la barra, al que vuelve todo el centro.
    pub target: (f32, f32),
    /// Duración (encoger, desvanecer y volver al icono).
    pub duration: f32,
    /// Escala final del conjunto al llegar.
    pub end_scale: f32,
}
