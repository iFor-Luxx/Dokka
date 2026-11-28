//! Fundido de salida del centro de control: al cerrarse no desaparece de
//! golpe, se desvanece en `DISMISS_SECONDS` encogiéndose un poco hacia su
//! icono de la barra. Vale para la inactividad, el clic fuera y Escape.
//!
//! La isla guarda el progreso (`Reveal` de `dokka-animation`, el mismo que
//! las vistas previas); cada fotograma avanza y, al terminar, hace el cierre
//! real de siempre. Mientras dura no acepta clics dentro del panel; si el
//! usuario vuelve a abrirlo o abre otro panel, se cancela limpiamente. Con la
//! galería abierta se cierra al instante como hoy, que es lo más simple.

use dokka_animation::Reveal;
use dokka_theme::control_center as measures;

use super::TrayKind;
use super::click::MenubarClick;
use crate::Menubar;

impl Menubar {
    /// `true` si el centro se está desvaneciendo: el shell pide fotogramas.
    pub fn is_control_closing(&self) -> bool {
        self.control_dismiss.is_some()
    }

    /// Progreso del fundido para la huella: cambia en cada fotograma y pide
    /// repintar. `None` sin cierre en curso.
    pub(crate) fn control_dismiss_progress(&self) -> Option<f32> {
        self.control_dismiss.as_ref().map(Reveal::progress)
    }

    /// Empieza a cerrar el centro: fundido si está abierto sin galería,
    /// cierre al instante con galería como hoy. Devuelve si hay fundido en
    /// curso (el panel sigue abierto). Sin centro no hace nada.
    pub fn begin_control_close(&mut self) -> bool {
        if self.tray_kind() != Some(TrayKind::ControlCenter) {
            return false;
        }
        if self.control_dismiss.is_some() {
            return true;
        }
        if self.is_gallery_open() {
            self.close_tray();
            return false;
        }
        let mut reveal = Reveal::visible();
        reveal.start_exit();
        self.control_dismiss = Some(reveal);
        self.dragging_slider = false;
        self.dragging_brightness = false;
        self.control_idle_ticks = 0;
        self.control_cursor_inside = false;
        true
    }

    /// Cancela el fundido: el centro vuelve a estar abierto del todo.
    pub(crate) fn cancel_control_close(&mut self) {
        self.control_dismiss = None;
    }

    /// Avanza un fotograma del fundido; al terminar hace el cierre real de
    /// siempre. Devuelve `true` si terminó en este fotograma.
    pub fn advance_control_frame(&mut self, seconds: f32) -> bool {
        let Some(reveal) = self.control_dismiss.as_mut() else {
            return false;
        };
        reveal.advance(
            seconds,
            measures::DISMISS_SECONDS,
            measures::DISMISS_SECONDS,
        );
        if reveal.has_exited() {
            self.close_tray();
            return true;
        }
        false
    }

    /// Aplica el fundido al panel del centro: opacidad y escala con origen
    /// arriba en el centro, hacia su icono de la barra. Sin cierre en curso
    /// lo deja como hoy (la apertura no anima).
    pub(crate) fn apply_control_dismiss(&self, overlay: &mut dokka_scene::Overlay) {
        let Some(reveal) = &self.control_dismiss else {
            return;
        };
        let Some(bounds) = self.tray_bounds() else {
            return;
        };
        overlay.opacity = reveal.fade(measures::DISMISS_OPACITY_SPAN);
        overlay.scale = reveal.scale(1.0, measures::DISMISS_END_SCALE);
        overlay.origin = (bounds.x + bounds.width / 2.0, bounds.y);
    }

    /// Clic con el fundido en curso. Dentro del panel se ignora; repetir su
    /// icono lo vuelve a abrir (se cancela y se queda); otro icono, el icono,
    /// la hora o fuera cancela y sigue el camino normal, o se ignora si es
    /// fuera. `None` si no hay fundido y no se hace nada.
    pub(crate) fn control_closing_click(
        &mut self,
        x: f32,
        y: f32,
        bar_width: f32,
    ) -> Option<MenubarClick> {
        if !self.is_control_closing() {
            return None;
        }
        if self.tray_contains(x, y) {
            return Some(MenubarClick::TrayKept);
        }
        if self.tray_kind_at(bar_width, x, y) == Some(TrayKind::ControlCenter) {
            self.cancel_control_close();
            self.tray_hovered = Some(TrayKind::ControlCenter);
            return Some(MenubarClick::TrayKept);
        }
        if self.tray_kind_at(bar_width, x, y).is_some()
            || contains(self.icon_hit(), x, y)
            || contains(self.clock_hit(bar_width), x, y)
        {
            self.cancel_control_close();
            return None;
        }
        Some(MenubarClick::TrayKept)
    }

    /// Cola del clic con la fila abierta: repetir el icono cierra (el centro
    /// con fundido) y otro cambia de panel; el icono y la hora cambian al menú
    /// y al calendario; fuera cierra (el centro con fundido).
    pub(crate) fn click_tray_tail(
        &mut self,
        x: f32,
        y: f32,
        bar_width: f32,
        screen_height: f32,
    ) -> MenubarClick {
        if let Some(kind) = self.tray_kind_at(bar_width, x, y) {
            if self.tray_kind() == Some(kind) {
                if kind == TrayKind::ControlCenter && self.begin_control_close() {
                    self.tray_hovered = Some(kind);
                    return MenubarClick::Dismissed;
                }
                self.close_tray();
                self.tray_hovered = Some(kind);
                return MenubarClick::Dismissed;
            }
            self.open_tray(kind, bar_width, screen_height);
            return MenubarClick::TrayOpened;
        }
        if contains(self.icon_hit(), x, y) {
            self.close_tray();
            self.open_at(bar_width);
            return MenubarClick::MenuOpened;
        }
        if contains(self.clock_hit(bar_width), x, y) {
            self.close_tray();
            self.open_calendar();
            return MenubarClick::CalendarOpened;
        }
        if self.tray_kind() == Some(TrayKind::ControlCenter) && self.begin_control_close() {
            return MenubarClick::Dismissed;
        }
        self.close_tray();
        self.icon_hovered = false;
        self.clock_hovered = false;
        MenubarClick::Dismissed
    }
}

fn contains(area: dokka_scene::Rect, x: f32, y: f32) -> bool {
    x >= area.x && x <= area.x + area.width && y >= area.y && y <= area.y + area.height
}
