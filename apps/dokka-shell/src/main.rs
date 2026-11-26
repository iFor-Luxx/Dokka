//! Punto de entrada de Dokka: crea las piezas y las conecta.

mod context_menu;
mod dock_session;
mod floating_panels;
mod handle_event;
mod launchpad;
mod media;
mod perform_click;
mod pinned_apps;
mod session_render;
mod taskbar_mode;

use std::error::Error;

use dokka_theme::typography;

use dokka_win_apps::explorer_app;
use dokka_win_composition::SceneRenderer;
use dokka_win_control::set_taskbar_visible;
use dokka_win_surface::{Surface, SurfaceOptions};
use dokka_win_tracking::{ChangeWatcher, WINDOWS_CHANGED_MESSAGE};

use crate::dock_session::DockSession;
use crate::handle_event::handle_event;
use crate::pinned_apps::load_pinned;
use crate::taskbar_mode::load_taskbar_mode;

fn main() -> Result<(), Box<dyn Error>> {
    // Recupera la barra si un cierre anterior la dejó oculta.
    set_taskbar_visible(true);
    // La ventana se crea primero: declara el modo de DPI de todo el proceso.
    let mut surface = Surface::create(SurfaceOptions {
        width: 0.0,
        height: 0.0,
        bottom_margin: 0.0,
        over_taskbar: false,
    })?;
    let renderer = SceneRenderer::attach(
        surface.handle(),
        surface.above_handle(),
        surface.launchpad_handle(),
        surface.scale(),
        typography::FONT_FAMILY,
        dokka_icons::LUCIDE_FONT_BYTES,
    )?;
    let explorer = explorer_app();
    let mut session = DockSession::new(renderer, explorer.id.clone());
    let _watcher = ChangeWatcher::start();

    load_pinned(&mut session, explorer);
    if load_taskbar_mode(&mut session) {
        // En automático la barra ya empieza oculta.
        set_taskbar_visible(false);
    }
    session.sync()?;
    session.update_clock_and_trash();
    session.resize(&mut surface)?;
    crate::session_render::render(&mut session, &mut surface)?;

    println!("Dokka en marcha. Para cerrarlo: Ctrl+C en esta terminal.");
    let result = surface.run(WINDOWS_CHANGED_MESSAGE, |surface, event| {
        if let Err(error) = handle_event(&mut session, surface, event) {
            eprintln!("Dokka no pudo atender un evento: {error}");
        }
        session.is_animating()
    });
    // La barra siempre vuelve, también si `run` terminó con error.
    set_taskbar_visible(true);
    result?;
    Ok(())
}
