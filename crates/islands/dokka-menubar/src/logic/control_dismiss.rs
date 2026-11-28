//! Salida del centro de control por piezas hacia su icono de la barra.
//!
//! Al cerrarse (inactividad, clic fuera, Escape o repetir su icono) no
//! desaparece de golpe: cada píldora, círculo y tarjeta vuela por separado
//! hacia el icono, encogiéndose y desvaneciéndose, escalonadas desde la más
//! cercana (~25 ms entre una y otra, tope ~0,45 s), estilo macOS Tahoe.
//!
//! La isla solo guarda el destino y el tiempo para el cierre real; las
//! anima el motor de Composition de Windows con la orden de `dokka-scene`
//! (`ControlDismiss`) y los tiempos de `dokka-theme`, sin recalcular cada
//! fotograma desde Dokka. Mientras dura no acepta clics dentro del panel;
//! si el usuario vuelve a abrirlo o abre otro panel, se cancela limpiamente.
//! Con la galería abierta se cierra al instante como hoy.

use dokka_theme::control_center as measures;

use super::TrayKind;
use super::click::MenubarClick;
use crate::Menubar;

/// Cierre en curso: destino en la barra y tiempo desde que empezó.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ClosingCenter {
    target: (f32, f32),
    elapsed: f32,
}

impl Menubar {
    /// `true` si el centro se está yendo por piezas: el shell mantiene los
    /// fotogramas para el cierre real, sin repintar la capa.
    pub fn is_control_closing(&self) -> bool {
        self.control_dismiss.is_some()
    }

    /// Orden de salida para la plataforma, una sola vez: destino y tiempos.
    /// `None` sin cierre en curso.
    pub fn dismiss_order(&self) -> Option<dokka_scene::ControlDismiss> {
        self.control_dismiss
            .map(|closing| dokka_scene::ControlDismiss {
                target: closing.target,
                stagger: measures::DISMISS_STAGGER_SECONDS,
                duration: measures::DISMISS_PIECE_SECONDS,
                end_scale: measures::DISMISS_END_SCALE,
            })
    }

    /// Empieza a cerrar el centro: por piezas si está abierto sin galería,
    /// cierre al instante con galería como hoy. Devuelve si hay salida en
    /// curso (el panel sigue abierto). Sin centro no hace nada.
    pub fn begin_control_close(&mut self, bar_width: f32) -> bool {
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
        self.control_dismiss = Some(ClosingCenter {
            target: self.control_icon_target(bar_width),
            elapsed: 0.0,
        });
        self.dragging_slider = false;
        self.dragging_brightness = false;
        self.control_idle_ticks = 0;
        self.control_cursor_inside = false;
        true
    }

    /// Punto del icono del centro en la barra, al que vuela cada pieza.
    /// Sin su caja (no debería pasar), arriba en el centro del panel.
    fn control_icon_target(&self, bar_width: f32) -> (f32, f32) {
        if let Some(anchor) = self.tray_anchor(TrayKind::ControlCenter, bar_width) {
            return (
                anchor.x + anchor.width / 2.0,
                dokka_theme::menubar::HEIGHT / 2.0,
            );
        }
        if let Some(bounds) = self.tray_bounds() {
            return (bounds.x + bounds.width / 2.0, bounds.y);
        }
        (bar_width / 2.0, dokka_theme::menubar::HEIGHT)
    }

    /// Cancela la salida: el centro vuelve a estar abierto del todo.
    /// El shell descarta las animaciones y reconstruye limpio.
    pub(crate) fn cancel_control_close(&mut self) {
        self.control_dismiss = None;
    }

    /// Avanza el tiempo hacia el cierre real de siempre. Devuelve `true`
    /// si terminó en este fotograma. No toca opacidad, escala ni posición:
    /// las anima Composition.
    pub fn advance_control_frame(&mut self, seconds: f32) -> bool {
        let Some(closing) = self.control_dismiss.as_mut() else {
            return false;
        };
        closing.elapsed += seconds;
        if closing.elapsed >= measures::DISMISS_TOTAL_SECONDS {
            self.close_tray();
            return true;
        }
        false
    }

    /// Clic con la salida en curso. Dentro del panel se ignora; repetir su
    /// icono lo vuelve a abrir (se cancela y se queda); otro icono, el icono,
    /// la hora o fuera cancela y sigue el camino normal, o se ignora si es
    /// fuera. `None` si no hay salida y no se hace nada.
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
    /// con salida por piezas) y otro cambia de panel; el icono y la hora
    /// cambian al menú y al calendario; fuera cierra (el centro con salida).
    pub(crate) fn click_tray_tail(
        &mut self,
        x: f32,
        y: f32,
        bar_width: f32,
        screen_height: f32,
    ) -> MenubarClick {
        if let Some(kind) = self.tray_kind_at(bar_width, x, y) {
            if self.tray_kind() == Some(kind) {
                if kind == TrayKind::ControlCenter && self.begin_control_close(bar_width) {
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
        if self.tray_kind() == Some(TrayKind::ControlCenter) && self.begin_control_close(bar_width)
        {
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
