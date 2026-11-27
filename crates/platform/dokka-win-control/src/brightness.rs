//! Brillo del monitor principal por DDC/CI.
//!
//! Las llamadas DDC tardan decenas o cientos de milisegundos: todo ocurre en
//! un hilo propio, como `dokka-win-media/media_monitor.rs`. La interfaz solo
//! lee el último valor y envía órdenes sin esperar. Al arrastrar llegan muchas
//! órdenes seguidas: el hilo vacía la cola y escribe solo la última.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, channel};
use std::thread;

use dokka_contracts::BrightnessControl;
use windows::Win32::Devices::Display::{
    DestroyPhysicalMonitors, GetMonitorBrightness, GetNumberOfPhysicalMonitorsFromHMONITOR,
    GetPhysicalMonitorsFromHMONITOR, PHYSICAL_MONITOR, SetMonitorBrightness,
};
use windows::Win32::Foundation::{HANDLE, POINT};
use windows::Win32::Graphics::Gdi::{HMONITOR, MONITOR_DEFAULTTOPRIMARY, MonitorFromPoint};

/// Órdenes al hilo DDC: fijar el nivel o releerlo.
#[derive(Debug, Clone, Copy)]
enum BrightnessCommand {
    Set(f32),
    Refresh,
}

/// Último valor conocido, compartido con el hilo.
#[derive(Debug, Default)]
struct BrightnessShared {
    latest: Mutex<Option<f32>>,
}

/// Brillo DDC del monitor principal.
///
/// `start` lanza el hilo y pide la primera lectura. `brightness` devuelve lo
/// último sin esperar (`None` sin DDC). `set_brightness` envía la orden y
/// adelanta el valor local para que el deslizador no salte. `request_refresh`
/// relee bajo demanda (al abrir el centro de control).
#[derive(Debug)]
pub struct SystemBrightness {
    commands: Sender<BrightnessCommand>,
    shared: Arc<BrightnessShared>,
}

impl SystemBrightness {
    pub fn start() -> Self {
        let (commands, queue) = channel();
        let shared = Arc::new(BrightnessShared::default());
        let worker_shared = Arc::clone(&shared);
        // El hilo termina solo cuando se suelta este monitor.
        thread::spawn(move || watch(&queue, &worker_shared));
        let monitor = Self { commands, shared };
        monitor.request_refresh();
        monitor
    }

    /// Relee el brillo en el hilo (por si cambió con los botones del monitor).
    pub fn request_refresh(&self) {
        let _ = self.commands.send(BrightnessCommand::Refresh);
    }
}

impl Default for SystemBrightness {
    fn default() -> Self {
        Self::start()
    }
}

impl BrightnessControl for SystemBrightness {
    fn brightness(&self) -> Option<f32> {
        if let Ok(latest) = self.shared.latest.lock() {
            *latest
        } else {
            None
        }
    }

    fn set_brightness(&self, level: f32) {
        let clamped = level.clamp(0.0, 1.0);
        // Adelanta el valor local para no esperar al DDC.
        if let Ok(mut latest) = self.shared.latest.lock() {
            *latest = Some(clamped);
        }
        let _ = self.commands.send(BrightnessCommand::Set(clamped));
    }
}

/// Bucle DDC: una orden tras otra, quedándose con la última si se acumulan.
fn watch(queue: &std::sync::mpsc::Receiver<BrightnessCommand>, shared: &Arc<BrightnessShared>) {
    let mut minimum = 0_u32;
    let mut maximum = 100_u32;
    let mut has_range = false;
    loop {
        let first = match queue.recv() {
            Ok(command) => command,
            Err(_) => return,
        };
        // Vacía la cola: solo importa la última orden de nivel,
        // más si hubo algún refresco entre medias.
        let mut pending_set: Option<f32> = None;
        let mut pending_refresh = false;
        match first {
            BrightnessCommand::Set(level) => pending_set = Some(level),
            BrightnessCommand::Refresh => pending_refresh = true,
        }
        for command in queue.try_iter() {
            match command {
                BrightnessCommand::Set(level) => pending_set = Some(level),
                BrightnessCommand::Refresh => pending_refresh = true,
            }
        }
        if let Some(level) = pending_set {
            if !has_range && let Some((low, _, high)) = read_brightness() {
                minimum = low;
                maximum = high;
                has_range = true;
            }
            if has_range {
                write_brightness(level, minimum, maximum);
            } else {
                // Sin rango conocido (y sin lectura posible), intenta
                // escribir en 0-100 y corrige el optimismo local.
                write_brightness(level, 0, 100);
                if read_brightness().is_none()
                    && let Ok(mut latest) = shared.latest.lock()
                {
                    *latest = None;
                }
            }
            // Tras escribir, el valor local ya adelanta lo pedido.
        }
        if pending_refresh && let Some((low, current, high)) = read_brightness() {
            minimum = low;
            maximum = high;
            has_range = true;
            let span = high.saturating_sub(low).max(1) as f32;
            let level = (current.saturating_sub(low) as f32 / span).clamp(0.0, 1.0);
            if let Ok(mut latest) = shared.latest.lock() {
                *latest = Some(level);
            }
        }
    }
}

