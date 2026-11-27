//! Lo que el usuario puede ajustar desde Preferencias.

use super::control_item::HiddenControls;

/// Límites de los multiplicadores del dock, sobre las medidas del tema (1.0).
pub const DOCK_ICON_SCALE_RANGE: (f32, f32) = (0.75, 1.5);
pub const DOCK_MAGNIFICATION_RANGE: (f32, f32) = (0.5, 1.5);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Preferences {
    pub dock: DockPreferences,
    pub menubar: MenubarPreferences,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DockPreferences {
    /// Multiplica el tamaño de los iconos del tema.
    pub icon_scale: f32,
    /// Lupa al pasar el cursor.
    pub magnification: bool,
    /// Multiplica cuánto amplía la lupa respecto al tema.
    pub magnification_strength: f32,
    /// Vistas previas de ventanas al pasar el cursor.
    pub previews: bool,
    /// Rebote del icono al abrir una app.
    pub animate_launch: bool,
    /// Punto bajo las apps abiertas.
    pub show_indicators: bool,
    /// El dock baja con una app a pantalla completa y queda la línea.
    pub auto_hide: bool,
    /// Borde de la pantalla donde vive el dock.
    pub position: DockPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DockPosition {
    Left,
    #[default]
    Bottom,
    Right,
}

impl Default for DockPreferences {
    fn default() -> Self {
        Self {
            icon_scale: 1.0,
            magnification: true,
            magnification_strength: 1.0,
            previews: true,
            animate_launch: true,
            show_indicators: true,
            auto_hide: false,
            position: DockPosition::Bottom,
        }
    }
}

/// Qué iconos muestra la parte derecha de la barra superior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenubarPreferences {
    pub volume: bool,
    pub network: bool,
    pub music: bool,
    pub language: bool,
    pub search: bool,
    pub control_center: bool,
    /// Iconos de las apps en segundo plano; apagado, Dokka no toca la bandeja.
    pub background_apps: bool,
    /// Orden de izquierda a derecha en la barra; cada icono aparece una vez.
    /// La hora no se ordena: va siempre al final.
    pub order: [MenubarItem; 7],
    /// Controles del centro ocultos; los demás se reacomodan sin huecos.
    pub hidden_controls: HiddenControls,
}

/// Un icono ordenable de la parte derecha de la barra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenubarItem {
    Volume,
    Network,
    Music,
    BackgroundApps,
    Language,
    Search,
    ControlCenter,
}

impl MenubarItem {
    /// Orden de fábrica, de izquierda a derecha.
    pub const FACTORY_ORDER: [Self; 7] = [
        Self::Volume,
        Self::Network,
        Self::Music,
        Self::BackgroundApps,
        Self::Language,
        Self::Search,
        Self::ControlCenter,
    ];
}

impl Default for MenubarPreferences {
    fn default() -> Self {
        Self {
            volume: true,
            network: true,
            music: true,
            language: true,
            search: true,
            control_center: true,
            background_apps: true,
            order: MenubarItem::FACTORY_ORDER,
            hidden_controls: HiddenControls::empty(),
        }
    }
}
