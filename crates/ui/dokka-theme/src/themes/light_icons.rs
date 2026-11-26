use crate::IconColors;
use crate::palette;

pub(super) fn light_icons() -> IconColors {
    IconColors {
        explorer_background: palette::BLUE,
        folder_back: palette::SKY,
        folder_front: palette::SNOW,
        launchpad_background: palette::SILVER,
        launchpad_squares: [
            palette::GREEN,
            palette::YELLOW,
            palette::ORANGE,
            palette::RED,
            palette::GRAY,
            palette::PINK,
            palette::PURPLE,
            palette::BLUE,
            palette::TEAL,
        ],
        calendar_background: palette::SNOW,
        calendar_month: palette::RED,
        calendar_day: palette::INK,
        clock_background: palette::CHARCOAL,
        clock_face: palette::SNOW,
        clock_ink: palette::INK,
        clock_second_hand: palette::ORANGE,
        trash_body: palette::SILVER_SOFT,
        trash_rim: palette::STEEL,
        trash_rib: palette::STEEL_SOFT,
        trash_paper: palette::SNOW,
        trash_waste: palette::BLUE,
    }
}