/// Monitor principal: punto (0, 0) con vuelta al primario.
fn primary_monitor() -> HMONITOR {
    // SAFETY: pide el monitor de un punto válido con vuelta al primario;
    // nunca toca memoria, solo devuelve un identificador.
    unsafe { MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY) }
}

/// Mínimo, actual y máximo DDC del primer monitor físico, si responde.
fn read_brightness() -> Option<(u32, u32, u32)> {
    let monitor = primary_monitor();
    if monitor.is_invalid() {
        return None;
    }
    let mut count = 0_u32;
    // SAFETY: cuenta los monitores físicos del principal con un puntero válido.
    unsafe {
        GetNumberOfPhysicalMonitorsFromHMONITOR(monitor, &mut count).ok()?;
    }
    if count == 0 {
        return None;
    }
    let mut physical = vec![PHYSICAL_MONITOR::default(); count as usize];
    // SAFETY: rellena el vector ya reservado y lo destruye al salir.
    unsafe {
        GetPhysicalMonitorsFromHMONITOR(monitor, &mut physical).ok()?;
    }
    let handle = physical.first()?.hPhysicalMonitor;
    let mut low = 0_u32;
    let mut current = 0_u32;
    let mut high = 0_u32;
    // SAFETY: lee el brillo con tres salidas válidas del monitor abierto.
    let ok = unsafe { GetMonitorBrightness(handle, &mut low, &mut current, &mut high) != 0 };
    // SAFETY: libera los monitores abiertos en este mismo bloque.
    unsafe {
        let _ = DestroyPhysicalMonitors(&physical);
    }
    if !ok || high <= low {
        return None;
    }
    Some((low, current.min(high), high))
}

/// Escribe el nivel 0-1 como valor entre mínimo y máximo.
fn write_brightness(level: f32, minimum: u32, maximum: u32) {
    let span = maximum.saturating_sub(minimum) as f32;
    if span <= 0.0 {
        return;
    }
    let value = minimum.saturating_add((level.clamp(0.0, 1.0) * span).round() as u32);
    let monitor = primary_monitor();
    if monitor.is_invalid() {
        return;
    }
    let mut count = 0_u32;
    // SAFETY: cuenta los monitores físicos del principal con un puntero válido.
    unsafe {
        if GetNumberOfPhysicalMonitorsFromHMONITOR(monitor, &mut count).is_err() {
            return;
        }
    }
    if count == 0 {
        return;
    }
    let mut physical = vec![PHYSICAL_MONITOR::default(); count as usize];
    // SAFETY: rellena el vector ya reservado y lo destruye al salir.
    unsafe {
        if GetPhysicalMonitorsFromHMONITOR(monitor, &mut physical).is_err() {
            return;
        }
    }
    let handle: HANDLE = physical
        .first()
        .map(|entry| entry.hPhysicalMonitor)
        .unwrap_or_default();
    if handle.is_invalid() {
        // SAFETY: libera los monitores abiertos en este mismo bloque.
        unsafe {
            let _ = DestroyPhysicalMonitors(&physical);
        }
        return;
    }
    // SAFETY: escribe un valor dentro del rango leído en el monitor abierto.
    unsafe {
        SetMonitorBrightness(handle, value);
        let _ = DestroyPhysicalMonitors(&physical);
    }
}
