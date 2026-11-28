//! Salida del centro de control hacia su icono, descrita una sola vez.
//!
//! La isla decide cuándo empieza (al cerrarse por inactividad, clic fuera,
//! Escape o repetir su icono) y cuándo termina (cierre real tras el tope
//! total). La plataforma crea y lanza las animaciones de Composition por
//! pieza con estos datos, sin que Dokka recalcule cada fotograma.

/// Destino y tiempos de la salida por piezas, en unidades lógicas y segundos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlDismiss {
    /// Punto del icono del centro en la barra, al que vuela cada pieza.
    pub target: (f32, f32),
    /// Retardo entre piezas, de la más cercana a la más lejana (~0,025 s).
    pub stagger: f32,
    /// Duración de cada pieza (encoger, desvanecer y volar).
    pub duration: f32,
    /// Escala final de cada pieza al llegar al icono.
    pub end_scale: f32,
}
