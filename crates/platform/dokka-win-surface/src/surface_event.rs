/// Algo que la ventana comunica a quien la usa. Coordenadas lógicas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurfaceEvent {
    PointerMoved {
        x: f32,
        y: f32,
    },
    PointerLeft,
    Clicked {
        x: f32,
        y: f32,
        with_shift: bool,
    },
    RightClicked {
        x: f32,
        y: f32,
    },
    /// Llegó el aviso de actualización pedido en `run`.
    Refresh,
    /// Pasó un segundo.
    Tick,
    /// Nuevo fotograma de una animación en curso.
    Frame {
        seconds: f32,
    },
}
