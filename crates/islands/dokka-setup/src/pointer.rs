//! Puntero de la isla: hover, pulsación y clic por control.

use dokka_scene::Rect;
use dokka_theme::installer as measures;

use super::state::Screen;
use super::{Control, Setup, SetupAction};

/// Rectángulo del interruptor de la fila, con el aire propio.
fn switch_rect(row: Rect) -> Rect {
    Rect {
        x: row.x + row.width - measures::ROW_PADDING - dokka_theme::settings::SWITCH_WIDTH,
        y: row.y + (row.height - dokka_theme::settings::SWITCH_HEIGHT) / 2.0,
        width: dokka_theme::settings::SWITCH_WIDTH,
        height: dokka_theme::settings::SWITCH_HEIGHT,
    }
}

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}

fn round_button() -> Rect {
    let (x, y, w, h) = measures::round_rect();
    Rect {
        x,
        y,
        width: w,
        height: h,
    }
}

fn primary_button() -> Rect {
    let (x, y, w, h) = measures::button_rect();
    Rect {
        x,
        y,
        width: w,
        height: h,
    }
}

fn link_hit() -> Rect {
    let (_, top, _, height) = measures::button_rect();
    let y = top + height + measures::LINK_GAP - measures::LINK_HIT_PAD;
    Rect {
        x: measures::center_x() - 120.0,
        y,
        width: 240.0,
        height: measures::LINK_TEXT_SIZE + measures::LINK_HIT_PAD * 2.0,
    }
}

fn card_rows() -> [Rect; 2] {
    let (x, y, w, _) = measures::card_rect();
    [
        Rect {
            x,
            y,
            width: w,
            height: measures::ROW_HEIGHT,
        },
        Rect {
            x,
            y: y + measures::ROW_HEIGHT,
            width: w,
            height: measures::ROW_HEIGHT,
        },
    ]
}

fn check_hit() -> Rect {
    Rect {
        x: measures::center_x() - 170.0,
        y: measures::CHECK_TOP - 4.0,
        width: 340.0,
        height: measures::CHECK_TEXT_SIZE + 16.0,
    }
}

/// Control bajo el punto en la pantalla actual, si hay alguno.
pub fn control_at(screen: Screen, x: f32, y: f32) -> Option<Control> {
    match screen {
        Screen::Installing | Screen::Uninstalling => None,
        Screen::Options => {
            if contains(round_button(), x, y) {
                return Some(Control::Round);
            }
            if contains(primary_button(), x, y) {
                return Some(Control::Primary);
            }
            let rows = card_rows();
            if contains(switch_rect(rows[0]), x, y) {
                return Some(Control::SwitchStart);
            }
            if contains(switch_rect(rows[1]), x, y) {
                return Some(Control::SwitchShortcut);
            }
            None
        }
        Screen::UninstallConfirm => {
            if contains(primary_button(), x, y) {
                return Some(Control::Primary);
            }
            if contains(link_hit(), x, y) {
                return Some(Control::Link);
            }
            if contains(check_hit(), x, y) {
                return Some(Control::CheckRemove);
            }
            None
        }
        Screen::Done | Screen::UninstallDone => {
            if contains(primary_button(), x, y) {
                return Some(Control::Primary);
            }
            None
        }
        Screen::Welcome | Screen::Error | Screen::UninstallError => {
            if contains(round_button(), x, y) {
                return Some(Control::Round);
            }
            if contains(primary_button(), x, y) {
                return Some(Control::Primary);
            }
            if contains(link_hit(), x, y) {
                return Some(Control::Link);
            }
            None
        }
    }
}

/// Acción del control pulsado en la pantalla actual.
fn action_of(screen: Screen, control: Control) -> Option<SetupAction> {
    match (screen, control) {
        (Screen::Welcome, Control::Primary) => Some(SetupAction::StartInstall),
        (Screen::Welcome, Control::Link) => Some(SetupAction::ShowOptions),
        (Screen::Welcome, Control::Round) => Some(SetupAction::Close),
        (Screen::Options, Control::Round) => Some(SetupAction::BackWelcome),
        (Screen::Options, Control::Primary) => Some(SetupAction::OptionsDone),
        (Screen::Options, Control::SwitchStart) => Some(SetupAction::ToggleStart),
        (Screen::Options, Control::SwitchShortcut) => Some(SetupAction::ToggleShortcut),
        (Screen::Done, Control::Primary) => Some(SetupAction::Launch),
        (Screen::Error, Control::Primary) => Some(SetupAction::Retry),
        (Screen::Error, Control::Round | Control::Link) => Some(SetupAction::Close),
        (Screen::UninstallConfirm, Control::Primary) => Some(SetupAction::StartUninstall),
        (Screen::UninstallConfirm, Control::Link) => Some(SetupAction::Close),
        (Screen::UninstallConfirm, Control::CheckRemove) => Some(SetupAction::ToggleRemove),
        (Screen::UninstallDone, Control::Primary) => Some(SetupAction::Close),
        (Screen::UninstallError, Control::Primary) => Some(SetupAction::Retry),
        (Screen::UninstallError, Control::Round | Control::Link) => Some(SetupAction::Close),
        _ => None,
    }
}

impl Setup {
    /// Resalta el control bajo el cursor.
    pub fn pointer_moved(&mut self, x: f32, y: f32) {
        self.set_hovered(control_at(self.screen(), x, y));
    }

    pub fn pointer_left(&mut self) {
        self.set_hovered(None);
    }

    /// Recuerda el control pulsado para la respuesta al pulsar.
    pub fn press(&mut self, x: f32, y: f32) {
        self.set_pressed(control_at(self.screen(), x, y));
    }

    /// Suelta el botón: si coincide con lo pulsado, emite su acción.
    pub fn release(&mut self, x: f32, y: f32) -> Option<SetupAction> {
        let pressed = self.pressed();
        self.set_pressed(None);
        let hit = control_at(self.screen(), x, y);
        if pressed == hit {
            if let Some(control) = hit {
                return action_of(self.screen(), control);
            }
        }
        None
    }

    /// `true` si el punto arrastra la ventana (fondo, no un control).
    pub fn drag_zone(&self, x: f32, y: f32) -> bool {
        control_at(self.screen(), x, y).is_none()
    }
}
