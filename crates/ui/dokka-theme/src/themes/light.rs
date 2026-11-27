use super::light_icons::light_icons;
use crate::palette;
use crate::{
    DockColors, LaunchpadColors, MediaColors, MenuColors, MenubarColors, NavPillColors,
    PanelColors, SettingsColors, Theme, TooltipColors,
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
            background: palette::white(40),
            border: palette::white(96),
            text: palette::INK,
            hover: palette::SETTINGS_HOVER,
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
        badge: [
            palette::GRAY,
            palette::BLUE,
            palette::PURPLE,
            palette::TEAL,
            palette::GREEN,
            palette::ORANGE,
            palette::RED,
        ],
        traffic: [palette::RED, palette::YELLOW, palette::GREEN],
    }
}
