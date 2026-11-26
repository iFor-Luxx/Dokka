//! Para qué se usa cada color. Un tema es un valor de `Theme`.

use dokka_scene::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub dock: DockColors,
    pub panel: PanelColors,
    pub tooltip: TooltipColors,
    pub media: MediaColors,
    pub menu: MenuColors,
    pub launchpad: LaunchpadColors,
    pub icons: IconColors,
}

#[derive(Debug, Clone, Copy)]
pub struct DockColors {
    pub background: Color,
    pub border: Color,
    pub placeholder: Color,
    pub divider: Color,
    pub running_dot: Color,
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
