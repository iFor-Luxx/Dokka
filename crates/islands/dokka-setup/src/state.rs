//! Pantalla, opciones, avance y fundido de la isla.

use dokka_theme::installer as measures;

use super::WorkStep;

/// Pantalla visible de la ventana.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    Options,
    Installing,
    Done,
    Error,
    UninstallConfirm,
    Uninstalling,
    UninstallDone,
    UninstallError,
}

/// Lo ya dibujado, para que la app solo repinte si cambió algo.
#[derive(Debug, Clone, PartialEq)]
pub struct PaintedSetup {
    pub screen: Screen,
    pub start_with_windows: bool,
    pub desktop_shortcut: bool,
    pub remove_settings: bool,
    pub step: Option<(WorkStep, PaintedF32)>,
    pub error_title: String,
    pub error_detail: String,
    pub hovered: Option<super::Control>,
    pub pressed: Option<super::Control>,
}

/// Fracción del avance con comparación exacta para el repintado.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaintedF32(pub f32);

/// Estado de la ventana del instalador.
#[derive(Debug)]
pub struct Setup {
    uninstall: bool,
    screen: Screen,
    start_with_windows: bool,
    desktop_shortcut: bool,
    remove_settings: bool,
    step: Option<(WorkStep, f32)>,
    error_title: String,
    error_detail: String,
    hovered: Option<super::Control>,
    pressed: Option<super::Control>,
    fade: f32,
}

impl Setup {
    /// `uninstall` es `true` cuando el exe se abrió con `--uninstall`.
    pub fn new(uninstall: bool) -> Self {
        Self {
            uninstall,
            screen: if uninstall {
                Screen::UninstallConfirm
            } else {
                Screen::Welcome
            },
            start_with_windows: true,
            desktop_shortcut: true,
            remove_settings: false,
            step: None,
            error_title: String::new(),
            error_detail: String::new(),
            hovered: None,
            pressed: None,
            fade: 0.0,
        }
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn start_with_windows(&self) -> bool {
        self.start_with_windows
    }

    pub fn desktop_shortcut(&self) -> bool {
        self.desktop_shortcut
    }

    pub fn remove_settings(&self) -> bool {
        self.remove_settings
    }

    pub fn step(&self) -> Option<(WorkStep, f32)> {
        self.step
    }

    pub fn error_title(&self) -> &str {
        &self.error_title
    }

    pub fn error_detail(&self) -> &str {
        &self.error_detail
    }

    pub fn hovered(&self) -> Option<super::Control> {
        self.hovered
    }

    pub fn pressed(&self) -> Option<super::Control> {
        self.pressed
    }

    /// Opacidad del contenido: 0 al cambiar de pantalla hasta 1 en reposo.
    pub fn fade(&self) -> f32 {
        self.fade
    }

    /// `true` mientras trabaja: sin botón de cerrar y no se puede cerrar.
    pub fn is_working(&self) -> bool {
        matches!(self.screen, Screen::Installing | Screen::Uninstalling)
    }

    fn go(&mut self, screen: Screen) {
        self.screen = screen;
        self.hovered = None;
        self.pressed = None;
        self.fade = 0.0;
    }

    pub fn show_options(&mut self) {
        self.go(Screen::Options);
    }

    pub fn back_welcome(&mut self) {
        self.go(Screen::Welcome);
    }

    pub fn options_done(&mut self) {
        self.go(Screen::Welcome);
    }

    pub fn toggle_start(&mut self) {
        self.start_with_windows = !self.start_with_windows;
    }

    pub fn toggle_shortcut(&mut self) {
        self.desktop_shortcut = !self.desktop_shortcut;
    }

    pub fn toggle_remove(&mut self) {
        self.remove_settings = !self.remove_settings;
    }

    pub fn start_install(&mut self) {
        self.step = None;
        self.go(Screen::Installing);
    }

    pub fn start_uninstall(&mut self) {
        self.step = None;
        self.go(Screen::Uninstalling);
    }

    pub fn finish(&mut self) {
        if self.screen == Screen::Installing {
            self.go(Screen::Done);
        } else if self.screen == Screen::Uninstalling {
            self.go(Screen::UninstallDone);
        }
    }

    /// Guarda el fallo y pasa a la pantalla de error que corresponda.
    pub fn fail(&mut self, title: &str, detail: &str) {
        self.error_title = title.to_owned();
        self.error_detail = detail.to_owned();
        self.step = None;
        if self.screen == Screen::Uninstalling {
            self.go(Screen::UninstallError);
        } else {
            self.go(Screen::Error);
        }
    }

    pub fn retry(&mut self) {
        self.step = None;
        if self.screen == Screen::UninstallError {
            self.go(Screen::Uninstalling);
        } else {
            self.go(Screen::Installing);
        }
    }

    /// Fija el paso y la fracción (0 a 1) que avisa la app.
    pub fn set_progress(&mut self, step: WorkStep, fraction: f32) {
        self.step = Some((step, fraction.clamp(0.0, 1.0)));
    }

    pub(crate) fn set_hovered(&mut self, hovered: Option<super::Control>) {
        self.hovered = hovered;
    }

    pub(crate) fn set_pressed(&mut self, pressed: Option<super::Control>) {
        self.pressed = pressed;
    }

    pub(crate) fn advance_fade(&mut self, seconds: f32) {
        if self.fade < 1.0 {
            self.fade = (self.fade + seconds / measures::FADE_SECONDS).min(1.0);
        }
    }

    /// Lo dibujado, para repintar solo si cambió.
    pub fn painted(&self) -> PaintedSetup {
        PaintedSetup {
            screen: self.screen,
            start_with_windows: self.start_with_windows,
            desktop_shortcut: self.desktop_shortcut,
            remove_settings: self.remove_settings,
            step: self.step.map(|(step, f)| (step, PaintedF32(f))),
            error_title: self.error_title.clone(),
            error_detail: self.error_detail.clone(),
            hovered: self.hovered,
            pressed: self.pressed,
        }
    }

    /// `true` si se abrió para desinstalar.
    pub fn is_uninstall(&self) -> bool {
        self.uninstall
    }
}
