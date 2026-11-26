use super::light_icons::light_icons;
use crate::palette;
use crate::{
    DockColors, LaunchpadColors, MediaColors, MenuColors, PanelColors, Theme, TooltipColors,
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
        },
        icons: light_icons(),
    }
}
