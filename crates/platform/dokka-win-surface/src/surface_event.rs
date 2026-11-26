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
    /// El menú debe cerrarse: se perdió el foco o se pulsó Esc.
    Dismissed,
    /// El cursor se mueve sobre el Launchpad.
    LaunchpadMoved {
        x: f32,
        y: f32,
    },
    LaunchpadLeft,
    LaunchpadClicked {
        x: f32,
        y: f32,
    },
    /// El Launchpad perdió el foco y debe cerrarse.
    LaunchpadDismissed,
    /// Letra escrita sobre el Launchpad, ya traducida de `WM_KEYDOWN`.
    LaunchpadTyped {
        character: char,
    },
    /// Retroceso sobre el Launchpad.
    LaunchpadBackspace,
    /// Enter sobre el Launchpad.
    LaunchpadEnter,
    /// Esc sobre el Launchpad: limpia la búsqueda antes de cerrar.
    LaunchpadEscape,
    /// Rueda sobre el Launchpad: cambia de página.
    LaunchpadWheel {
        delta: i16,
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
