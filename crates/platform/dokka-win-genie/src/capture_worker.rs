//! Hilo único de copia: conserva el dispositivo gráfico entre vuelos.
//!
//! Primero copia con la gráfica; si algo falla, usa la copia GDI de siempre.

use std::sync::mpsc::{Sender, channel};

use windows::Win32::Graphics::Dwm::DwmFlush;

use crate::GenieImage;
use crate::capture::capture_rect;
use crate::duplication::DesktopCopier;
use crate::geometry::{PixelRect, primary_screen_rect};

type Job = Box<dyn FnOnce(&mut Capturer) + Send>;

/// Puerta al hilo de copia. Al soltarla, el hilo termina solo.
pub(crate) struct CaptureWorker {
    jobs: Option<Sender<Job>>,
}

impl CaptureWorker {
    pub(crate) fn start() -> Self {
        let (jobs, queue) = channel::<Job>();
        let built = std::thread::Builder::new()
            .name("dokka-genie-capture".into())
            .spawn(move || {
                let mut capturer = Capturer { copier: None };
                capturer.warm_up();
                for job in queue {
                    job(&mut capturer);
                }
            });
        Self {
            jobs: built.is_ok().then_some(jobs),
        }
    }

    /// Encarga un trabajo; `false` si el hilo no está.
    pub(crate) fn run(&self, job: impl FnOnce(&mut Capturer) + Send + 'static) -> bool {
        self.jobs
            .as_ref()
            .is_some_and(|jobs| jobs.send(Box::new(job)).is_ok())
    }
}

/// Estado propio del hilo de copia.
pub(crate) struct Capturer {
    copier: Option<DesktopCopier>,
}

impl Capturer {
    /// Crea el dispositivo antes del primer vuelo para que no lo pague él.
    fn warm_up(&mut self) {
        let [left, top, right, bottom] = primary_screen_rect();
        self.copier = DesktopCopier::for_rect(PixelRect {
            left,
            top,
            right,
            bottom,
        })
        .ok();
    }

    /// Foto de `rect` y el método usado.
    pub(crate) fn capture(&mut self, rect: PixelRect) -> Option<(GenieImage, &'static str)> {
        // La foto fija recién puesta tiene que estar ya en pantalla.
        // SAFETY: solo espera a la siguiente composición.
        unsafe {
            let _ = DwmFlush();
        }
        if !self
            .copier
            .as_ref()
            .is_some_and(|copier| copier.holds(rect))
        {
            self.copier = DesktopCopier::for_rect(rect).ok();
        }
        if let Some(copier) = self.copier.as_ref().filter(|copier| copier.holds(rect)) {
            match copier.copy(rect) {
                Ok(image) => return Some((image, "gráfica")),
                Err(error) => {
                    eprintln!(
                        "genio: la copia con la gráfica falló ({:#X}); se usa GDI",
                        error.code().0
                    );
                    self.copier = None;
                }
            }
        }
        capture_rect(rect).map(|image| (image, "GDI"))
    }
}
