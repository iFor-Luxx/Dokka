//! Punto de entrada de Dokka: crea las piezas y las conecta.

mod app_menu_actions;
mod context_menu;
mod dock_session;
mod floating_panels;
mod handle_event;
mod launchpad;
mod media;
mod menubar;
mod perform_click;
mod pinned_apps;
mod preview_actions;
mod preview_large;
mod session_render;
mod settings;
mod taskbar_mode;

use std::error::Error;

use dokka_theme::typography;

use dokka_win_apps::explorer_app;
use dokka_win_composition::{SceneRenderer, WindowHandles};
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
    surface.settings_place(dokka_theme::settings::WIDTH, dokka_theme::settings::HEIGHT)?;
    crate::menubar::setup_menubar(&mut surface)?;
    let renderer = SceneRenderer::attach(
        WindowHandles {
            main: surface.handle(),
            above: surface.above_handle(),
            launchpad: surface.launchpad_handle(),
            preview: surface.preview_handle(),
            settings: surface.settings_handle(),
            menubar: surface.menubar_handle(),
        },
        surface.scale(),
        surface.settings_scale(),
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
    crate::menubar::refresh_foreground(&mut session);
    crate::menubar::refresh_clock(&mut session);
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
    surface.menubar_hide();
    set_taskbar_visible(true);
    result?;
    Ok(())
}
