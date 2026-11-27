use super::light_icons::light_icons;
use crate::palette;
use crate::{
    DockColors, LaunchpadColors, MediaColors, MenuColors, MenubarColors, NavPillColors,
    PanelColors, SettingsColors, Theme, TooltipColors, TrayColors,
};

/// Tema claro de cristal: el actual de Dokka.
pub fn light() -> Theme {
    let border = palette::white(140);
    Theme {
        dock: DockColors {
            background: palette::white(40),
            border: palette::white(96),
            placeholder: palette::white(80),
            divider: palette::white(110),
            running_dot: palette::white(220),
        },
        menubar: MenubarColors {
            background: palette::MENUBAR_VEIL,
            border: palette::MENUBAR_VEIL,
            text: palette::white(255),
            hover: palette::white(45),
        },
        panel: PanelColors {
            background: palette::MIST,
            border,
            text: palette::INK,
            highlight: palette::white(245),
        },
        tooltip: TooltipColors {
            tint: palette::TOOLTIP_TINT,
            border: palette::TOOLTIP_BORDER,
            text: palette::INK,
        },
        media: MediaColors {
            background: palette::MIST_STRONG,
            border,
            icon: palette::INK,
        },
        menu: MenuColors {
            background: palette::MENU_BACKGROUND,
            border: palette::MENU_BORDER,
            text: palette::INK,
            highlighted_text: palette::white(255),
            highlight: palette::MACOS_BLUE,
            separator: palette::MENU_SEPARATOR,
        },
        launchpad: LaunchpadColors {
            background: palette::LAUNCHPAD_BACKGROUND,
            text: palette::LAUNCHPAD_TEXT,
            highlight: palette::LAUNCHPAD_HIGHLIGHT,
            search_background: palette::LAUNCHPAD_SEARCH_BACKGROUND,
            search_border: palette::LAUNCHPAD_SEARCH_BORDER,
            search_text: palette::LAUNCHPAD_SEARCH_TEXT,
            search_placeholder: palette::LAUNCHPAD_SEARCH_PLACEHOLDER,
            dot_active: palette::LAUNCHPAD_DOT_ACTIVE,
            dot_inactive: palette::LAUNCHPAD_DOT_INACTIVE,
            tile: palette::APP_TILE,
        },
        icons: light_icons(),
        nav_pill: NavPillColors {
            background: palette::NAV_PILL_BACKGROUND,
            border: palette::NAV_PILL_BORDER,
            icon: palette::INK,
            icon_disabled: palette::NAV_PILL_ICON_DISABLED,
            divider: palette::NAV_PILL_DIVIDER,
            hover: palette::NAV_PILL_HOVER,
        },
        // Mismo cristal claro que el dock: el color lo pone el fondo. De las
        // referencias de Tahoe se toma la forma, no el tono.
        tray: TrayColors {
            clear: palette::TRAY_CLEAR,
            tile: palette::TRAY_TILE,
            tile_border: border,
            tile_translucent: palette::TRAY_TILE_TRANSLUCENT,
            text: palette::INK,
            secondary: palette::TRAY_SECONDARY,
            track: palette::TRAY_TRACK,
            fill: palette::white(255),
            circle: palette::TRAY_CIRCLE,
            circle_icon: palette::INK,
            circle_active: palette::MACOS_BLUE,
            circle_active_icon: palette::white(255),
            hover: palette::SETTINGS_HOVER,
            panel: palette::MIST,
            panel_border: border,
            separator: palette::TRAY_SEPARATOR,
            percent: palette::TRAY_PERCENT,
        },
        settings: settings_colors(border),
    }
}

/// Colores de la ventana de Preferencias: cristal claro con la selección azul.
fn settings_colors(border: dokka_scene::Color) -> SettingsColors {
    SettingsColors {
        window: palette::SETTINGS_WINDOW,
        window_border: border,
        sidebar: palette::SETTINGS_SIDEBAR,
        sidebar_border: palette::SETTINGS_SIDEBAR_BORDER,
        search_background: palette::SETTINGS_SEARCH_BACKGROUND,
        search_border: palette::SETTINGS_SEARCH_BORDER,
        search_text: palette::GRAY,
        search_icon: palette::GRAY,
        row_text: palette::INK,
        row_text_selected: palette::white(255),
        selection: palette::MACOS_BLUE,
        hover: palette::SETTINGS_HOVER,
        title: palette::INK,
        nav: palette::GRAY,
        divider: palette::SETTINGS_DIVIDER,
        card: palette::SETTINGS_CARD,
        card_border: palette::SETTINGS_SEARCH_BORDER,
        card_text: palette::GRAY,
        switch_on: palette::SETTINGS_SWITCH_ON,
        switch_off: palette::SETTINGS_SWITCH_OFF,
        switch_knob: palette::SETTINGS_SWITCH_KNOB,
        slider_track: palette::SETTINGS_SLIDER_TRACK,
        slider_fill: palette::SETTINGS_SLIDER_FILL,
        slider_muted: palette::SETTINGS_SLIDER_MUTED,
        badge: [
            palette::GRAY,
            palette::BLUE,
            palette::PURPLE,
            palette::PINK,
            palette::TEAL,
            palette::GREEN,
            palette::ORANGE,
            palette::RED,
        ],
        traffic: [palette::RED, palette::YELLOW, palette::GREEN],
        traffic_symbol: [
            palette::TRAFFIC_CLOSE_SYMBOL,
            palette::TRAFFIC_MINIMIZE_SYMBOL,
            palette::TRAFFIC_ZOOM_SYMBOL,
        ],
        traffic_inactive: palette::SETTINGS_TRAFFIC_INACTIVE,
    }
}
