use dokka_animation::Bounce;
use dokka_model::{AppId, DockApp};

use crate::Dock;

/// Salto de una app que se está abriendo.
#[derive(Debug)]
pub(crate) struct LaunchBounce {
    bounce: Bounce,
    /// Ventanas que tenía al pulsar: una más significa que ya abrió.
    windows_before: usize,
}

impl LaunchBounce {
    pub(crate) fn lift(&self, height: f32) -> f32 {
        self.bounce.lift(height)
    }

    pub(crate) fn advance(&mut self, seconds: f32) -> bool {
        self.bounce.advance(seconds)
    }
}

impl Dock {
    pub fn start_bounce(&mut self, app_id: AppId, apps: &[DockApp]) {
        let launch = LaunchBounce {
            bounce: Bounce::new(),
            windows_before: window_count(apps, &app_id),
        };
        self.bounces.insert(app_id, launch);
    }

    /// Detiene el salto de las apps que ya mostraron su ventana.
    pub fn apps_changed(&mut self, apps: &[DockApp]) {
        for (app_id, launch) in &mut self.bounces {
            if window_count(apps, app_id) > launch.windows_before {
                launch.bounce.finish();
            }
        }
    }
}

fn window_count(apps: &[DockApp], app_id: &AppId) -> usize {
    apps.iter()
        .find(|app| &app.info.id == app_id)
        .map_or(0, |app| app.windows.len())
}
