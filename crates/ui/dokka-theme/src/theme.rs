//! Para qué se usa cada color. Un tema es un valor de `Theme`.

use dokka_scene::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub dock: DockColors,
    pub menubar: MenubarColors,
    pub panel: PanelColors,
    pub tooltip: TooltipColors,
    pub media: MediaColors,
    pub menu: MenuColors,
    pub launchpad: LaunchpadColors,
    pub settings: SettingsColors,
    pub nav_pill: NavPillColors,
    pub tray: TrayColors,
    pub icons: IconColors,
}

#[derive(Debug, Clone, Copy)]
pub struct DockColors {
    pub background: Color,
    pub border: Color,
    pub placeholder: Color,
    pub divider: Color,
    pub running_dot: Color,
    /// Relleno claro de la línea cuando el dock está oculto.
    pub home_line: Color,
    /// Borde oscuro de la línea, que la recorta sobre fondos claros.
    pub home_line_border: Color,
}

/// Calendario y vistas previas.
#[derive(Debug, Clone, Copy)]
pub struct PanelColors {
    pub background: Color,
    pub border: Color,
    pub text: Color,
    pub highlight: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct TooltipColors {
    pub tint: Color,
    pub border: Color,
    pub text: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct MediaColors {
    pub background: Color,
    pub border: Color,
    pub icon: Color,
}

/// Colores de la barra superior: mismo cristal claro del dock,
/// texto oscuro y resaltado suave detrás del icono.
#[derive(Debug, Clone, Copy)]
pub struct MenubarColors {
    pub background: Color,
    pub border: Color,
    pub text: Color,
    pub hover: Color,
}

/// Colores del menú del clic derecho.
#[derive(Debug, Clone, Copy)]
pub struct MenuColors {
    pub background: Color,
    pub border: Color,
    pub text: Color,
    pub highlighted_text: Color,
    pub highlight: Color,
    pub separator: Color,
}

/// Colores del Launchpad a pantalla completa.
#[derive(Debug, Clone, Copy)]
pub struct LaunchpadColors {
    pub background: Color,
    pub text: Color,
    pub highlight: Color,
    pub search_background: Color,
    pub search_border: Color,
    pub search_text: Color,
    /// Marcador "Buscar" tenue junto al cursor cuando la casilla está enfocada y vacía.
    pub search_placeholder: Color,
    pub dot_active: Color,
    pub dot_inactive: Color,
    /// Baldosa clara de los iconos uniformes.
    pub tile: Color,
}

/// Colores de la píldora segmentada de navegación (calendario y Preferencias).
#[derive(Debug, Clone, Copy)]
pub struct NavPillColors {
    pub background: Color,
    pub border: Color,
    pub icon: Color,
    pub icon_disabled: Color,
    pub divider: Color,
    pub hover: Color,
}

/// Paneles de la barra superior: un solo cristal ahumado con texto blanco.
#[derive(Debug, Clone, Copy)]
pub struct TrayColors {
    /// Fondo transparente que agrupa las piezas.
    pub clear: Color,
    pub tile: Color,
    pub tile_border: Color,
    /// Tinte propio del Centro de control, mucho más claro que `tile`.
    pub tile_translucent: Color,
    pub text: Color,
    pub secondary: Color,
    pub track: Color,
    pub fill: Color,
    /// Círculo de un icono apagado y su icono.
    pub circle: Color,
    pub circle_icon: Color,
    /// Círculo activo (blanco con icono azul).
    pub circle_active: Color,
    pub circle_active_icon: Color,
    pub hover: Color,
    /// Cristal único oscuro de cada menú.
    pub panel: Color,
    /// Borde fino del cristal único, con filo claro.
    pub panel_border: Color,
    /// Línea fina entre secciones del cristal único.
    pub separator: Color,
    /// Número del volumen sobre el relleno blanco.
    pub percent: Color,
}

/// Colores de la ventana de Preferencias en tema claro.
#[derive(Debug, Clone, Copy)]
pub struct SettingsColors {
    pub window: Color,
    pub window_border: Color,
    pub sidebar: Color,
    pub sidebar_border: Color,
    pub search_background: Color,
    pub search_border: Color,
    pub search_text: Color,
    pub search_icon: Color,
    pub row_text: Color,
    pub row_text_selected: Color,
    pub selection: Color,
    pub hover: Color,
    pub title: Color,
    pub nav: Color,
    pub divider: Color,
    pub card: Color,
    pub card_border: Color,
    pub card_text: Color,
    /// Pista encendida del interruptor.
    pub switch_on: Color,
    /// Pista apagada del interruptor.
    pub switch_off: Color,
    /// Tirador del interruptor.
    pub switch_knob: Color,
    /// Barra del deslizador.
    pub slider_track: Color,
    /// Relleno del deslizador cuando responde.
    pub slider_fill: Color,
    /// Relleno del deslizador cuando está atenuado.
    pub slider_muted: Color,
    /// Marca del valor de fábrica en el deslizador.
    pub slider_tick: Color,
    /// Cápsula gris del segmentado de posición.
    pub segment_track: Color,
    /// Pastilla blanca del segmento elegido.
    pub segment_selected: Color,
    /// Cuadrado de color de cada sección, en su orden.
    pub badge: [Color; 8],
    /// Semáforos rojo, amarillo y verde, en ese orden.
    pub traffic: [Color; 3],
    /// Símbolos oscuros sobre cada semáforo al pasar el cursor, en su orden.
    pub traffic_symbol: [Color; 3],
    /// Gris de los tres semáforos cuando la ventana no tiene el foco.
    pub traffic_inactive: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct IconColors {
    pub explorer_background: Color,
    pub folder_back: Color,
    pub folder_front: Color,
    pub launchpad_background: Color,
    pub launchpad_squares: [Color; 9],
    pub calendar_background: Color,
    pub calendar_month: Color,
    pub calendar_day: Color,
    pub clock_background: Color,
    pub clock_face: Color,
    pub clock_ink: Color,
    pub clock_second_hand: Color,
    pub trash_body: Color,
    pub trash_rim: Color,
    pub trash_rib: Color,
    pub trash_paper: Color,
    pub trash_waste: Color,
}
