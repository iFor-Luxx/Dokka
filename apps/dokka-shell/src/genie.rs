//! Puente del efecto genio: detecta minimizar y restaurar y pide los vuelos.
//! La vía inmediata (minimizar con el hwnd) pone la foto fija antes de
//! sincronizar; el aviso agrupado (`Refresh`) sigue valiendo para restaurar
//! por fuera y como red de seguridad. El destino lo da el dock en sus
//! coordenadas y aquí pasa a píxeles de pantalla. Si algo falla, la ventana
//! sigue su curso normal.
//! Recibe las piezas por separado para no pedir prestado el `DockSession`
//! entero mientras se pide prestado su campo `genie`.

use std::collections::HashMap;
use std::time::Instant;

use dokka_dock::Dock;
use dokka_model::{DockApp, DockPosition, WindowId};
use dokka_win_control::{foreground_window, window_rect};
use dokka_win_genie::{dock_slot_to_screen, monitor_rect_for_window, rect_before_minimize};
use dokka_win_surface::Surface;

/// Estado del genio entre eventos. Solo el shell lo posee.
pub(crate) struct GenieBridge {
    director: dokka_win_genie::GenieDirector,
    last_minimized: Vec<isize>,
    /// Último rectángulo visto por ventana no minimizada, para volar desde
    /// donde estaba (la ventana ya no lo dice cuando avisa).
    last_rects: HashMap<isize, [i32; 4]>,
    started: bool,
    settings_window: Option<u64>,
}

impl GenieBridge {
    pub(crate) fn new() -> Self {
        Self {
            director: dokka_win_genie::GenieDirector::new(),
            last_minimized: Vec::new(),
            last_rects: HashMap::new(),
            started: false,
            settings_window: None,
        }
    }

