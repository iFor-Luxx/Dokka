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
    /// El cursor se mueve sobre la barra superior.
    MenubarMoved {
        x: f32,
        y: f32,
    },
    MenubarLeft,
    /// Botón abajo sobre la barra: empieza un posible arrastre.
    MenubarPressed {
        x: f32,
        y: f32,
    },
    MenubarClicked {
        x: f32,
        y: f32,
    },
    /// La barra perdió el foco con el menú abierto.
    /// Botón derecho soltado sobre la barra.
    MenubarRightClicked {
        x: f32,
        y: f32,
    },
    MenubarDismissed,
    /// Letra escrita sobre la barra, ya traducida de `WM_KEYDOWN`.
    /// Solo llega cuando la barra tiene el foco.
    MenubarTyped {
        character: char,
    },
    /// Retroceso sobre la barra. Solo con foco.
    MenubarBackspace,
    /// Enter sobre la barra. Solo con foco.
    MenubarEnter,
    /// Flecha arriba sobre la barra. Solo con foco.
    MenubarUp,
    /// Flecha abajo sobre la barra. Solo con foco.
    MenubarDown,
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
    /// Flecha izquierda sobre el Launchpad: retrocede de página.
    LaunchpadPrevPage,
    /// Flecha derecha sobre el Launchpad: avanza de página.
    LaunchpadNextPage,
    /// Esc sobre el Launchpad: limpia la búsqueda antes de cerrar.
    LaunchpadEscape,
    /// Rueda sobre el Launchpad: cambia de página.
    LaunchpadWheel {
        delta: i16,
    },
    /// El cursor se mueve sobre Preferencias.
    SettingsMoved {
        x: f32,
        y: f32,
    },
    SettingsLeft,
    /// Botón abajo sobre Preferencias: empieza un posible arrastre.
    SettingsPressed {
        x: f32,
        y: f32,
    },
    SettingsClicked {
        x: f32,
        y: f32,
    },
    /// Preferencias se cerró con la X, Alt+F4 o la barra de tareas.
    SettingsDismissed,
    /// Preferencias ganó el foco: los semáforos recuperan su color.
    SettingsFocused,
    /// Preferencias perdió el foco: los semáforos se ven grises.
    SettingsBlurred,
    /// Letra escrita sobre Preferencias, ya traducida de `WM_KEYDOWN`.
    SettingsTyped {
        character: char,
    },
    /// Retroceso sobre Preferencias.
    SettingsBackspace,
    /// Enter sobre Preferencias.
    SettingsEnter,
    /// Esc sobre Preferencias: limpia la búsqueda antes de quitar el foco.
    SettingsEscape,
    /// Llegó el aviso de actualización pedido en `run`.
    Refresh,
    /// Una ventana empezó a minimizarse: trae su handle sin agrupar.
    /// Vía de Muelle (MIT): avisa la ventana concreta de inmediato.
    MinimizeStarted {
        window: u64,
    },
    /// Pasó un segundo.
    Tick,
    /// El cursor debe seguir sobre el panel abierto o pendiente.
    CursorCheck,
    /// Nuevo fotograma de una animación en curso.
    Frame {
        seconds: f32,
    },
}
