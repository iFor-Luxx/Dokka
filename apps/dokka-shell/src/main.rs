//! Punto de entrada de Dokka: crea las piezas y las conecta.

mod dock_session;

use std::error::Error;

use dokka_win_composition::SceneRenderer;
use dokka_win_surface::{Surface, SurfaceOptions};
use dokka_win_tracking::{ChangeWatcher, WINDOWS_CHANGED_MESSAGE};

use crate::dock_session::{DockSession, surface_options};

fn main() -> Result<(), Box<dyn Error>> {
    // La ventana se crea primero: declara el modo de DPI de todo el proceso.
    let mut surface = Surface::create(SurfaceOptions {
        width: 0.0,
        height: 0.0,
        bottom_margin: 0.0,
    })?;
    let mut renderer = SceneRenderer::attach(surface.handle(), surface.scale())?;
    let mut session = DockSession::default();
    let _watcher = ChangeWatcher::start();

    session.sync()?;
    let scene = session.scene();
    surface.resize(surface_options(&scene))?;
    renderer.draw(&scene)?;

    println!("Dokka en marcha. Para cerrarlo: Ctrl+C en esta terminal.");
    surface.run(WINDOWS_CHANGED_MESSAGE, |surface| {
        if let Err(error) = refresh(surface, &mut renderer, &mut session) {
            eprintln!("No se pudo actualizar el dock: {error}");
        }
    })?;
    Ok(())
}

fn refresh(
    surface: &mut Surface,
    renderer: &mut SceneRenderer,
    session: &mut DockSession,
) -> Result<(), Box<dyn Error>> {
    if session.sync()? {
        let scene = session.scene();
        surface.resize(surface_options(&scene))?;
        renderer.draw(&scene)?;
    }
    Ok(())
}