    /// Enciende o apaga el efecto con la preferencia del dock.
    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.director.set_enabled(enabled);
    }

    /// Recuerda la ventana del dock para pasar el vuelo tras su cristal.
    pub(crate) fn set_dock_window(&mut self, raw: isize) {
        self.director.set_dock_window(raw);
    }

    /// Recuerda la ventana propia de Preferencias para no animarla.
    pub(crate) fn set_settings_window(&mut self, raw: u64) {
        self.settings_window = Some(raw);
    }

    /// `true` mientras hay un vuelo dibujandose.
    pub(crate) fn is_flying(&self) -> bool {
        self.director.is_flying()
    }

    /// Proporción de la foto guardada para esta ventana, si la hay.
    pub(crate) fn snapshot_aspect(&self, hwnd: isize) -> Option<f32> {
        self.director.snapshot_aspect(hwnd)
    }

    fn is_settings(&self, raw: u64) -> bool {
        self.settings_window.is_some_and(|known| known == raw)
    }

    /// Guarda lo último visto de cada ventana abierta (no minimizada).
    pub(crate) fn remember_rects(&mut self, apps: &[DockApp]) {
        let mut open: Vec<isize> = Vec::new();
        for app in apps {
            for window in &app.windows {
                if window.is_minimized {
                    continue;
                }
                let raw = window.id.value() as isize;
                open.push(raw);
                if let Some(rect) = window_rect(window.id) {
                    self.last_rects.insert(raw, rect);
                }
            }
        }
        // Las cerradas no vuelven: Windows reutiliza sus handles.
        self.last_rects.retain(|known, _| open.contains(known));
    }

    /// Calma las ventanas abiertas para sustituir su animacion propia.
    fn calm_open(&mut self, apps: &[DockApp]) {
        let mut windows: Vec<isize> = Vec::new();
        for app in apps {
            for window in &app.windows {
                if self.is_settings(window.id.value()) {
                    continue;
                }
                windows.push(window.id.value() as isize);
            }
        }
        self.director.calm_windows(&windows);
    }

    /// Minimizada ahora segun las apps ya sincronizadas.
    fn current_minimized(apps: &[DockApp]) -> Vec<isize> {
        let mut minimized: Vec<isize> = Vec::new();
        for app in apps {
            for window in &app.windows {
                if window.is_minimized {
                    minimized.push(window.id.value() as isize);
                }
            }
        }
        minimized
    }

    /// Tras sincronizar: detecta nuevas minimizadas y restaura por fuera.
    pub(crate) fn on_refresh(
        &mut self,
        dock: &Dock,
        apps: &[DockApp],
        position: DockPosition,
        surface: &Surface,
    ) {
        let minimized = Self::current_minimized(apps);
        if !self.started {
            self.last_minimized = minimized;
            self.last_rects.clear();
            self.remember_rects(apps);
            self.started = true;
            self.director.mark_started();
            self.calm_open(apps);
            return;
        }
        self.remember_rects(apps);
        self.calm_open(apps);
        let foreground = foreground_window().map_or(0, |id| id.value() as isize);
        self.director.cancel_if_stale(foreground, &minimized);
        for &hwnd in &minimized {
            if self.last_minimized.contains(&hwnd) {
                continue;
            }
            self.begin_minimize(dock, apps, position, surface, hwnd, foreground);
        }
        self.last_minimized = minimized;
    }

    /// Foto fija inmediata: lo último visto más lo que diga Windows ahora.
    /// Devuelve lo que se veía para apuntar después. Vía de Muelle (MIT).
    pub(crate) fn begin_stand_in(
        &mut self,
        position: DockPosition,
        hwnd: isize,
        foreground: isize,
    ) -> Option<[i32; 4]> {
        if self.is_settings(hwnd as u64) {
            eprintln!("genio: sin vuelo para {hwnd}: es la ventana de Preferencias");
            return None;
        }
        let seen = self.last_rects.get(&hwnd).copied();
        let Some((from, frame)) = rect_before_minimize(hwnd, seen) else {
            eprintln!("genio: sin vuelo para {hwnd}: no se supo de donde volar");
            return None;
        };
        let screen = monitor_rect_for_window(hwnd);
        if !self
            .director
            .begin_minimize(hwnd, from, frame, screen, position, foreground)
        {
            return None;
        }
        // Lo último visto ya no vale: la ventana está minimizada.
        self.last_rects.remove(&hwnd);
        Some(frame)
    }

    /// Empieza el vuelo de una ventana recién minimizada (vía agrupada).
    fn begin_minimize(
        &mut self,
        dock: &Dock,
        apps: &[DockApp],
        position: DockPosition,
        surface: &Surface,
        hwnd: isize,
        foreground: isize,
    ) {
        if self.is_settings(hwnd as u64) {
            return;
        }
        let seen = self.last_rects.get(&hwnd).copied();
        let Some((from, frame)) = rect_before_minimize(hwnd, seen) else {
            eprintln!("genio: sin vuelo para {hwnd} (vía agrupada): no se supo de donde volar");
            return;
        };
        let screen = monitor_rect_for_window(hwnd);
        if !self
            .director
            .begin_minimize(hwnd, from, frame, screen, position, foreground)
        {
            return;
        }
        let aspect = (frame[2] - frame[0]) as f32 / (frame[3] - frame[1]).max(1) as f32;
        self.last_rects.remove(&hwnd);
        self.aim_with_aspect(dock, apps, surface, hwnd, aspect);
    }

    /// Apunta el vuelo en curso al hueco definitivo del dock.
    pub(crate) fn aim_with_aspect(
        &mut self,
        dock: &Dock,
        apps: &[DockApp],
        surface: &Surface,
        hwnd: isize,
        aspect: f32,
    ) {
        let id = WindowId::new(hwnd as u64);
        let Some(slot) = dock.genie_target(apps, id, aspect) else {
            eprintln!("genio: vuelo {hwnd} sin hueco: el dock aun no lo muestra");
            return;
        };
        let dock_rect = window_rect(WindowId::new(surface.handle().raw().0 as u64));
        let Some(dock_rect) = dock_rect else {
            eprintln!("genio: vuelo {hwnd} sin apuntar: no se supo donde esta el dock");
            return;
        };
        let slot_screen = dock_slot_to_screen(
            dock_rect,
            [slot.x, slot.y, slot.width, slot.height],
            surface.scale(),
        );
        self.director.aim_current(slot_screen);
        self.director.tuck_current_if_hidden(dock.is_hidden());
    }

    /// Marca una ventana como ya vista minimizada (la puso la vía inmediata).
    pub(crate) fn note_minimized(&mut self, hwnd: isize) {
        if !self.last_minimized.contains(&hwnd) {
            self.last_minimized.push(hwnd);
        }
    }

    /// Clic en la miniatura: vuela al revés desde su foto.
    pub(crate) fn on_restore_click(
        &mut self,
        dock: &Dock,
        apps: &[DockApp],
        position: DockPosition,
        surface: &Surface,
        window: WindowId,
    ) -> bool {
        let hwnd = window.value() as isize;
        let aspect = self.snapshot_aspect(hwnd).unwrap_or(1.0);
        let Some(slot) = dock.genie_target(apps, window, aspect) else {
            eprintln!("genio: sin vuelta para {hwnd}: no tiene hueco en el dock");
            return false;
        };
        let screen = monitor_rect_for_window(hwnd);
        let dock_rect = window_rect(WindowId::new(surface.handle().raw().0 as u64));
        let Some(dock_rect) = dock_rect else {
            eprintln!("genio: sin vuelta para {hwnd}: no se supo donde esta el dock");
            return false;
        };
        let slot_screen = dock_slot_to_screen(
            dock_rect,
            [slot.x, slot.y, slot.width, slot.height],
            surface.scale(),
        );
        let foreground = foreground_window().map_or(0, |id| id.value() as isize);
        let flying = self
            .director
            .begin_restore(hwnd, slot_screen, screen, position, foreground);
        if flying {
            self.director.tuck_current_if_hidden(dock.is_hidden());
        }
        flying
    }

    /// Avanza el vuelo; devuelve ventanas a restaurar ahora mismo.
    pub(crate) fn on_frame(&mut self, now: Instant) -> Vec<WindowId> {
        self.director
            .advance(now)
            .into_iter()
            .map(|hwnd| WindowId::new(hwnd as u64))
            .collect()
    }

    /// Al cerrar: devuelve transiciones y olvida vuelos.
    pub(crate) fn shutdown(&mut self) {
        self.director.restore_calmed();
    }

    /// Devuelve transiciones sin tocar vuelos (al apagar la opción).
    pub(crate) fn restore_transitions(&mut self) {
        self.director.restore_calmed();
    }
}
