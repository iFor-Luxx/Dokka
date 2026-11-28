/// Lo que la ventana del instalador comunica a quien la usa.
/// Coordenadas lógicas de contenido (sin el aire de la sombra).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallerEvent {
    PointerMoved {
        x: f32,
        y: f32,
    },
    PointerLeft,
    /// Botón abajo: empieza un posible arrastre desde el fondo.
    Pressed {
        x: f32,
        y: f32,
    },
    Clicked {
        x: f32,
        y: f32,
    },
    /// La X, Alt+F4 o la barra de tareas: la app decide si cierra.
    Dismissed,
    /// Aviso de actualización pedido en `run`.
    Refresh,
    /// Pasó un segundo.
    Tick,
    /// Nuevo fotograma de una animación en curso.
    Frame {
        seconds: f32,
    },
}
